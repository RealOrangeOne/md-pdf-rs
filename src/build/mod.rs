pub mod pandoc;

use config::Config;


pub fn build_input(config: Config, input: String) -> Result<String, String> {
    let html = try!(pandoc::render(config, input));
    return Ok(html);
}
