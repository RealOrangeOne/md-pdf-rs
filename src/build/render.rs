use config::Config;


fn stub(config: Config, input: String) -> Result<String, String> {
    return Ok(input);
}

pub fn render(config: Config, input: String) -> Result<String, String> {
    let mut rendered_input = input;
    for renderer in vec![
        stub
    ] {
        rendered_input = try!(renderer(config.clone(), rendered_input));
    }
    return Ok(rendered_input);
}
