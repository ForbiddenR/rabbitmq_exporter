use std::{error::Error, fs};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub rabbit_url: String,
    pub rabbit_user: String,
    pub rabbit_pass: String,
    pub publish_port: u16,
    pub enabled_exporters: Vec<String>,
    pub timeout: u32,
}

impl Config {
    pub fn read(path: &str) -> Result<Config, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }
}
