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
    return String::from_utf8(element.get_html(true)).expect("Failed to get HTML from element");
}
