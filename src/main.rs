extern crate env_logger;
#[macro_use]
extern crate log;

use std::cell::RefCell;
use std::path::Path;
//use std::ffi::CStr;

mod dgressions;
mod unit;

//fn load_units_in_folder(path: &Path) {
//    // Read every unit file present inside the directory at `path'
//    if let Ok(dir) = std::fs::read_dir(path) {
//        for entry in dir {
//            let file = entry.unwrap();
//            // Read the content of the file and store it inside a String
//            let content = {
//                let data = std::fs::read(file.path()).unwrap_or_default();
//                String::from_utf8(data).unwrap_or_default()
//            };
//
//            // Print the path and the content of the unit file
//            println!("{}:", &file.path().to_str().unwrap());
//            println!("{}", &content);
//        }
//    }
//}

extern "C" {
    pub fn ctime(time: *const libc::time_t) -> *mut libc::c_char;
}

thread_local!(
    // Global thread-local variable that is set to true when a SIGHUP has been caught
    static SIGHUP_CAUGHT: RefCell<bool> = RefCell::new(false)
);

fn handler_sighup(_signum: i32, _siginfo: *mut libc::siginfo_t, _arg: *mut std::ffi::c_void) {
    SIGHUP_CAUGHT.with(|val| *val.borrow_mut() = true);
}

fn catch_sighup() {
    let mut set = unsafe { std::mem::zeroed::<libc::sigset_t>() };
    unsafe {
        libc::sigemptyset(&mut set);
        libc::sigaddset(&mut set, libc::SIGHUP);
    };

    let sa = libc::sigaction {
        sa_sigaction: handler_sighup as usize,
        sa_mask: set,
        sa_flags: libc::SA_SIGINFO,
        sa_restorer: None,
    };

    unsafe {
        libc::sigaction(
            libc::SIGHUP,
            &sa,
            std::ptr::null_mut() as *mut libc::sigaction,
        );
    };
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    //    let path = Path::new("./nginx.service");
    //    let mut unit: unit::Unit = unit::Unit::new(path);
    //
    //    unit.start();
    //
    //    let mut now: libc::time_t = 0;
    //    unsafe { libc::time(&mut now); };
    //
    //    let local_time = {
    //        let time_now = unsafe { ctime(&now) };
    //        let local_time = unsafe { CStr::from_ptr(time_now) }.to_str().unwrap();
    //
    //        local_time.replace("\n", "")
    //    };
    //
    //    let elapsed = unit.started_at.duration_since(unit.started_at).unwrap();
    //
    //    println!("Active: active (running) since {}; {:?}", local_time, elapsed);

    let mut units = dgressions::Master::load_units_in_folder(Path::new("./units"));

    debug!("Starting all units");
    dgressions::Master::start_all_units(&mut units);

    catch_sighup();

    while dgressions::Master::units_alive(&mut units) {
        let triggered = SIGHUP_CAUGHT.with(|triggered| *triggered.borrow());

        if triggered {
            SIGHUP_CAUGHT.with(|val| *val.borrow_mut() = false);

            // Reload Configuration File
        }

        dgressions::Master::update_units(&mut units);
    }

    Ok(())
}
