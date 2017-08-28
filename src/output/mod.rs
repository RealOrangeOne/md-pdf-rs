use config::Config;

pub mod pdf;

pub fn output(config: Config, output: String) -> Result<String, String> {
    pdf::output(config, output);
    return Ok("".into());
}
