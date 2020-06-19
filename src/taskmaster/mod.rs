pub mod unit;
pub mod config;

use std::collections::HashMap;
use std::path::Path;
use unit::UnitError;
use std::cell::RefCell;

pub struct UnitInfo {
    pub unit: RefCell<unit::Unit>,
    restarts: u32,
    is_running: bool,
}

impl UnitInfo {
    pub fn new(unit: unit::Unit) -> UnitInfo {
        UnitInfo {
            unit: RefCell::new(unit),
            /// Current number of restarts
            restarts: 0,
            is_running: false,
        }
    }
}

pub type UnitPool = HashMap<String, UnitInfo>;

pub struct Taskmaster {
    units: UnitPool
}

impl Taskmaster {

    pub fn new() -> Taskmaster {
        Taskmaster {
            units: UnitPool::new()
        }
    }

    /// Read every unit files inside a folder and start and manage them
    /// when needed.
    pub fn load_units_in_folder(&mut self, path: &Path) -> usize {
        let mut number_of_units = 0;

        // Read every unit file present inside the directory at `path'
        if let Ok(dir) = std::fs::read_dir(path) {
            for entry in dir {
                match entry {
                    Ok(file) => {
                        if file.file_type().unwrap().is_file() {
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
                                            self.units.insert(filename, UnitInfo::new(unit));
                                            number_of_units += 1;
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

        number_of_units
    }

    /// Cycle through units and update the state of all the units
    pub fn update_units(&mut self) {
        for mut unit in self.units.iter_mut() {
            let mut has_exited = false;

            debug!("Updating unit {}", &unit.0);
            match unit.1.unit.borrow_mut().child.as_mut().unwrap().try_wait() {
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
                Taskmaster::restart_unit(&mut unit.1);
            }
        }
    }

    pub fn start_all_units(&mut self) {
        for (name, info) in self.units.iter_mut() {
            match info.unit.borrow_mut().start() {
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

    pub fn start_unit_by_name(units: &mut UnitPool, name: &str) -> Result<(), UnitError> {
        if !units.contains_key(name) {
            return Err(UnitError::DoesNotExist);
        }

        info!("Starting unit {}", name);

        match units.get_mut(name) {
            Some(unit_info) => match Taskmaster::start_unit(&mut *unit_info.unit.borrow_mut()) {
                Ok(_) => Ok(()),
                err => err,
            },
            None => Err(UnitError::DoesNotExist),
        }
    }

    /// Stop all running units
    pub fn stop_all_units() {
        unimplemented!();
    }

    /// Stop a running unit
    pub fn stop_unit(name: &str) {
        unimplemented!();
    }

    pub fn restart_unit(unit: &mut UnitInfo) {
        match unit.unit.borrow_mut().child.as_mut().unwrap().try_wait() {
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

    pub fn units_alive(&self) -> bool {
        for u in self.units.iter() {
            if u.1.unit.borrow().is_alive() {
                return true;
            }
        }

        false
    }
}
