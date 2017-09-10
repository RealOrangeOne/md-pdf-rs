use serde_yaml::Value;
use std::vec::Vec;
use config::read;
use config::validate_types::check_config_types;
use utils::resolve_path;
use build::csl::is_valid_csl;


pub type ValidationResult = Result<(), String>;


fn check_required_keys(config: Value) -> ValidationResult {
    for key in vec!["input", "output", "title"].iter() {
        if config.get(key).is_none() {
            return Err(format!("Missing required key {}.", key));
        }
    }
    return Ok(());
}

fn check_input_files(config: Value) -> ValidationResult {
    let files = read::get_input_files(config);

    for file in files.iter() {
        if !file.exists() || !file.is_file() {
            return Err(format!("Cannot find input file at {}.", file.as_path().display()));
        }
    }
    return Ok(());
}

fn check_output_files(config: Value) -> ValidationResult {
    let files = read::get_output_files(config);
    let output_types = vec!["pdf".into()];
    if files.is_empty() {
        return Err("You need to provide at least 1 output format".into());
    }
    for file_def in files.iter() {
        let dir = file_def.1.parent().unwrap();
        if !dir.exists() || !dir.is_dir() {
            return Err(format!("Cannot find output directory at {}.", dir.display()));
        }
        if !output_types.contains(file_def.0) {
            return Err(format!("Invalid output type {}.", file_def.0));
        }
    }
    return Ok(());
}

fn check_references(config: Value) -> ValidationResult {
    if config.get("references").is_none() {
        return Ok(());
    }
    let references = config.get("references").unwrap();
    let bibliography =
        resolve_path(references.get("bibliography").unwrap().as_str().unwrap().into());
    let valid_extensions = vec![
        "bib",
        "bibtex",
        "copac",
        "json",
        "yaml",
        "enl",
        "xml",
        "wos",
        "medline",
        "mods",
        "ris",
    ];
    if !bibliography.exists() {
        return Err(format!("Can't find bibliography at {}.", bibliography.display()));
    }
    if !valid_extensions.contains(&bibliography.extension().unwrap().to_str().unwrap()) {
        return Err(format!("Bibliography extension must be one of {:?}.", valid_extensions));
    }
    let csl = references.get("csl").unwrap().as_str().unwrap();
    if !is_valid_csl(csl.into()) {
        return Err(format!("{} isnt a valid CSL config.", csl));
    }
    return Ok(());
}

pub fn unwrap_group(config: Value, funcs: Vec<&Fn(Value) -> ValidationResult>) -> ValidationResult {
    for func in funcs.iter() {
        try!(func(config.clone()));
    }
    return Ok(());
}


pub fn validate(config: Value) -> ValidationResult {
    return unwrap_group(
        config,
        vec![
            &check_required_keys,
            &check_config_types,
            &check_input_files,
            &check_output_files,
            &check_references,
        ]
    );
}
