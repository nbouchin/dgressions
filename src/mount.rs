#![allow(unused)]
use std::io::{Result, Error};
use std::collections::linked_list::LinkedList;

pub struct MountInfo {
    source: &'static str,
    target: &'static str,
    file_system_type: &'static str,
    mount_flags: u64,
    _data: &'static str,
}

impl MountInfo {
    pub fn new(source: &'static str, target: &'static str, file_system_type: &'static str, mount_flags: u64, _data: &'static str) -> MountInfo {
        MountInfo { source, target, file_system_type, mount_flags, _data }
    }
}

pub struct KernelFilesystem {
    fs_vect: Vec<MountInfo>,
}

impl KernelFilesystem {
    pub fn new() -> KernelFilesystem {
        KernelFilesystem {
            fs_vect: Vec::new(),
        }
    }

    pub fn add(&mut self, mi: MountInfo) {
        self.fs_vect.push(mi);
    }

    pub fn mount_kernel_filesystem(&mut self) {
        for i in self.fs_vect.iter() {
           if let Err(e) = mount(i) {
               println!("{}", e);
           }
        }
    }
}

fn mount(mount: &MountInfo) -> Result<()> {
    let ret;

    unsafe {
        ret = libc::mount(std::ffi::CString::new(mount.source).unwrap().into_raw(),
        std::ffi::CString::new(mount.target).unwrap().into_raw(),
        std::ffi::CString::new(mount.file_system_type).unwrap().into_raw(),
        mount.mount_flags,
        std::ptr::null_mut())
    }

    if ret == -1 {
        return Err(Error::last_os_error());
    }
    return Ok(());
}
