use serde_yaml;

pub mod read;
pub mod validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    foo: String
}

impl Config {
}


fn construct(data: String) -> Config {
    return serde_yaml::from_str(&data).unwrap();
}


pub fn get_config() {
    let config_str = read::read();
    let config = construct(config_str);
    println!("{:?}", config);
    validate::validate(config);
}
