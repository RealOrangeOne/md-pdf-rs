use input::read_input_files;
use config::Config;
use build::build_input;


pub fn build(config: Config) -> Result<(), String> {
    let input = try!(read_input_files(config.input.clone()));
    println!("{}", input);
    build_input(config.clone(), input);
    return Ok(());
}
