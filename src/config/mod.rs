use serde_yaml;
use serde_yaml::Value;
use std::path::PathBuf;
use std::collections::HashMap;

pub mod read;
pub mod validate;
pub mod consts;
pub mod validate_types;


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub input: Vec<PathBuf>,
    pub output: HashMap<String, PathBuf>,
    pub title: String,
    pub verbosity: u64
}

impl Config {
    fn new(raw: Value) -> Config {
        return Config {
            input: read::get_input_files(&raw),
            output: read::get_output_files(&raw),
            title: read::get_string(&raw, "title"),
            ..Default::default()
        };
    }
}


pub fn get_config() -> Result<Config, String> {
    let config_str = read::read();
    if config_str.is_err() {
        return Err(config_str.unwrap_err());
    }
    let config_value = serde_yaml::from_str(&config_str.unwrap());
    if config_value.is_err() {
        return Err(format!("Failed to parse config. {}", config_value.unwrap_err()));
    }
    let config = config_value.unwrap();
    let validation_output = validate::validate(&config);
    if validation_output.is_err() {
        return Err(format!("Validation error: {}", validation_output.unwrap_err()));
    };
    return Ok(Config::new(config));
}
