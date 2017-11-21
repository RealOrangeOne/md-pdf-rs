use serde_yaml;
use serde_yaml::Value;
use std::path::PathBuf;
use std::collections::HashMap;
use utils::{result_prefix, resolve_path, result_override};
use std::fs::{remove_file, File};
use std::env::current_dir;
use std::io::Read;

pub mod consts;
pub mod csl;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    input: Vec<PathBuf>,
    output: HashMap<String, PathBuf>,
    pub title: String,

    #[serde(default = "default_verbosity")]
    pub verbosity: u64,

    pub references: Option<References>
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct References {
    bibliography: PathBuf,
    csl: String,
    pub inner_csl: Option<String>
}


impl Config {
    pub fn absolute_output(&self, output_type: String) -> PathBuf {
        return resolve_path(self.output.get(&output_type).unwrap());
    }

    pub fn has_output(&self, output_type: String) -> bool {
        return self.output.contains_key(&output_type);
    }

    pub fn absolute_inputs(&self) -> Vec<PathBuf> {
        let working_dir = current_dir().unwrap();
        return self.input.iter().map(|x| working_dir.join(&x)).collect();
    }
}

impl References {
    pub fn absolute_bibliography(&self) -> PathBuf {
        return resolve_path(&self.bibliography);
    }

    pub fn absolute_csl(&mut self) -> PathBuf {
        let tmp_csl_path = csl::unpack_csl(self.csl.as_str().into());
        self.inner_csl = Some(tmp_csl_path.as_path().to_str().unwrap().into());
        return tmp_csl_path;
    }
}

fn default_verbosity() -> u64 {
    return 0;
}

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
