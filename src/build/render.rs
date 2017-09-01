use config::Config;

use renderers::html_cleanup::html_cleanup;

pub fn render(config: Config, input: String) -> Result<String, String> {
    let mut rendered_input = input;
    for renderer in vec![html_cleanup] {
        rendered_input = try!(renderer(config.clone(), rendered_input));
    }
    return Ok(rendered_input);
}
