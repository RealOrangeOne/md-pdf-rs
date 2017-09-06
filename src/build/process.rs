use config::Config;

use processors::PROCESSORS;

pub fn render(config: Config, input: String) -> Result<String, String> {
    let mut rendered_input = input;
    for renderer in PROCESSORS.into_iter() {
        rendered_input = try!(renderer(config.clone(), rendered_input));
    }
    return Ok(rendered_input);
}
