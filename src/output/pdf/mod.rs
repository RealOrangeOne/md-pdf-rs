use config::Config;
use utils::result_override;

use std::error::Error;

use wkhtmltopdf::{PdfApplication, Result as WKResult, PageSize, PdfBuilder};

fn pdf_result<T>(res: WKResult<T>) -> Result<T, String> {
    return match res {
        Ok(out) => Ok(out),
        Err(err) => Err(err.description().into()),
    };
}


fn create_builder<'a>(config: Config, builder: &'a mut PdfBuilder) -> &'a mut PdfBuilder {
    let mut safe_builder = builder.page_size(PageSize::A4).image_quality(100).title(&config.title);
    unsafe {
        return safe_builder
            .global_setting("useCompression", "true")
            .object_setting("useLocalLinks", "true")
            .object_setting("useExternalLinks", "true")
            .object_setting("pagesCount", "true");
    }
}

pub fn output(config: Config, html: String) -> Result<(), String> {
    let output_location = &config.absolute_output("pdf".into());
    let mut pdf_app =
        try!(result_override(PdfApplication::new(), "Failed to create PDF Application".into()));
    let mut base_builder = pdf_app.builder();
    let builder = create_builder(config.clone(), &mut base_builder);

    let mut pdfout = try!(pdf_result(builder.build_from_html(&html)));
    let output_file = try!(result_override(
        pdfout.save(output_location),
        format!("Failed to output PDF file to {}", output_location.display())
    ));
    debug_assert!(output_file.metadata().unwrap().is_file());
    return Ok(());
}
