use std::char;
use config::Config;

fn remove_blank_lines(line: &&str) -> bool {
    if line.is_empty() {
        return false;
    }
    let whitespace_chars: Vec<&str> = line.matches(char::is_whitespace).collect();
    if whitespace_chars.len() == line.len() {
        return false;
    }
    return true;
}

pub fn strip_blank(config: Config, input: String) -> Result<String, String> {
    let split: Vec<&str> = input.lines().filter(remove_blank_lines).collect();
    return Ok(split.join("\n"));
}
