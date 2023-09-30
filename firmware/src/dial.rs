use arduino_hal::Adc;
use core::num::{NonZeroU16, NonZeroU32};
use fixed::{FixedU16, FixedU32};
use lib::data::Snapshot;

pub type DialValue = u16;
/// A fixed-point value in the range [0, 2^16)
pub type AdcResolution = NonZeroU16;
/// A fixed-point value in the range [0, 1/2^16)
pub type Percentage = FixedU16<16>;

pub struct Dial {
    adc_channel: arduino_hal::adc::Channel,
}
impl Dial {
    const RESOLUTION: AdcResolution = AdcResolution::new(2 << 10).unwrap();

    pub fn new(adc_channel: arduino_hal::adc::Channel) -> Self {
        Dial { adc_channel }
    }

    pub fn read(&self, adc: &mut Adc) -> DialValue {
        adc.read_blocking(&self.adc_channel)
    }

    pub fn read_percentage(&self, adc: &mut Adc) -> Percentage {
        let value = self.read(adc);
        // Put the 16-bit value into a 32-bit fixed-point value with 16 bits of integer precision (fits exactly).
        // Then divide that by the resolution (16-bits cast to 32-bits), which requires at most 16 bits of
        // fractional precision. Since the resolution is always larger than the value (asserted below), this should be lossless.
        let ratio = FixedU32::<16>::from_num(value)
            / core::convert::Into::<NonZeroU32>::into(Self::RESOLUTION);

        assert!(ratio.int() == 0, "ADC value outside resolution");
        ratio.frac().to_num()
    }
}

pub struct Dials<const N: usize> {
    dials: [Dial; N],
}
impl<const N: usize> Dials<N> {
    pub fn new(dials: [Dial; N]) -> Self {
        Dials { dials }
    }

    pub fn snapshot<E>(&self, adc: &mut Adc) -> Result<Snapshot<N>, E> {
        let mut percentages = [Percentage::ZERO; N];
        for (i, dial) in self.dials.iter().enumerate() {
            percentages[i] = dial.read_percentage(adc);
        }
        Ok(Snapshot::new(percentages))
    }
}
