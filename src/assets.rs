#[allow(dead_code)]
mod included_assets {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}


pub fn get(path: &str) -> String {
    let file = included_assets::SRC.find(path).expect(&format!("Can't find asset at {}.", path));
    return String::from_utf8(file.contents.to_vec()).expect(&format!("Failed to parse {}", path));
}
