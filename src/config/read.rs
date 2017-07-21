use std::env::current_dir;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use serde_yaml::Value;
use std::collections::HashMap;

use config::consts;


fn get_config_path() -> PathBuf {
    let mut working_dir = current_dir().unwrap();
    working_dir.push(consts::CONFIG_FILE);
    return working_dir;
}


pub fn read() -> Result<String, String> {
    let config_path = get_config_path();
    if !config_path.is_file() {
        return Err(format!("Failed to find config file at {}.", config_path.display()));
    }
    let file = File::open(config_path);
    if file.is_err() {
        return Err("Failed to open file".into());
    }
    let mut config_file = file.unwrap();
    let mut contents = String::new();
    let file_read = config_file.read_to_string(&mut contents);
    if file_read.is_err() {
        return Err("Failed to read config file".into());
    }
    return Ok(contents);
}


fn to_string(data: &Value) -> String {
    return data.as_str().unwrap().to_string();
}


pub fn get_string(conf: &Value, key: &str) -> String {
    return to_string(conf.get(key).unwrap());
}


pub fn get_input_files(conf: &Value) -> Vec<PathBuf> {
    let working_dir = current_dir().unwrap();
    let input_values = conf.get("input").unwrap().as_sequence().unwrap().to_vec();
    return input_values.into_iter().map(|x| working_dir.join(to_string(&x))).collect();
}


pub fn get_output_files(conf: &Value) -> HashMap<String, PathBuf> {
    let working_dir = current_dir().unwrap();
    let output_raw = conf.get("output").unwrap().as_mapping().unwrap();
    let mut output_map: HashMap<String, PathBuf> = HashMap::new();
    for output in output_raw.into_iter() {
        output_map.insert(to_string(output.0), working_dir.join(to_string(output.1)));
    }
    return output_map;
}
