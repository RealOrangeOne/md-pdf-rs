use std::fs::File;
use std::io::Read;
use std::path::PathBuf;


pub fn read_input_files(files: Vec<PathBuf>) -> String {
    let mut input = String::new();
    for input_file_path in files.iter() {
        let mut input_file = File::open(input_file_path).expect("Unable to open file");
        input_file.read_to_string(&mut input).expect("Failed to read file");
    }
    return input;
}
