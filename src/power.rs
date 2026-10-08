use serde::{Deserialize, Serialize};
use std::{
    os::unix::process::CommandExt,
    process::{Command, Stdio, exit},
};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Lock,
    Sleep,
    Hibernate,
    Reboot,
    Shutdown,
}

impl Action {
    pub const ALL: [Action; 5] = [
        Self::Lock,
        Self::Sleep,
        Self::Hibernate,
        Self::Reboot,
        Self::Shutdown,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Action::Lock => "Lock",
            Action::Sleep => "Sleep",
            Action::Hibernate => "Hibernate",
            Action::Reboot => "Reboot",
            Action::Shutdown => "Shutdown",
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Cmd {
    command: String,
    args: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PowerConfig {
    pub lock_cmd: Cmd,
    pub sleep_cmd: Cmd,
    pub hibernate_cmd: Cmd,
    pub reboot_cmd: Cmd,
    pub shutdown_cmd: Cmd,
}

impl PowerConfig {
    pub fn cmd(&self, action: Action) -> &Cmd {
        match action {
            Action::Lock => &self.lock_cmd,
            Action::Sleep => &self.sleep_cmd,
            Action::Hibernate => &self.hibernate_cmd,
            Action::Reboot => &self.reboot_cmd,
            Action::Shutdown => &self.shutdown_cmd,
        }
    }
}

impl Default for PowerConfig {
    fn default() -> Self {
        PowerConfig {
            lock_cmd: Cmd {
                command: String::from("hyprlock"),
                args: Vec::new(),
            },
            sleep_cmd: Cmd {
                command: String::from("systemctl"),
                args: vec![String::from("suspend")],
            },
            hibernate_cmd: Cmd {
                command: String::from("systemctl"),
                args: vec![String::from("hibernate")],
            },
            reboot_cmd: Cmd {
                command: String::from("systemctl"),
                args: vec![String::from("reboot")],
            },
            shutdown_cmd: Cmd {
                command: String::from("systemctl"),
                args: vec![String::from("poweroff")],
            },
        }
    }
}

pub fn run_cmd(cmd: &Cmd) {
    Command::new(&cmd.command)
        .args(&cmd.args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .process_group(0)
        .spawn()
        .expect("failed to spawn command");
    exit(0);
}
