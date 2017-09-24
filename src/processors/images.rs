use html::{sciter_start, get_html, get_body, find_all};
use sciter::{Element, Value};
use config::Config;
use utils::{resolve_path, path_to_string};


pub fn images(config: Config, input: String) -> Result<String, String> {
    let mut root = sciter_start(input);
    let mut body = get_body(&mut root);
    let mut images = find_all(&mut body, "img[src]");
    for mut image in images {
        let image_src = image.get_attribute("src").expect("Image doesn't have a src");
        let image_path = resolve_path(image_src);
        if image_path.exists() && image_path.is_file() {
            image.set_attribute("src", path_to_string(&image_path.canonicalize().unwrap()));
        }
    }
    return Ok(get_html(root));
}
