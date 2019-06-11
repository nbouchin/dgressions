use std::io::{Result, Error};
use std::ffi::{CString};
use std::ptr::{null_mut};

#[allow(unused)]
fn stat(pathname: &str, buf: *mut libc::stat) -> Result<()> {
    let ret;

    unsafe {
        ret = libc::stat(CString::new(pathname).unwrap().into_raw(), buf);
    }
    if ret == -1 {
        return Err(Error::last_os_error());
    }
    return Ok(());
}

#[allow(unused)]
fn mount(source: &str, target: &str, file_system_type: &str, mount_flags: u64,
         _data: &str) -> Result<()> {

    let ret;

    unsafe {
        ret = libc::mount(CString::new(source).unwrap().into_raw(),
        CString::new(target).unwrap().into_raw(),
        CString::new(file_system_type).unwrap().into_raw(),
        mount_flags,
        null_mut())
    }

    if ret == -1 {
        return Err(Error::last_os_error());
    }
    return Ok(());
}

const NR_INIT_MODULE: i64 = 105;
const NR_FINIT_MODULE: i64 = 273;

#[allow(unused)]
fn init_module(module_image: *mut libc::c_void, len: libc::c_ulong,
               param_values: *const libc::c_char) -> i64 {
    unsafe { libc::syscall(NR_INIT_MODULE, module_image, len, param_values) }
}

#[allow(unused)]
fn finit_module(fd: i32, param_values: *const libc::c_char,
                flags: i32) -> i64 {
    unsafe { libc::syscall(NR_FINIT_MODULE, fd, param_values, flags) }
}

#[allow(unused)]
fn to_exec_array(args: &[&str]) -> Vec<*const libc::c_char> {
    use libc::c_char;
    use std::ptr;

    let mut args_p: Vec<*const c_char> = args.iter().map(|s| s.as_ptr() as *const c_char).collect();
    args_p.push(ptr::null());
    args_p
}

#[allow(unused)]
fn execv(path: &str, argv: &[&str]) {
    let args_p = to_exec_array(&argv);

    unsafe {
        libc::execv(CString::new(path).unwrap().into_raw(), args_p.as_ptr());
    }
}

fn syslog(loglevel: i32, args: &[&str]) {
    unsafe { libc::syslog(loglevel, to_exec_array(args).as_ptr() as *const i8); };
}

fn start_udev() {
    use std::process::Command;

    syslog(libc::LOG_CONS, &["Starting udevd..."]);
    Command::new("/sbin/udevd")
        .arg("--daemon")
        .output()
        .expect("failed udevd");

    Command::new("/sbin/udevadm")
        .arg("trigger")
        .arg("--action=add")
        .arg("--type=subsystems")
        .output()
        .expect("failed udevadm");

    Command::new("/sbin/udevadm")
        .arg("trigger")
        .arg("--action=add")
        .arg("--type=devices")
        .output()
        .expect("failed udevadm");

    Command::new("/sbin/udevadm")
        .arg("trigger")
        .arg("--action=change")
        .arg("--type=devices")
        .output()
        .expect("failed udevadm");
}

fn main() {
    start_udev();
}
