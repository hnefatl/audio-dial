mod config;
use config::read_config;
mod application_audio;
use application_audio::*;

use lib::computer_state;

const NUM_APPLICATION_VOLUME_DIALS: usize = 3;

type Config = config::Config<NUM_APPLICATION_VOLUME_DIALS>;

fn main() {
    match read_config("config.yaml") {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(config) => main_loop(&config),
    }
}

fn main_loop(config: &Config) -> ! {
    loop {
        if let Err(e) = sync_state(config) {
            println!("{}", e);
        }
    }
}

fn sync_state(config: &Config) -> WindowsResult<()> {
    let mut audio_state = AudioState::create()?;
    send_state(config, &audio_state)?;
    receive_state(config, &mut audio_state)
}

fn send_state(config: &Config, audio_state: &AudioState) -> WindowsResult<()> {
    let computer_state = computer_state::State::<NUM_APPLICATION_VOLUME_DIALS> {
        application_volume_dial_states: unimplemented!(),
    };
    Ok(())
}
fn receive_state(config: &Config, audio_state: &mut AudioState) -> WindowsResult<()> {
    unimplemented!()
}
