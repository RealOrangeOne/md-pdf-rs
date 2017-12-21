use html::{sciter_start, get_html, get_head};
use sciter::Element;
use config::Config;
use assets;

fn create_css_element(style: String) -> Element {
    let mut style_tag = Element::with_text("style", &style).unwrap();
    style_tag.set_attribute("type", "text/css").expect(&format!("Failed to set CSS mimetype for {}", style));
    style_tag.set_attribute("media", "all").expect(&format!("Failed to set CSS media type for {}", style));
    return style_tag;
}


pub fn static_files(_: Config, input: String) -> Result<String, String> {
    let mut root = sciter_start(input);
    let mut head = get_head(&mut root);
    head.append(&create_css_element(assets::get("style.css"))).expect("Failed to inject CSS file");
    return Ok(get_html(root));
}
