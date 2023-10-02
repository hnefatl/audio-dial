use binary_serde::BinarySerde;

#[derive(Debug, BinarySerde)]
pub struct State<const N: usize> {
    pub application_volume_dial_states: [ApplicationVolumeDialState; N],
}

#[derive(Debug, BinarySerde, Clone, Copy)]
pub struct ApplicationVolumeDialState {
    pub mute: bool,
}