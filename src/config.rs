use anyhow::Result;
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub verbose: Option<bool>,
}

impl Config {
    pub fn load(path: Option<&str>) -> Result<Self> {
        if let Some(p) = path {
            let content = fs::read_to_string(Path::new(p))?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Config { verbose: Some(false) })
        }
    }
}