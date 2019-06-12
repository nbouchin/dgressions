mod mount;
use std::process::Command;

fn main() {
    if let Err(e) = mount::mount_fs() {
        println!("{}", e);
    }

    Command::new("/bin/mount")
        .spawn()
        .expect("mount command failed to start");
//    Command::new("/bin/bash")
//        .spawn()
//        .expect("bash command failed to start");

    Command::new("/sbin/agetty")
        .arg("tty10")
        .arg("9600")
        .spawn()
        .expect("agetty failed");
    loop {
    }
}
