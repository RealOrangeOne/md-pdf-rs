use html::{sciter_start, get_html, get_head, find_first};
use config::Config;


pub fn rebrand(config: Config, input: String) -> Result<String, String> {
    let mut root = sciter_start(input);
    let mut head = get_head(&mut root);
    let mut ele = find_first(&mut root, "meta[name='generator']");
    ele.set_attribute("content", crate_name!());
    return Ok(get_html(root));
}
