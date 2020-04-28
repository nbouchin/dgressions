use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Child};
use std::path::Path;

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

        file.read_to_string(&mut contents).expect("Could not parse config");
        match toml::from_str::<Config>(&contents) {
            Ok(n) => Ok(n),
            Err(n) => Err(n),
        }
    }

    fn build_env(config: Config) -> HashMap<String, String> {
        let mut env: HashMap<String, String> = env::vars().collect();

        for val in config.env.unwrap_or_default().iter() {
            let (first, last) = val.split_at(val.trim().find('=').unwrap_or(val.len()));
            env.insert(first.to_string(), last.get(1..).unwrap_or_default().to_string());
        }
        env
    }

    pub fn run(&self) {
        let cmd: &String = &self.config.cmd;
        let args: Vec<String> = self.config.args.unwrap_or_default();
        let env: HashMap<String, String> = Unit::build_env(self.config);

        let child = Command::new(cmd)
            .args(args)
            .envs(&env)
            .spawn()
            .expect("Command failed to execute");

        &self.child = Some(child);
    }
}


fn main() -> std::io::Result<()> {
    let path = Path::new("./nginx.service");
    let unit: Unit = Unit::new(path);
    unit.run();
    Ok(())
}
