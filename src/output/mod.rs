use config::{Config, OutputType};

pub mod pdf;

pub fn output(config: Config, output: String) -> Result<(), String> {
    if config.has_output(OutputType::PDF) {
        try!(pdf::output(config, output));
    }
    return Ok(());
}
