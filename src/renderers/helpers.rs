use sciter::{Window, Host, Element};
use std::rc::Rc;
use std::ops::Deref;


fn get_root(frame: Window) -> Element {
    return frame.get_host().deref().get_root().expect("Failed to get root of window");
}

pub fn sciter_start(source: String) -> Element {
    let mut frame = Window::new();
    frame.load_html(&source.as_bytes(), None);
    return get_root(frame);
}

pub fn get_html(element: Element) -> String {
    return String::from_utf8(element.get_html(true)).expect("Failed to get HTML from element");
}
