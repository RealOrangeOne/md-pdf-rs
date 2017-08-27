use config::Config;
use pandoc::{self, Pandoc, PandocOutput, PandocError};
use std::env::{current_dir, current_exe};
use std::path::PathBuf;
use std::error::Error;

fn add_path_hints(pandoc: &mut Pandoc, path: &mut PathBuf) {
    let mut out = true;
    while out {
        pandoc.add_pandoc_path_hint(path);
        out = path.pop();
    }
}


fn execute_pandoc(config: Config, input: String) -> Result<PandocOutput, PandocError> {
    let mut renderer = Pandoc::new();
    renderer.set_output_format(pandoc::OutputFormat::Html5, vec![]);
    renderer.set_input_format(pandoc::InputFormat::Markdown, vec![]);
    renderer.set_input(pandoc::InputKind::Pipe(input));
    renderer.set_output(pandoc::OutputKind::Pipe);
    renderer.add_option(pandoc::PandocOption::Smart);
    renderer.add_option(pandoc::PandocOption::Standalone);
    add_path_hints(&mut renderer, &mut current_exe().unwrap());
    add_path_hints(&mut renderer, &mut current_dir().unwrap());
    return renderer.execute();
}


pub fn render(config: Config, input: String) -> Result<String, String> {
    let output = execute_pandoc(config, input);
    if output.is_err() {
        return Err(output.err().unwrap().description().into());
    }
    return match output.unwrap() {
        pandoc::PandocOutput::ToBuffer(out) => Ok(out),
        _ => Err("Incorrect output type.".into()),
    };
}
