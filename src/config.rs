use color_eyre::eyre::Result;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("config file not accessible")]
    NotAccessible,
    #[error("value `{value_name:?}` out of range (expected a value between {from:?}-{to:?})")]
    OutOfRange {
        value_name: String,
        from: i32,
        to: i32,
    },
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    stop_charging_threshold: i32,
    start_charging_threshold: i32,
}

impl Config {
    pub fn read_from_file(path: &str) -> Result<Self, ConfigError> {
        let config_content = fs::read_to_string(path).or(Err(ConfigError::NotAccessible))?;
        let config = toml::from_str::<Self>(&config_content).or(Err(ConfigError::NotAccessible))?;

        config.validate()
    }

    pub fn get_stop_charging_threshold(&self) -> i32 {
        self.stop_charging_threshold
    }

    pub fn get_start_charging_threshold(&self) -> i32 {
        self.start_charging_threshold
    }

    fn validate(self) -> Result<Self, ConfigError> {
        if !(0..=100).contains(&self.start_charging_threshold) {
            return Err(ConfigError::OutOfRange {
                value_name: String::from("start_charging_threshold"),
                from: 0,
                to: 100,
            });
        }

        Ok(self)
    }
}
