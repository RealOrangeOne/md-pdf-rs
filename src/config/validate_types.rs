use config::validate::{unwrap_group, ValidationResult};
use serde_yaml::Value;


fn check_root(config: Value) -> ValidationResult {
    if !config.is_mapping() {
        return Err("Config should be a mapping".into());
    }
    return Ok(());
}

fn check_input(config: Value) -> ValidationResult {
    let input = config.get("input").unwrap();
    if !input.is_sequence() {
        return Err("Input must be sequence".into());
    }

    if input.as_sequence().into_iter().count() == 0 {
        return Err("Must provide input files".into());
    }

    for input_file in input.as_sequence().unwrap() {
        if !input_file.is_string() {
            return Err("Input must be string".into());
        }
    }
    return Ok(());
}

fn check_output(config: Value) -> ValidationResult {
    let output = config.get("output").unwrap();
    if !output.is_mapping() {
        return Err("Output must be mapping".into());
    }

    if output.as_mapping().into_iter().count() == 0 {
        return Err("Must provide output files".into());
    }

    for output_def in output.as_mapping().unwrap() {
        if !output_def.0.is_string() {
            return Err("Output keys must be strings".into());
        }
        if !output_def.1.is_string() {
            return Err("Output values must be strings".into());
        }
    }
    return Ok(());
}

fn check_title(config: Value) -> ValidationResult {
    if !config.get("title").unwrap().is_string() {
        return Err("Title should be a string".into());
    }
    return Ok(());
}

fn check_references(config: Value) -> ValidationResult {
    if config.get("references").is_none() {
        return Ok(()); // references is optional, dont type it if it's not there
    }
    if !config.get("references").unwrap().is_mapping() {
        return Err("References should be mapping".into());
    }
    let references = config.get("references").unwrap().as_mapping().unwrap();
    let csl = references.get(&Value::String("csl".into()));
    let bibliography = references.get(&Value::String("bibliography".into()));
    if csl.is_none() {
        return Err("Missing CSL file".into());
    }
    if bibliography.is_none() {
        return Err("Missing Bibliography".into());
    }
    if !csl.unwrap().is_string() {
        return Err("CSL file must be a string".into());
    }
    if !bibliography.unwrap().is_string() {
        return Err("Bibliography must be a string".into());
    }
    return Ok(());
}


pub fn check_config_types(config: Value) -> ValidationResult {
    return unwrap_group(
        config,
        vec![&check_root, &check_input, &check_output, &check_title, &check_references]
    );
}
