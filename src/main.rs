mod mount;
use libc::*;
use std::process::Command;
use std::io::{self, Write};

fn main() {
	let mut kfs: mount::KernelFilesystem = mount::KernelFilesystem::new();

	kfs.add(mount::MountInfo::new("rootfs", "/", "ext4", MS_RDONLY|MS_REMOUNT, ""));

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

	kfs.add(mount::MountInfo::new("rootfs", "/", "ext4", MS_REMOUNT, ""));
	kfs.add(mount::MountInfo::new("udev", "/dev", "devtmpfs", MS_NOSUID|MS_RELATIME, ""));
	kfs.add(mount::MountInfo::new("sysfs", "/sys", "sysfs", MS_NOSUID|MS_NODEV|MS_NOEXEC|MS_RELATIME, ""));
	kfs.add(mount::MountInfo::new("proc", "/proc", "proc", MS_NOSUID|MS_NODEV|MS_NOEXEC|MS_RELATIME, ""));

	kfs.mount_kernel_filesystem();
	Command::new("/bin/mount")
		.spawn()
		.expect("mount command failed to start");
	Command::new("/bin/bash")
		.spawn()
		.expect("bash command failed to start");
    Command::new("/usr/sbin/agetty")
        .args("/dev/tty10")
        .spawn()
        .expect("getty failed");
    loop {
    }
}
