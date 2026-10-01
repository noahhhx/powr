use std::process::{Command, exit};

pub fn lock() {
    Command::new("hyprlock").output().expect("Uh oh");
    exit(0);
}

pub fn sleep() {
    Command::new("systemctl")
        .arg("suspend")
        .output()
        .expect("uh oh");
    exit(0);
}

pub fn hibernate() {
    println!("hibernate!");
}

pub fn reboot() {
    println!("reboot!");
}

pub fn shutdown() {
    println!("shutdown!");
}
