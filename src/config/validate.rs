use serde_yaml::Value;
use std::vec::Vec;

pub type ValidationResult = Result<(), String>;

use config::read;


fn check_required_keys(config: &Value) -> ValidationResult {
    for key in vec!(
        "input",
        "output",
        "title"
    ).iter() {
        if config.get(key).is_none() {
            return Err(format!("Missing required key {}.", key));
        }
    }
    return Ok(());
}

fn check_input_files(config: &Value) -> ValidationResult {
    let files = read::get_input_files(config);
    for file in files.iter() {
        if !file.exists() || !file.is_file() {
            return Err(format!("Cannot find input file at {}.", file.as_path().display()));
        }
    }
    return Ok(());
}


pub fn unwrap_group(config: &Value, funcs: Vec<&Fn(&Value) -> ValidationResult>) -> ValidationResult {
    for func in funcs.iter() {
        let func_result = func(config);
        if func_result.is_err() {
            return Err(func_result.unwrap_err());
        }
    }
    return Ok(());
}


pub fn validate(config: &Value) -> ValidationResult {
    return unwrap_group(config, vec!(
        &check_required_keys,
        &check_input_files
    ));
}
