use serde_yaml;

pub mod read;
pub mod validate;
pub mod consts;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    foo: String
}

impl Config {
    fn new(raw: String) -> Config {
        return serde_yaml::from_str(&raw).unwrap();
    }
}


pub fn get_config() {
    let config_str = read::read();
    let config = Config::new(config_str);
    println!("{:?}", config);
    validate::validate(config);
}
