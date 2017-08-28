use config::Config;
use utils::result_override;

use std::error::Error;

use wkhtmltopdf::{PdfApplication, Result as WKResult, PageSize};

fn pdf_result<T>(res: WKResult<T>) -> Result<T, String> {
    return match res {
        Ok(out) => Ok(out),
        Err(err) => Err(err.description().into()),
    };
}

pub fn output(config: Config, html: String) -> Result<(), String> {
    let output_location = &config.output["pdf"];
    let mut pdf_app =
        try!(result_override(PdfApplication::new(), "Failed to create PDF Application".into()));
    let pdfout_result = pdf_app
        .builder()
        .page_size(PageSize::A4)
        .image_quality(100)
        .title(&config.title)
        .build_from_html(&html);

    let mut pdfout = try!(pdf_result(pdfout_result));
    let output_file = try!(result_override(
        pdfout.save(output_location),
        format!("Failed to output PDF file to {}", output_location.display())
    ));
    debug_assert!(output_file.metadata().unwrap().is_file());
    return Ok(());
}
