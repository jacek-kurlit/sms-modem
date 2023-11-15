use dirs::config_dir;
use std::sync::OnceLock;

use crate::config::SmsConfig;

pub mod config;

#[derive(Debug)]
pub enum ConfigError {
    AlreadyInitialized,
    ConfigFileParseError(String),
}

pub static CONFIG: OnceLock<SmsConfig> = OnceLock::new();

pub fn init() -> Result<(), ConfigError> {
    let config = load_config()?;
    CONFIG
        .set(config)
        .map_err(|_| ConfigError::AlreadyInitialized)
}

pub fn get() -> &'static SmsConfig {
    CONFIG
        .get()
        .expect("Config not initialized. Call init_config before this method!")
}

fn load_config() -> Result<SmsConfig, ConfigError> {
    let config_path = config_dir().map(|config_dir| config_dir.join("sms_modem/config.toml"));
    match config_path {
        Some(path) if path.exists() => {
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
        _ => Ok(SmsConfig::default()),
    }
}
