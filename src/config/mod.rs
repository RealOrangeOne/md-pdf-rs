use std::path::PathBuf;
use std::collections::HashMap;
use utils::{result_prefix, resolve_path, result_override};
use std::env::current_dir;


pub mod consts;
pub mod csl;
pub mod read;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum OutputType {
    PDF,
    HTML
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    input: Vec<PathBuf>,
    output: HashMap<OutputType, PathBuf>,
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
    pub fn absolute_output(&self, output_type: OutputType) -> PathBuf {
        return resolve_path(self.output.get(&output_type).unwrap());
    }

    pub fn has_output(&self, output_type: OutputType) -> bool {
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
