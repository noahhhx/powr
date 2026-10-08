use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::{colour::ColourConfig, power::PowerConfig};

pub enum ConfigError {
    IoError(std::io::Error),
    InvalidConfig(toml::de::Error),
}

impl From<std::io::Error> for ConfigError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(value: toml::de::Error) -> Self {
        Self::InvalidConfig(value)
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AppConfig {
    pub colour: ColourConfig,
    pub power: PowerConfig,
}

pub fn load_or_init() -> Result<AppConfig, ConfigError> {
    let mut config_home = std::env::var("XDG_CONFIG_HOME").unwrap_or_default();
    if config_home.is_empty() {
        config_home = std::env::var("HOME").unwrap() + "/.config";
    };
    let powr_cfg_home = config_home + "/powr/config.toml";
    let powr_cfg_path = Path::new(&powr_cfg_home);

    if powr_cfg_path.exists() {
        let content = fs::read_to_string(powr_cfg_path)?;
        let config = toml::from_str(&content)?;
        return Ok(config);
    }

    let config = AppConfig::default();
    let toml = toml::to_string(&config).unwrap();

    fs::create_dir_all(powr_cfg_path.parent().unwrap()).unwrap();
    fs::write(powr_cfg_path, toml)?;
    Ok(config)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let user_home = std::env::var("XDG_CONFIG_HOME").unwrap();
        println!("{user_home}")
    }
}
