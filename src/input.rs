use std::fs::File;
use std::io::Read;
use std::path::PathBuf;


pub fn read_input_files(files: Vec<PathBuf>) -> Result<String, String> {
    let mut input = String::new();
    for input_file_path in files.iter() {
        let input_file_result = File::open(input_file_path);
        if input_file_result.is_err() {
            return Err(format!("Failed to open input file {}.", input_file_path.display()));
        }
        if input_file_result.unwrap().read_to_string(&mut input).is_err() {
            return Err(format!("Failed to read input file {}.", input_file_path.display()));
        }
    }
    return Ok(input);
}
