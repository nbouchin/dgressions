mod mount;

fn main() {
    let mut kfs: mount::KernelFilesystem = mount::KernelFilesystem::new();

    kfs.add(mount::MountInfo::new("/dev/sda1", "/mnt/mnt_dest1", "iso9660", libc::MS_RDONLY, ""));// mnt
    kfs.add(mount::MountInfo::new("/dev/sda1", "/mnt/mnt_dest2", "iso9660", libc::MS_RDONLY, ""));// sys
    kfs.add(mount::MountInfo::new("/dev/sda1", "/mnt/mnt_dest3", "iso9660", libc::MS_RDONLY, ""));// var
    kfs.add(mount::MountInfo::new("/dev/sda1", "/mnt/mnt_dest4", "iso9660", libc::MS_RDONLY, ""));// proc

    //kfs.mount_kernel_filesystem();
}
