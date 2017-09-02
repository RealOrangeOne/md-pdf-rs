use config::Config;

pub mod pdf;

pub fn output(config: Config, output: String) -> Result<(), String> {
    if config.output.contains_key("pdf") {
        try!(pdf::output(config, output));
    }
    return Ok(());
}
