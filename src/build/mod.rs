pub mod pandoc;

use config::Config;


pub fn build_input(config: &Config, input: String) -> Result<String, String> {
    pandoc::render(&config, input);
    return Ok("".into());
}
