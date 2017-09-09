pub mod pandoc;
pub mod process;
mod csl;

use config::Config;


pub fn build_input(config: Config, input: String) -> Result<String, String> {
    let html = try!(pandoc::render(input));
    let rendered = try!(process::render(config.clone(), html));
    return Ok(rendered);
}
