use config::Config;

use renderers::html_cleanup::html_cleanup;
use renderers::strip_blank::strip_blank;

pub fn render(config: Config, input: String) -> Result<String, String> {
    let mut rendered_input = input;
    let renderers: Vec<fn(Config, String) -> Result<String, String>> =
        vec![html_cleanup, strip_blank];
    for renderer in renderers {
        rendered_input = try!(renderer(config.clone(), rendered_input));
    }
    return Ok(rendered_input);
}
