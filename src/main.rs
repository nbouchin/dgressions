mod mount;

fn main() {
    let mut kfs: mount::KernelFilesystem = mount::KernelFilesystem::new();

    kfs.add(mount::MountInfo::new("/dev/sda1", "/mnt", "ext4", libc::MS_RDONLY, ""));// mnt
    kfs.add(mount::MountInfo::new("/dev/sda2", "/mnt", "ext4", libc::MS_RDONLY, ""));// sys
    kfs.add(mount::MountInfo::new("/dev/sda3", "/mnt", "ext4", libc::MS_RDONLY, ""));// var
    kfs.add(mount::MountInfo::new("/dev/sda4", "/mnt", "ext4", libc::MS_RDONLY, ""));// proc

    kfs.mount_kernel_filesystem();
}
