use serde_yaml;
use serde_yaml::Value;

pub mod read;
pub mod validate;
pub mod consts;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    raw: String,
    input: Vec<String>,

}

impl Config {
    fn new(raw: String) -> Config {
        let raw_conf: Value = serde_yaml::from_str(&raw).unwrap();
        return Config {
            raw: raw,
            input: read::get_inputs(raw_conf),
            ..Default::default()
        };
    }
}


pub fn get_config() -> Config {
    let config_str = read::read();

    return Config::new(config_str);
}
