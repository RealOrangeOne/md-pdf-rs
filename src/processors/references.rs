use html::{sciter_start, get_html, get_body};
use sciter::Element;
use config::Config;


fn create_title() -> Element {
    let mut references_title = Element::with_text("h1", "References").unwrap();
    references_title.set_attribute("class", "references-title").unwrap();
    return references_title;
}


pub fn references(config: Config, input: String) -> Result<String, String> {
    let mut root = sciter_start(input);
    let mut body = get_body(&mut root);
    let possible_references = body.find_first("div#refs").expect("Failed to get refs");
    if possible_references.is_some() {
        body.insert(possible_references.unwrap().index(), &create_title()).expect(
            "Failed to add references title"
        );
    }
    return Ok(get_html(root));
}
