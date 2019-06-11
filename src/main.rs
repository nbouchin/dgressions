use std::io::{Result, Error};
use std::ffi::{CString};
use std::ptr::{null_mut};
use libc::{MS_RDONLY};

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

const NR_init_module: i64 = 105;
const NR_finit_module: i64 = 273;

#[allow(unused)]
fn init_module(module_image: *mut libc::c_void, len: libc::c_ulong,
               param_values: *const libc::c_char) -> i64 {
    unsafe { libc::syscall(NR_init_module, module_image, len, param_values) }
}

#[allow(unused)]
fn finit_module(fd: i32, param_values: *const libc::c_char,
                flags: i32) -> i64 {
    unsafe { libc::syscall(NR_finit_module, fd, param_values, flags) }
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

fn main() {
    use std::process::Command;

    match Command::new("/sbin/udevd")
        .arg("--monitor")
        .output() {
            Ok(output) => println!("{:?}", output),
            Err(e) => println!("{}", e),
        }
}
