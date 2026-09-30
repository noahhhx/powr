use std::process::{Command, exit};

pub fn lock() {
    println!("locked!");
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
