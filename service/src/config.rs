use std::{fs::File, io::BufReader, path::Path};

use serde_derive;
use serde_yaml;

pub fn read_config<P: AsRef<Path>, const NumAppVolume: usize>(
    path: P,
) -> Result<Config<NumAppVolume>, Error> {
    let config_file = File::open(path)?;
    let reader = BufReader::new(config_file);
    Ok(serde_yaml::from_reader(reader)?)
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Deserialize error: {0}")]
    Deserialize(#[from] serde_yaml::Error),
}

/// The top-level configuration struct.
#[derive(Debug, serde_derive::Deserialize)]
pub struct Config<const NumAppVolume: usize> {
    #[serde(with = "serde_arrays")]
    pub application_volume_dials: [ApplicationVolumeDial; NumAppVolume],
}

/// Configuration for a single application audio control dial.
#[derive(Debug, serde_derive::Deserialize)]
pub struct ApplicationVolumeDial {
    pub name: String,
    pub selector: ApplicationVolumeDialApplicationSelector,
}

/// How to select applications affected by a dial.
#[derive(Debug, serde_derive::Deserialize)]
pub enum ApplicationVolumeDialApplicationSelector {
    /// A regex to search for inside the application's path converted to lowercase.
    /// This regex is not anchored by default.
    Regex(String),
    /// Select any applications not matched by an existing selector. There can be only one
    /// `Unmatched` dial configured.
    Unmatched,
}
