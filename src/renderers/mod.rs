use config::Config;

pub mod sciter;

pub fn stub(config: Config, input: String) -> Result<String, String> {
    let root = sciter::sciter_start(input);
    let html = sciter::get_html(root);
    return Ok(html);
}

