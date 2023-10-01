pub struct State<const N: usize> {
    pub application_volume_dial_states: [ApplicationVolumeDialState; N],
}

pub struct ApplicationVolumeDialState {
    pub mute: bool,
}