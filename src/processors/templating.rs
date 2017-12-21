use config::Config;
use templating::{render_template, get_errors};


pub fn templating(config: Config, input: String) -> Result<String, String> {
    let rendered = render_template(&input, config);
    if rendered.is_ok() {
        return Ok(rendered.unwrap());
    }
    return Err(get_errors(rendered));
}
