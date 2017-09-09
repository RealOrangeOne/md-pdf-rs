use zip::ZipArchive;
use mktemp::Temp;
use std::path::PathBuf;
use utils::get_exe_dir;
use std::fs::File;
use std::io::{Read, Write};


fn get_temp_file() -> PathBuf {
    return Temp::new_file().expect("Failed to create temporary file").to_path_buf();
}

pub fn unpack_csl(csl_name: String) -> PathBuf {
    let file = get_temp_file();
    let zip_file = File::open(get_exe_dir().join("csl.zip")).expect("Failed to read CSL zip");
    let mut archive = ZipArchive::new(zip_file).expect("Failed to load zip file");
    debug_assert!(archive.len() >= 10);
    let mut csl_zip_file = archive.by_name(&format!("{}.csl", csl_name)).expect(&format!(
        "Failed to find CSL {}.",
        csl_name
    ));
    debug_assert!(csl_zip_file.size() > 0);
    let mut csl_temp = File::create(&file).expect("Failed to open temporary file");
    let mut csl_buffer = String::new();
    csl_zip_file.read_to_string(&mut csl_buffer).expect("Failed to read CSL");
    csl_temp.write_all(csl_buffer.as_bytes()).expect("Failed to write CSL to temporary file");
    return file;
}
