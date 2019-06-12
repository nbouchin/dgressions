#![allow(unused)]
use libc::*;
use std::io::{Result, Error};
use std::collections::linked_list::LinkedList;
use std::io::{self, Write};

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

impl PartialEq for MountInfo {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
    }
}

impl Eq for MountInfo {}

pub struct KernelFilesystem {
    fs_vect: Vec<MountInfo>,
}

impl KernelFilesystem {
    pub fn new() -> KernelFilesystem {
        KernelFilesystem {
            fs_vect: Vec::new(),
        }
    }

    pub fn add(&mut self, mi: MountInfo) -> std::result::Result<(), String> {
        if !self.fs_vect.contains(&mi) {
            self.fs_vect.push(mi);
            return Ok(());
        } else {
            return Err(format!("{} already in mount list", mi.source));
        }
    }

    pub fn pop(&mut self) {
        self.fs_vect.pop();
    }

    pub fn mount_kernel_filesystem(&mut self) {
        for i in self.fs_vect.iter() {
            if let Err(e) = mount(i) {
                println!("{}", e);
            }
        }
    }
}

use std::process::Command;

pub fn mount_fs() -> std::result::Result<(), String> {
    let mut kfs: KernelFilesystem = KernelFilesystem::new();

    kfs.add(MountInfo::new("rootfs", "/", "ext4", MS_RDONLY|MS_REMOUNT, ""))?;

    kfs.mount_kernel_filesystem();
    println!("fsck /dev/sda3");

    let output = Command::new("/sbin/fsck")
        .arg("-M")
        .arg("/dev/sda3")
        .output()
        .expect("fsck command failed to start");

    println!("fsck exited with: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    kfs.pop();

    kfs.add(MountInfo::new("rootfs", "/", "ext4", MS_REMOUNT, ""))?;
    kfs.add(MountInfo::new("udev", "/dev", "devtmpfs", MS_NOSUID|MS_RELATIME, ""))?;
    kfs.add(MountInfo::new("sysfs", "/sys", "sysfs", MS_NOSUID|MS_NODEV|MS_NOEXEC|MS_RELATIME, ""))?;
    kfs.add(MountInfo::new("proc", "/proc", "proc", MS_NOSUID|MS_NODEV|MS_NOEXEC|MS_RELATIME, ""))?;

    kfs.mount_kernel_filesystem();
    Ok(())
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
