pub mod pandoc;
pub mod render;

use config::Config;


pub fn build_input(config: Config, input: String) -> Result<String, String> {
    let html = try!(pandoc::render(config.clone(), input));
    let rendered = try!(render::render(config.clone(), html));
    return Ok(rendered);
}
