use serde_yaml::Value;
use std::vec::Vec;

pub type ValidationResult = Result<(), String>;


fn check_required_keys(config: &Value) -> ValidationResult {
    for key in vec!(
        "input",
        "output",
        "title"
    ).iter() {
        if config.get(key).is_none() {
            return Err("Missing key".into());
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
        &check_required_keys
    ));
}
