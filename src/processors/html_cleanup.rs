use html::{sciter_start, get_html, destroy_matching, destroy_at, get_head};
use config::Config;


pub fn html_cleanup(config: Config, input: String) -> Result<String, String> {
    let mut root = sciter_start(input);
    let mut head = get_head(&mut root);
    destroy_at(&mut head, 4); // Sciter doesnt like finding this style tag for some reason
    destroy_matching(&mut head, "meta[content='text/css']");
    destroy_matching(&mut head, "style");
    destroy_matching(&mut head, "title");
    return Ok(get_html(root));
}
