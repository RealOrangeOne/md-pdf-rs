use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use utils::result_override;


pub fn read_input_files(files: Vec<PathBuf>) -> Result<String, String> {
    let mut input = String::new();
    for input_file_path in files.iter() {
        let mut input_file = try!(result_override(
            File::open(input_file_path),
            format!("Failed to open input file {}.", input_file_path.display())
        ));
        try!(result_override(
            input_file.read_to_string(&mut input),
            format!("Failed to read input file {}.", input_file_path.display())
        ));
        input.push('\n');
    }
    return Ok(input);
}
