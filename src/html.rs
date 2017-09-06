use sciter::{Window, Element};
use std::ops::Deref;


pub fn get_root(frame: Window) -> Element {
    return frame.get_host().deref().get_root().expect("Failed to get root of window");
}

pub fn sciter_start(source: String) -> Element {
    let mut frame = Window::new();
    frame.load_html(&source.as_bytes(), None);
    return get_root(frame);
}

pub fn get_html(element: Element) -> String {
    element.update(true).expect("Failed to update");
    return String::from_utf8(element.get_html(true)).expect(&format!(
        "Failed to get HTML from {}.",
        element.get_tag()
    ));
}

pub fn find_all(root: &mut Element, selector: &str) -> Vec<Element> {
    let elements = root.find_all(selector).expect(&format!("Failed to get {}.", selector));
    if elements.is_none() {
        return Vec::new();
    }
    return elements.unwrap();
}

pub fn find_first(root: &mut Element, selector: &str) -> Element {
    let mut all_matches = find_all(root, selector);
    all_matches.reverse();
    return all_matches.pop().expect(&format!("Failed to find {}.", selector));
}

pub fn destroy_at(root: &mut Element, index: usize) {
    let mut ele = root.get(index).expect(&format!("Failed to get element at {}.", index));
    ele.destroy().expect("Failed to delete.");
}

pub fn destroy_matching(root: &mut Element, selector: &str) {
    let matches = find_all(root, selector);
    if matches.is_empty() {
        return;
    }
    for mut ele in matches {
        ele.destroy().expect("Failed to delete");
    }
}

pub fn get_head(root: &mut Element) -> Element {
    return find_first(root, "head");
}

pub fn get_body(root: &mut Element) -> Element {
    return find_first(root, "body");
}
