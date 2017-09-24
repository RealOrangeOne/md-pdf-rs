use html::{sciter_start, get_html, destroy_matching, destroy_at, get_head};
use sciter::Element;
use config::Config;


pub fn head_cleanup(config: Config, input: String) -> Result<String, String> {
    let mut root = sciter_start(input);
    let mut head = get_head(&mut root);
    destroy_matching(&mut head, "meta[content='text/css']");
    destroy_matching(&mut head, "style");
    destroy_matching(&mut head, "title");
    let mut meta_charset = Element::create_at("meta", &mut head).unwrap();
    meta_charset.set_attribute("charset", "UTF-8");
    return Ok(get_html(root));
}
