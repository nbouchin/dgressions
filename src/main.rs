extern crate env_logger;
#[macro_use]
extern crate log;

use std::cell::RefCell;
use std::path::Path;
use taskmaster::Taskmaster;

mod taskmaster;

thread_local!(
    // Global thread-local variable that is set to true when a SIGHUP has been caught
    static SIGHUP_CAUGHT: RefCell<bool> = RefCell::new(false)
);

fn handler_sighup(_signum: i32, _siginfo: *mut libc::siginfo_t, _arg: *mut std::ffi::c_void) {
    SIGHUP_CAUGHT.with(|val| *val.borrow_mut() = true);
}

fn catch_sighup() {
    debug!("Setting up signal catching for SIGHUP");

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

    debug!("Process is now catching SIGHUP");
}

fn main() -> std::io::Result<()> {
    env_logger::init();

    let mut tm = Taskmaster::new();

    tm.load_units_in_folder(Path::new("./units"));

    println!("taskmaster v0.1.0");
    println!("By nbouchin, oyagci");

    debug!("Starting all units");
    tm.start_all_units();

    catch_sighup();

    while tm.units_alive() {
        let triggered = SIGHUP_CAUGHT.with(|triggered| *triggered.borrow());

        if triggered {
            SIGHUP_CAUGHT.with(|val| *val.borrow_mut() = false);

            // Reload Configuration File
        }

        tm.update_units();
    }

    Ok(())
}
