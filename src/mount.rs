use std::io::{Result, Error};

struct MountInfo {
    source: &'static str,
    target: &'static str,
    file_system_type: &'static str,
    mount_flags: u64,
    _data: &'static str,
}

impl MountInfo {
    fn new(source: &'static str, target: &'static str, file_system_type: &'static str, mount_flags: u64, _data: &'static str) -> MountInfo {
        MountInfo { source, target, file_system_type, mount_flags, _data }
    }
}

struct KernelFilesystem {
    dev: MountInfo,
    sys: MountInfo,
    var: MountInfo,
    proc: MountInfo,
}

fn mount_kernel_filesystem() {
    let kfs = KernelFilesystem {
        dev: MountInfo::new("/dev/sda1", "/mnt", "ext4", libc::MS_RDONLY, ""),
        sys: MountInfo::new("/dev/sda1", "/mnt", "ext4", libc::MS_RDONLY, ""),
        var: MountInfo::new("/dev/sda1", "/mnt", "ext4", libc::MS_RDONLY, ""),
        proc: MountInfo::new("/dev/sda1", "/mnt", "ext4", libc::MS_RDONLY, ""),
    };

    mount(kfs.dev);
    mount(kfs.sys);
    mount(kfs.var);
    mount(kfs.proc);
}

fn mount(mount: MountInfo) -> Result<()> {

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
