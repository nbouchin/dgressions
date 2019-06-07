mod mount;
use libc::*;

fn main() {
    let mut kfs: mount::KernelFilesystem = mount::KernelFilesystem::new();

    kfs.add(mount::MountInfo::new("udev", "/mnt/dev", "devtmpfs", MS_NOSUID|MS_RELATIME, ""));
    kfs.add(mount::MountInfo::new("sysfs", "/mnt/sys", "sysfs", MS_NOSUID|MS_NODEV|MS_NOEXEC|MS_RELATIME, ""));
//    kfs.add(mount::MountInfo::new("var", "/var", "iso9660", 0, ""));// find info on var
    kfs.add(mount::MountInfo::new("proc", "/mnt/proc", "proc", MS_NOSUID|MS_NODEV|MS_NOEXEC|MS_RELATIME, ""));
    kfs.add(mount::MountInfo::new("/mnt/dev/vda1", "/mnt/root", "ext4", MS_RDONLY, ""));

    kfs.mount_kernel_filesystem();
}
