use serde_yaml;
use serde_yaml::Value;
use std::path::PathBuf;

pub mod read;
pub mod validate;
pub mod consts;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    input: Vec<PathBuf>,

}

impl Config {
    fn new(raw: Value) -> Config {
        return Config {
            input: read::get_input_files(&raw),
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
