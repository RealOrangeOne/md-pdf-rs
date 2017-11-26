use html::{sciter_start, get_html, get_head, find_first};
use sciter::Element;
use config::Config;
use assets;

fn create_css_element(style: String) -> Element {
    let mut style_tag = Element::with_text("style", &style).unwrap();
    style_tag.set_attribute("type", "text/css");
    style_tag.set_attribute("media", "all");
    return style_tag;
}


pub fn static_files(_: Config, input: String) -> Result<String, String> {
    let mut root = sciter_start(input);
    let mut head = get_head(&mut root);
    head.append(&create_css_element(assets::get("style.css")));
    return Ok(get_html(root));
}
