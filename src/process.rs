use input::read_input_files;
use config::Config;
use build::build_input;


pub fn build(config: Config) -> Result<(), String> {
    let input = try!(read_input_files(config.input.clone()));
    let raw_html = try!(build_input(config.clone(), input));
    println!("{}", raw_html);
    return Ok(());
}
