use std::env::current_dir;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use serde_yaml::Value;

use config::consts;


fn get_config_path() -> PathBuf {
    let mut working_dir = current_dir().unwrap();
    working_dir.push(consts::CONFIG_FILE);
    return working_dir;
}


pub fn read() -> String {
    let mut config_file = File::open(get_config_path()).expect("Failed to open file");
    let mut contents = String::new();
    config_file.read_to_string(&mut contents).expect("Failed to read file");
    return contents;
}


pub fn get_input_files(conf: Value) -> Vec<PathBuf> {
    let working_dir = current_dir().unwrap();
    let input_values = conf.get("input").unwrap().as_sequence().unwrap().to_vec();
    return input_values.into_iter().map(|x| working_dir.join(x.as_str().unwrap().to_string())).collect();
}
