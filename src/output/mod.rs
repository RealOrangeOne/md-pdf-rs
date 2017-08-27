use wkhtmltopdf::PdfApplication;


pub fn output(html: String) {
    let mut pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app.builder().build_from_html(&html).expect("failed to build pdf");

    pdfout.save("foo.pdf").expect("failed to save foo.pdf");
    println!("generated PDF saved as: foo.pdf");
}
