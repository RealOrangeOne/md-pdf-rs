use config::Config;

use std::vec::Vec;


pub fn unwrap_group(config: &Config, funcs: Vec<&Fn(&Config) -> Result<(), String>>) -> Result<(), String> {
    for func in funcs.iter() {
        let func_result = func(config);
        if func_result.is_err() {
            return Err(func_result.unwrap_err());
        }
    }
    return Ok(());
}


pub fn validate(config: Config) -> Result<(), String> {
    return unwrap_group(&config, vec!(
        &|c| { Ok(()) }
    ));
}
