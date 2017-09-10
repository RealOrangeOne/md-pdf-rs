use zip::ZipArchive;
use mktemp::Temp;
use std::path::PathBuf;
use utils::{get_exe_dir, result_override};
use std::fs::File;
use std::io::{Read, Write};
use config::consts::CSL_FILE_NAME;

fn get_csl_path() -> PathBuf {
    return get_exe_dir().join(CSL_FILE_NAME);
}


fn get_temp_file() -> PathBuf {
    return Temp::new_file().expect("Failed to create temporary file").to_path_buf();
}


fn get_csl_data(csl_name: String) -> Result<String, String> {
    let zip_file = try!(result_override(File::open(get_csl_path()), "Failed to read CSL zip".into()));
    let mut archive = try!(result_override(ZipArchive::new(zip_file), "Failed to load zip file".into()));
    debug_assert!(archive.len() >= 10);
    let mut csl_zip_file = try!(result_override(archive.by_name(&format!("{}.csl", csl_name)), format!("Can't find CSL {}.", csl_name)));
    debug_assert!(csl_zip_file.size() > 0);
    let mut csl_buffer = String::new();
    try!(result_override(csl_zip_file.read_to_string(&mut csl_buffer), "Failed to read CSL".into()));
    return Ok(csl_buffer);
}

pub fn is_valid_csl(csl_name: String) -> bool {
    return get_csl_data(csl_name).is_ok();
}


pub fn unpack_csl(csl_name: String) -> PathBuf {
    let file = get_temp_file();
    let mut csl_temp = File::create(&file).expect("Failed to open temporary file");
    let mut csl_buffer = get_csl_data(csl_name).unwrap();
    csl_temp.write_all(csl_buffer.as_bytes()).expect("Failed to write CSL to temporary file");
    return file;
}
