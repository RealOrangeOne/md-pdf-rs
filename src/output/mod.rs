use config::Config;

pub mod pdf;

pub fn output(config: Config, output: String) -> Result<(), String> {
    if config.has_output("pdf".into()) {
        try!(pdf::output(config, output));
    }
    return Ok(());
}
