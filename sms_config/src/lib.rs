use dirs::config_dir;

use crate::config::SmsConfig;

pub mod config;

#[derive(Debug)]
pub enum ConfigError {
    ConfigDirNotFound,
    ConfigFileNotFound,
    ConfigFileParseError(String),
}

pub fn load_config() -> Result<SmsConfig, ConfigError> {
    let config_path = config_dir().map(|config_dir| config_dir.join("sms_modem/config.toml"));
    match config_path {
        None => Err(ConfigError::ConfigDirNotFound),
        Some(p) if !p.exists() => Err(ConfigError::ConfigFileNotFound),
        Some(path) => {
            let config_content = std::fs::read_to_string(path).map_err(|e| {
                ConfigError::ConfigFileParseError(format!(
                    "Could not read config file: 'config.toml', Reason: {}",
                    e
                ))
            })?;
            toml::from_str(&config_content).map_err(|e| {
                ConfigError::ConfigFileParseError(format!(
                    "Could not parse config file: 'config.toml', Reason: {}",
                    e
                ))
            })
        }
    }
}
