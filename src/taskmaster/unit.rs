//! A unit holds needed to run a single command and keep track of it

#![allow(unused)]

use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::path::Path;
use std::process::{Child, Command};
use super::config::{Config, ConfigError};

#[derive(Debug)]
pub enum UnitError {
    AlreadyRunning,
    AlreadyStopped,
    DoesNotExist,
    ConfigError(ConfigError),
}

pub type UnitResult<T> = Result<T, UnitError>;

/// Information about the unit
pub struct Unit {
    /// The time since the process is running
    pub started_at: std::time::SystemTime,
    /// Contains the running child process
    pub child: Option<Child>,
    /// Configuration populated from TOML configuration file
    config: Config,
    /// Unit's configuration file's path
    path: Box<Path>,
}

impl Unit {
    /// Create a unit from a configuration file
    pub fn new(path: &Path) -> Result<Unit, UnitError> {
        match Config::from_file(path) {
            Ok(config) => Ok(Unit {
                started_at: std::time::SystemTime::UNIX_EPOCH,
                config,
                path: Box::from(path),
                child: None,
            }),
            Err(e) => Err(UnitError::ConfigError(e)),
        }
    }

    /// Create environment for the unit.
    /// Combines the current environment with configured custom env variables
    fn build_env(config: &Config) -> HashMap<String, String> {
        let mut env = env::vars().collect::<HashMap<String, String>>();

        for val in config.env.as_ref().unwrap() {
            let (first, last) = val.split_at(val.trim().find('=').unwrap_or(val.len()));
            env.insert(
                first.to_string(),
                last.get(1..).unwrap_or_default().to_string(),
            );
        }
        env
    }

    /// Configure the unit
    pub fn configure(&mut self) {
        unimplemented!();
    }

    /// Start the unit
    pub fn start(&mut self) -> UnitResult<()> {
        // Change working directory if set --
        // Set environment variables --
        // Redirect stdout/stderr if set --
        // Loop until unit is started or number of restarts reached
        // Start the unit with given command line --
        // Wait until starttime is reached
        // Check if unit is still alive

        if self.child.is_some() {
            return Err(UnitError::AlreadyRunning);
        }

        // TODO: Set UMASK

        // Command to run
        let mut command = Command::new(&self.config.cmd);

        // Change Working Directory
        if let Some(dir) = &self.config.workingdir {
            command.current_dir(&dir);
        }

        // Redirect stdout
        match &self.config.stdout {
            Some(stdout) => {
                let raw = std::fs::OpenOptions::new()
                    .read(false)
                    .write(true)
                    .create(true)
                    .truncate(false)
                    .open(stdout)
                    .unwrap()
                    .into_raw_fd();

                let io = unsafe { std::process::Stdio::from_raw_fd(raw) };
                command.stdout(io);
            }
            _ => (),
        };

        // Redirect stderr
        match &self.config.stderr {
            Some(stderr) => {
                let raw = std::fs::OpenOptions::new()
                    .read(false)
                    .write(true)
                    .create(true)
                    .truncate(false)
                    .open(stderr)
                    .unwrap()
                    .into_raw_fd();
                let io = unsafe { std::process::Stdio::from_raw_fd(raw) };
                command.stderr(io);
            }
            _ => (),
        };

        // Set Environment Variables
        let env = Unit::build_env(&self.config);
        command.envs(&env);

        // Set arguments
        let args = match &self.config.args {
            Some(arguments) => arguments.to_owned(),
            None => vec![],
        };
        command.args(&args);

        // Update start time
        self.started_at = std::time::SystemTime::now();

        // Run the unit
        let child = command.spawn().expect("Command failed to execute");

        self.child = Some(child);

        Ok(())
    }

    /// Stop the unit
    pub fn stop(&mut self) {
        match &mut self.child {
            Some(c) => {
                if let Err(_) = c.kill() {
                    println!("Command was not running.");
                }
            }
            None => (),
        };
    }

    /// Restart the unit
    pub fn restart(&mut self) {
        unimplemented!();
    }

    pub fn is_alive(&self) -> bool {
        true
    }
}
