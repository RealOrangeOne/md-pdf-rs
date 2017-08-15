use config::Config;
use pandoc::{Pandoc, OutputFormat, InputFormat, InputKind, OutputKind, PandocOutput, PandocError,
             PandocOption};
use std::env::{current_dir, current_exe};
use std::path::PathBuf;
use std::error::Error;

fn execute_pandoc(config: Config, input: String) -> Result<PandocOutput, PandocError> {
    let mut pandoc = Pandoc::new();
    pandoc.add_pandoc_path_hint(&current_dir().unwrap().join("lib"));
    pandoc.add_pandoc_path_hint(&current_exe().unwrap().parent().unwrap().join("lib"));
    pandoc.set_output_format(OutputFormat::Html5, vec![]);
    pandoc.set_input_format(InputFormat::Markdown, vec![]);
    pandoc.set_input(InputKind::Pipe(input));
    pandoc.set_output(OutputKind::Pipe);
    pandoc.add_option(PandocOption::Smart);
    pandoc.add_option(PandocOption::Standalone);
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
