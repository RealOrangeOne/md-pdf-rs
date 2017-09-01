use config::Config;

use sciter::{Window, Host, Element};
use std::rc::Rc;
use std::ops::Deref;

pub mod html_cleanup;
pub mod strip_blank;


fn get_root(frame: Window) -> Element {
    return frame.get_host().deref().get_root().expect("Failed to get root of window");
}

fn sciter_start(source: String) -> Element {
    let mut frame = Window::new();
    frame.load_html(&source.as_bytes(), None);
    return get_root(frame);
}

fn get_html(element: Element) -> String {
    return String::from_utf8(element.get_html(true)).expect(&format!(
        "Failed to get HTML from {}.",
        element.get_tag()
    ));
}

fn find_first(root: &mut Element, selector: &str) -> Element {
    return root.find_first(selector).expect(&format!("Failed to get {}.", selector)).expect(
        &format!(
            "Couldn't find any {}.",
            selector
        )
    );
}

fn destroy_matching(root: &mut Element, selector: &str) {
    let mut matches = root.find_all(selector).expect(&format!("Failed to get {}.", selector));
    if matches.is_none() {
        return;
    }

    for mut ele in matches.unwrap() {
        ele.destroy().expect("Failed to delete");
    }
}
