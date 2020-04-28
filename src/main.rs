use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct Config {
    cmd: String,
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

fn run_config(conf: Config) {
    let cmd: String = conf.cmd;
    let args: Vec<String> = conf.args.unwrap_or_default();
    let mut env: HashMap<String, String> = env::vars().collect();

    for val in conf.env.unwrap_or_default().iter() {
        let (first, last) = val.split_at(val.trim().find('=').unwrap_or(val.len()));
        env.insert(first.to_string(), last.get(1..).unwrap_or_default().to_string());
    }
    Command::new(cmd)
        .args(&args)
        .envs(&env)
        .spawn()
        .expect("Command failed to execute");
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("nginx.service")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    match toml::from_str::<Config>(&contents) {
        Ok(n) => run_config(n),
        Err(n) => println!("{}", n),
    }
    Ok(())
}
