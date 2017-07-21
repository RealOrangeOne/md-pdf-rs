use input::read_input_files;
use config::Config;


pub fn build(config: Config) {
    let input = read_input_files(config.input).unwrap();
    println!("{}", input);
}
