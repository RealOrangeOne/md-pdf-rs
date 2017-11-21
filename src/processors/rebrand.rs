use html::{sciter_start, get_html, get_head, find_first};
use config::Config;


pub fn rebrand(_: Config, input: String) -> Result<String, String> {
    let mut root = sciter_start(input);
    let mut head = get_head(&mut root);
    let mut ele = find_first(&mut head, "meta[name='generator']");
    ele.set_attribute("content", crate_name!()).expect("Failed to set generator.");
    return Ok(get_html(root));
}
