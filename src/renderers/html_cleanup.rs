use renderers::{sciter_start, get_html, destroy_matching, destroy_at, find_first};
use config::Config;
use sciter::{Window};
use std::ops::Deref;
use std::rc::Rc;


pub fn html_cleanup(config: Config, input: String) -> Result<String, String> {
    let mut root = sciter_start(input);
    let mut head = find_first(&mut root, "head");
    destroy_at(&mut head, 4);  // Sciter doesnt like finding this style tag for some reason
    destroy_matching(&mut head, "meta[content='text/css']");
    destroy_matching(&mut head, "style");
    destroy_matching(&mut head, "title");
    return Ok(get_html(root));
}
