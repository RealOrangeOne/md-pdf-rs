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
    pub verbosity: u64,
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


pub fn get_config() -> Config {
    let config_str = read::read();
    let config_value: Value = serde_yaml::from_str(&config_str).unwrap();
    validate::validate(&config_value).expect("Validation Error");
    return Config::new(config_value);
}
