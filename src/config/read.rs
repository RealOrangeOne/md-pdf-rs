use serde_yaml;
use utils::{result_prefix, result_override};
use config::Config;
use config::consts;
use std::io::Read;
use std::fs::File;
use std::env::current_dir;
use std::path::PathBuf;


fn get_config_path() -> PathBuf {
    let mut working_dir = current_dir().unwrap();
    working_dir.push(consts::CONFIG_FILE_NAME);
    return working_dir;
}

fn read() -> Result<String, String> {
    let config_path = get_config_path();
    let mut config_file = try!(result_override(
        File::open(&config_path),
        format!("Unable to find config file at {}", config_path.display())
    ));
    let mut contents = String::new();
    try!(result_override(
        config_file.read_to_string(&mut contents),
        format!("Failed to read config file at {}.", config_path.display())
    ));
    return Ok(contents);
}


pub fn get_config() -> Result<Config, String> {
    let config_str = try!(read());
    let config: Config =
        try!(result_prefix(serde_yaml::from_str(&config_str), "Config Parse Error".into()));
    return Ok(config);
}
