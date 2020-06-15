//! Controls every unit
#![allow(unused)]

use crate::unit;
use std::collections::HashMap;
use std::path::Path;
use unit::UnitError;

pub struct UnitInfo {
    pub unit: unit::Unit,
    restarts: u32,
    is_running: bool,
}

impl UnitInfo {
    pub fn new(unit: unit::Unit) -> UnitInfo {
        UnitInfo {
            unit,
            /// Current number of restarts
            restarts: 0,
            is_running: false,
        }
    }
}

pub type UnitPool = HashMap<String, UnitInfo>;

/// Controls every unit
pub struct Master {}

impl Master {
    /// Read every unit files inside a folder and start and manage them
    /// when needed.
    pub fn load_units_in_folder(path: &Path) -> UnitPool {
        let mut units = UnitPool::new();

        // Read every unit file present inside the directory at `path'
        if let Ok(dir) = std::fs::read_dir(path) {
            for entry in dir {
                match entry {
                    Ok(file) => {
                        if file.file_type().unwrap().is_file() {
                            let osname = file.file_name();
                            let filename = osname.to_str().unwrap();
                            let path = file.path();

                            // Isolate the filename in the path. This will be the unit's internal name
                            let filename = match &path.file_name() {
                                Some(os_name) => match os_name.to_str() {
                                    Some(name) => Some(String::from(name)),
                                    None => None,
                                },
                                None => None,
                            };

                            // Add the unit to the collection of units
                            if let Some(filename) = filename {
                                if filename.ends_with(".service") {
                                    info!("Loading unit {}", &path.to_str().unwrap());
                                    match unit::Unit::new(&path) {
                                        Ok(unit) => {
                                            units.insert(filename, UnitInfo::new(unit));
                                        }
                                        Err(e) => {
                                            warn!(
                                                "Could not load unit {}: {:?}",
                                                path.to_str().unwrap(),
                                                e
                                            );
                                        }
                                    };
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                };
            }
        }

        units
    }

    /// Cycle through units and update the state of all the units
    pub fn update_units(units: &mut UnitPool) {
        for mut unit in units.iter_mut() {
            let mut has_exited = false;

            debug!("Updating unit {}", &unit.0);
            match unit.1.unit.child.as_mut().unwrap().try_wait() {
                Ok(exit) => {
                    match exit {
                        Some(exit_status) => {
                            debug!(
                                "Process of unit {} has exited with code {}",
                                &unit.0, exit_status
                            );
                            has_exited = true;
                        }
                        None => {}
                    };
                }
                Err(e) => {
                    error!("{:?}", e);
                }
            }

            // Restart the unit if it has unexpectedly stopped
            if has_exited && unit.1.is_running {
                Master::restart_unit(&mut unit.1);
            }
        }
    }

    pub fn start_all_units(units: &mut UnitPool) {
        for (name, info) in units.iter_mut() {
            match info.unit.start() {
                Ok(_) => {
                    debug!("{}: started", name);
                    info.is_running = true;
                }
                Err(e) => {
                    debug!("{:?}", e);
                }
            }
        }
    }

    /// Start a unit
    pub fn start_unit(unit: &mut unit::Unit) -> Result<(), UnitError> {
        unit.start()
    }

    pub fn start_unit_by_name(&self, units: &mut UnitPool, name: &str) -> Result<(), UnitError> {
        if !units.contains_key(name) {
            return Err(UnitError::DoesNotExist);
        }

        info!("Starting unit {}", name);

        match units.get_mut(name) {
            Some(unit_info) => match Master::start_unit(&mut unit_info.unit) {
                Ok(_) => Ok(()),
                err => err,
            },
            None => Err(UnitError::DoesNotExist),
        }
    }

    /// Stop all running units
    pub fn stop_all_units(&self) {
        unimplemented!();
    }

    /// Stop a running unit
    pub fn stop_unit(&self, name: &str) {
        unimplemented!();
    }

    pub fn restart_unit(unit: &mut UnitInfo) {
        match unit.unit.child.as_mut().unwrap().try_wait() {
            Ok(exit) => {
                if let Some(exit_code) = exit {
                    // Process is not running
                }
            }
            Err(e) => {
                error!("{:?}", e);
            }
        };
    }

    pub fn units_alive(units: &UnitPool) -> bool {
        for u in units.iter() {
            if u.1.unit.is_alive() {
                return true;
            }
        }

        false
    }
}
