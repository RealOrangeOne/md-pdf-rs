use config::Config;
use pandoc::{Pandoc, OutputFormat, InputFormat, InputKind, OutputKind, PandocOutput, PandocError,
             PandocOption};
use std::env::{current_dir, current_exe};
use std::path::PathBuf;
use std::error::Error;

fn add_path_hints(pandoc: &mut Pandoc, path: &mut PathBuf) {
    let mut out = true;
    while out {
        pandoc.add_pandoc_path_hint(&path.join("lib"));
        out = path.pop();
    }
}


fn execute_pandoc(config: Config, input: String) -> Result<PandocOutput, PandocError> {
    let mut pandoc = Pandoc::new();
    pandoc.set_output_format(OutputFormat::Html5, vec![]);
    pandoc.set_input_format(InputFormat::Markdown, vec![]);
    pandoc.set_input(InputKind::Pipe(input));
    pandoc.set_output(OutputKind::Pipe);
    pandoc.add_option(PandocOption::Smart);
    pandoc.add_option(PandocOption::Standalone);
    add_path_hints(&mut pandoc, &mut current_exe().unwrap());
    add_path_hints(&mut pandoc, &mut current_dir().unwrap());
    return pandoc.execute();
}


pub fn render(config: Config, input: String) -> Result<String, String> {
    let output = execute_pandoc(config, input);
    if output.is_err() {
        return Err(output.err().unwrap().description().into());
    }
    return match output.unwrap() {
        PandocOutput::ToBuffer(out) => Ok(out),
        _ => Err("Incorrect output type.".into()),
    };
}
