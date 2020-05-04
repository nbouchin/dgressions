//! Controls every unit

use crate::unit;
use std::path::Path;
use std::collections::HashMap;

struct UnitInfo {
    unit: unit::Unit,
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

type UnitPool = HashMap<String, UnitInfo>;

/// Controls every unit
pub struct Master {
    units: HashMap<String, UnitInfo>,
}

impl Master {
    pub fn new() -> Master {
        Master {
            units: HashMap::new(),
        }
    }

    /// Read every unit files inside a folder and start and manage them
    /// when needed.
    pub fn load_units_in_folder(&mut self, path: &Path) {
        // Read every unit file present inside the directory at `path'
        if let Ok(dir) = std::fs::read_dir(path) {
            for entry in dir {
                match entry {
                    Ok(file) => {
                        let path = file.path();

                        // Isolate the filename in the path. This will be the unit's internal name
                        let filename = match &path.file_name() {
                            Some(os_name) => {
                                match os_name.to_str() {
                                    Some(name) => Some(String::from(name)),
                                    None => None,
                                }
                            },
                            None => None,
                        };

                        // Add the unit to the collection of units
                        if let Some(filename) = filename {
                            println!("Loading unit {}", &path.to_str().unwrap());
                            let u = unit::Unit::new(&path);
                            self.units.insert(filename, UnitInfo::new(u));
                        }
                    },
                    Err(e) => { println!("{:?}", e); },
                };
            }
        }
    }

    /// Cycle through units and update the state of all the units
    pub fn update(&self) {
        unimplemented!();
    }

    /// Start a unit
    fn start_unit(&self, mut unit: unit::Unit) -> Result<unit::Unit, ()> {
        match unit.start() {
            Ok(_) => Ok(unit),
            Err(e) => match e {
                unit::UnitError::AlreadyRunning => {
                    println!("Unit is already running.");
                    Err(())
                },
                _ => { unreachable!(); },
            }
        }
    }

    /// Stop all running units
    fn stop_all_units(&mut self) {
        unimplemented!();
    }

    /// Stop a running unit
    fn stop_unit(&mut self, name: &str) {
        unimplemented!();
    }

    fn restart_unit(&mut self, name: &str) {
    }
}
