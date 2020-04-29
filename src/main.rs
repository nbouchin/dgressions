use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::path::Path;
use std::process::{Child, Command};
use std::{thread, time};

#[derive(Deserialize, Debug)]
pub struct Config {
    cmd: String,
    description: Option<String>,
    args: Option<Vec<String>>,
    numprocs: Option<u8>,
    workingdir: Option<String>,
    autostart: Option<bool>,
    autorestart: Option<String>,
    exitcodes: Option<Vec<u8>>,
    startretries: Option<u8>,
    starttime: Option<u8>,
    stopsignal: Option<String>,
    stoptime: Option<u8>,
    stdout: Option<String>,
    stderr: Option<String>,
    env: Option<Vec<String>>,
}

struct Unit {
    //started_at: SystemTime,
    //status: UnitStatus,
    child: Option<Child>,
    config: Config,
    path: &'static Path,
}

impl Unit {
    pub fn new(path: &'static Path) -> Unit {
        Unit {
            config: Unit::get_config(path).unwrap(),
            path,
            child: None,
        }
    }

    fn get_config(path: &Path) -> Result<Config, toml::de::Error> {
        let mut file = File::open(path.to_str().unwrap_or_default()).expect("Unable to open");
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("Could not parse config");
        match toml::from_str::<Config>(&contents) {
            Ok(n) => Ok(n),
            Err(n) => Err(n),
        }
    }

    fn build_env(config: &Config) -> HashMap<String, String> {
        let mut env = env::vars().collect::<HashMap<String, String>>();

        let conf_env = config.env.as_ref();

        for val in conf_env.unwrap() {
            let (first, last) = val.split_at(val.trim().find('=').unwrap_or(val.len()));
            env.insert(
                first.to_string(),
                last.get(1..).unwrap_or_default().to_string(),
            );
        }
        env
    }

    pub fn start(&mut self) {
        // Change working directory if set --
        // Set environment variables --
        // Redirect stdout/stderr if set
        // Loop until unit is started or number of restarts reached
        // Start the unit with given command line
        // Wait until starttime is reached
        // Check if unit is still alive

        // Unit command
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
            },
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
            },
            _ => (),
        };

        // Set Environment Variables
        let env = Unit::build_env(&self.config);
        command.envs(&env);

        // Set arguments;
        let args = match &self.config.args {
            Some(arguments) => arguments.to_owned(),
            None => vec![],
        };
        command.args(&args);

        let child = command.spawn()
            .expect("Command failed to execute");
        self.child = Some(child);
    }

    pub fn stop(&mut self) {
        self.child.as_mut().unwrap().kill().expect("Command was not running");
    }
}

fn main() -> std::io::Result<()> {
    let path = Path::new("./nginx.service");
    let mut unit: Unit = Unit::new(path);
    unit.start();
    unit.stop();
    Ok(())
}
