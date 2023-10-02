#![feature(never_type)]

use binary_serde::BinarySerde;
use serialport;
use thiserror;

mod config;
use config::read_config;
mod application_audio;
use application_audio::*;

use lib::computer_state;

const NUM_APPLICATION_VOLUME_DIALS: usize = 3;

type Config = config::Config<NUM_APPLICATION_VOLUME_DIALS>;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[cfg(windows)]
    #[error("OS error: {0}")]
    OS(#[from] windows::core::Error),
    #[error("Serial error: {0}")]
    Serial(#[from] serialport::Error),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}
impl Error {
    fn is_recoverable(&self) -> bool {
        match self {
            Error::Serial(..) | Error::IO(..) => false,
            _ => true,
        }
    }
}
type AppResult<T> = Result<T, Error>;

fn main() {
    match read_config("config.yaml") {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(config) => {
            if let Err(e) = main_loop(&config) {
                println!("{}", e);
            }
        }
    }
}

fn main_loop(config: &Config) -> Result<!, Error> {
    let mut serial_port =
        serialport::new(&config.serial_port_path, config.serial_port_baud).open()?;

    loop {
        let result = sync_state(config, serial_port.as_mut());
        match result {
            Err(e) if e.is_recoverable() => {
                return Err(e);
            }
            Err(e) => {
                // TODO: log more visibly
                println!("{}", e);
            }
            Ok(_) => {}
        }
    }
}

fn sync_state(config: &Config, serial_port: &mut dyn serialport::SerialPort) -> AppResult<()> {
    let mut audio_interface = AudioInterface::create()?;
    send_state(config, serial_port, &audio_interface)?;
    receive_state(config, serial_port, &mut audio_interface)
}

fn send_state(
    config: &Config,
    mut serial_port: &mut dyn serialport::SerialPort,
    audio_interface: &AudioInterface,
) -> AppResult<()> {
    // For each dial, work out what the current state of the related applications are.
    let mut volume_dials = std::collections::HashMap::<
        &config::ApplicationVolumeDial,
        Vec<&application_audio::ApplicationAudio>,
    >::new();

    for application in audio_interface.application_audios.iter() {
        for dial in config.application_volume_dials.iter() {
            if dial.regex.is_match(&application.get_process_path()?) {
                volume_dials.entry(dial).or_default().push(application);
                continue;
            }
        }
    }

    // Assume things are muted until proven otherwise.
    let initial_volume_dial_state = computer_state::ApplicationVolumeDialState { mute: true };
    let mut volume_dial_states = [initial_volume_dial_state; NUM_APPLICATION_VOLUME_DIALS];
    // Important to iterate in order of config dials, so that the output is in the same order
    for (dial, dial_state) in config
        .application_volume_dials
        .iter()
        .zip(volume_dial_states.iter_mut())
    {
        let applications = volume_dials.remove(dial).unwrap_or_default();
        for application in applications {
            dial_state.mute &= application.get_mute()?;
        }
    }

    let computer_state = computer_state::State::<NUM_APPLICATION_VOLUME_DIALS> {
        application_volume_dial_states: volume_dial_states,
    };

    computer_state.binary_serialize_into(&mut serial_port, binary_serde::Endianness::Big)?;
    Ok(())
}

fn receive_state(
    config: &Config,
    serial_port: &mut dyn serialport::SerialPort,
    audio_interface: &mut AudioInterface,
) -> AppResult<()> {
    unimplemented!()
}
