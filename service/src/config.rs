use std::{fs::File, io::BufReader, path::Path};

use serde_derive;
use serde_yaml;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Deserialize error: {0}")]
    Deserialize(#[from] serde_yaml::Error),
}

pub fn read_config<P: AsRef<Path>, const NUM_APP_VOLUME: usize>(
    path: P,
) -> Result<Config<NUM_APP_VOLUME>, Error> {
    let config_file = File::open(path)?;
    let reader = BufReader::new(config_file);
    Ok(serde_yaml::from_reader(reader)?)
}

/// The top-level configuration struct.
#[derive(Debug, serde_derive::Deserialize)]
pub struct Config<const NUM_APP_VOLUME: usize> {
    /// A description of the audio dials, and which programs they match.
    /// Applications will be matched against these _in order_.
    #[serde(with = "serde_arrays")]
    pub application_volume_dials: [ApplicationVolumeDial; NUM_APP_VOLUME],

    #[serde(default = "default_serial_path")]
    pub serial_port_path: String,
    #[serde(default = "default_serial_baud")]
    pub serial_port_baud: u32,
}

/// Configuration for a single application audio control dial.
#[derive(Debug, derivative::Derivative, serde_derive::Deserialize)]
#[derivative(Hash, PartialEq, Eq)]
pub struct ApplicationVolumeDial {
    /// The informal name assigned to the dial. This must be unique among dials.
    pub name: String,
    /// Regex to match any application paths which should be assigned to this dial.
    /// Will be used unanchored against the lowercase program path. May be `.*` to match all applications.
    #[serde(with = "serde_regex")]
    #[derivative(Hash = "ignore", PartialEq = "ignore")]
    pub regex: regex::Regex,
}

#[cfg(windows)]
fn default_serial_path() -> String {
    "COM0".to_string()
}
#[cfg(linux)]
fn default_serial_path() -> String {
    "/dev/ttyUSB0".to_string()
}

fn default_serial_baud() -> u32 {
    56_000
}
