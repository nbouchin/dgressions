use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
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
        let env = Unit::build_env(&self.config);
        let args = match &self.config.args {
            Some(arguments) => arguments.to_owned(),
            None => vec![],
        };

        let child = Command::new(&self.config.cmd)
            .args(&args)
            .envs(&env)
            .spawn()
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
