use renderers::{sciter_start, get_html, destroy_matching};
use config::Config;


pub fn html_cleanup(config: Config, input: String) -> Result<String, String> {
    let mut root = sciter_start(input);
    destroy_matching(&mut root, "meta[content='text/css']");
    destroy_matching(&mut root, "style");
    destroy_matching(&mut root, "title");
    let html = get_html(root);
    return Ok(html);
}
