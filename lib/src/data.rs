use binary_serde::BinarySerde;
use fixed::FixedU16;

/// A fixed-point value in the range [0, 1/2^16)
pub type Percentage = FixedU16<16>;

pub struct Snapshot<const N: usize> {
    pub percentages: [Percentage; N],
}
impl<const N: usize> Snapshot<N> {
    pub fn new(percentages: [Percentage; N]) -> Self {
        Snapshot { percentages }
    }
}

#[derive(BinarySerde)]
pub struct WireFormat<const N: usize> {
    percentages: [u16; N],
}
impl<const N: usize> From<Snapshot<N>> for WireFormat<N> {
    fn from(snapshot: Snapshot<N>) -> Self {
        let mut wire = Self {
            percentages: [0u16; N],
        };
        for (wire_percentage, percentage) in wire.percentages.iter_mut().zip(snapshot.percentages) {
            *wire_percentage = percentage.to_be().to_bits();
        }
        wire
    }
}
impl<const N: usize> From<WireFormat<N>> for Snapshot<N> {
    fn from(wire: WireFormat<N>) -> Self {
        let mut snapshot = Self {
            percentages: [Percentage::ZERO; N],
        };
        for (percentage, wire_percentage) in snapshot.percentages.iter_mut().zip(wire.percentages) {
            *percentage = Percentage::from_be(Percentage::from_bits(wire_percentage));
        }
        snapshot
    }
}
