//! Controls every unit

use crate::unit;
use std::path::Path;

pub struct Master {
    units: Vec<unit::Unit>
}

impl Master {
    pub fn new() -> Master {
        Master {
            units: vec!()
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
                        println!("Loading unit {}", &path.to_str().unwrap());
                        let u = unit::Unit::new(&path);
                        self.units.push(u);
                    },
                    Err(e) => { println!("{:?}", e); },
                };
            }
        }
    }
}
