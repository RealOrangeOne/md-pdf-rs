use renderers::helpers::{sciter_start, get_html};
use config::Config;


pub fn html_cleanup(config: Config, input: String) -> Result<String, String> {
    let mut root = sciter_start(input);
    root.find_first("meta[content='text/css']").unwrap().unwrap().destroy().unwrap();
    root.find_first("style").unwrap().unwrap().destroy().unwrap();
    root.find_first("title").unwrap().unwrap().destroy().unwrap();
    let html = get_html(root);
    return Ok(html);
}
