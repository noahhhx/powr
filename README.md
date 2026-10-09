# powr

> A simple power menu for Linux & Hyprlan, written in Rust with [iced](https://github.com/iced-rs/iced).

## Developing

The dev environment uses devenv. It provides stable Rust, required runtime libs and a clippy pre-commit hook. If you use direnv, run `direnv allow` once. Otherwise, run `devenv shell`.

```sh
devenv shell
cargo run
```

## Installing

TODO - Nix + Hyprland

## Configuration

The styling and behaviour of Powr is config driven, on your first time running Powr a default config file we be created `.config/powr/config.toml`.

Use hex colour codes to customise the look of the application https://www.color-hex.com/, and modify the behaviour of the buttons by the commands that are ran:

```toml
[colour]
background = "#0f0f0f"
button_active_background = "#2a2a2a"
button_active_text = "#89b4fa"
button_inactive_background = "#0f0f0f"
button_inactive_text = "#a6adc8"

[power.lock_cmd]
command = "hyprlock"
args = []

[power.sleep_cmd]
command = "systemctl"
args = ["suspend"]

[power.hibernate_cmd]
command = "systemctl"
args = ["hibernate"]

[power.reboot_cmd]
command = "systemctl"
args = ["reboot"]

[power.shutdown_cmd]
command = "systemctl"
args = ["poweroff"]
```
