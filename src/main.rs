mod mount;
use libc::*;
use std::process::Command;

fn main() {
	let mut kfs: mount::KernelFilesystem = mount::KernelFilesystem::new();

	kfs.add(mount::MountInfo::new("rootfs", "/", "ext4", MS_RDONLY|MS_REMOUNT, ""));

	kfs.mount_kernel_filesystem();
	Command::new("fsck")
		.arg("/dev/sda3")
		.spawn()
		.expect("fsck command failed to start");

	kfs.pop();

	kfs.add(mount::MountInfo::new("rootfs", "/", "ext4", MS_REMOUNT, ""));
	kfs.add(mount::MountInfo::new("udev", "/dev", "devtmpfs", MS_NOSUID|MS_RELATIME, ""));
	kfs.add(mount::MountInfo::new("sysfs", "/sys", "sysfs", MS_NOSUID|MS_NODEV|MS_NOEXEC|MS_RELATIME, ""));
	kfs.add(mount::MountInfo::new("proc", "/proc", "proc", MS_NOSUID|MS_NODEV|MS_NOEXEC|MS_RELATIME, ""));

	kfs.mount_kernel_filesystem();
	Command::new("mount")
		.spawn()
		.expect("mount command failed to start");
	loop {
	}
}
