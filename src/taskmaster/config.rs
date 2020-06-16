use serde_derive::Deserialize;
use std::path::Path;
use std::fs::File;
use std::io::prelude::Read;

/// Configuration of a unit created from a TOML file
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub cmd: String,
    pub description: Option<String>,
    pub args: Option<Vec<String>>,
    pub numprocs: Option<u8>,
    pub workingdir: Option<String>,
    pub autostart: Option<bool>,
    pub autorestart: Option<String>,
    pub exitcodes: Option<Vec<u8>>,
    pub startretries: Option<u8>,
    pub starttime: Option<u8>,
    pub stopsignal: Option<String>,
    pub stoptime: Option<u8>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub env: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    TOML(toml::de::Error),
}

type ConfigResult<T> = Result<T, ConfigError>;

impl Config {
    /// Create a Config from a TOML file
    pub fn from_file(path: &Path) -> ConfigResult<Config> {
        let mut file = File::open(path.to_str().unwrap_or_default()).expect("Unable to open");
        let mut contents = String::new();

        if let Err(e) = file.read_to_string(&mut contents) {
            return Err(ConfigError::Io(e));
        }

        match toml::from_str::<Config>(&contents) {
            Ok(n) => Ok(n),
            Err(n) => Err(ConfigError::TOML(n)),
        }
    }
}
