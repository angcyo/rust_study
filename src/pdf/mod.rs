use pdf2image::{RenderOptionsBuilder, PDF};
//use pdfium_render::prelude::{PdfRenderConfig, Pdfium, PdfiumError};

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/26
///

/// `pdfium_render` 将pdf文件, 转换成图片保存
/*fn export_pdf_to_jpegs(pdf_path: &str, output_path: &str) -> anyhow::Result<()> {
    crate::utils::ensure_dir_exist(output_path);

    // Renders each page in the PDF file at the given path to a separate JPEG file.

    // Bind to a Pdfium library in the same directory as our Rust executable.
    // See the "Dynamic linking" section below.

    let pdfium = Pdfium::default();

    // Load the document from the given path...

    let document = pdfium.load_pdf_from_file(pdf_path, None)?;

    // ... set rendering options that will be applied to all pages...

    let render_config = PdfRenderConfig::new()
        //.set_target_width(2000)
        //.set_maximum_height(2000)
        /*.rotate_if_landscape(PdfPageRenderRotation::Degrees90, true)*/;

    // ... then render each page to a bitmap image, saving each image to a JPEG file.

    for (index, page) in document.pages().iter().enumerate() {
        page.render_with_config(&render_config)?
            .as_image() // Renders this page to an image::DynamicImage...
            .into_rgb8() // ... then converts it to an image::Image...
            .save_with_format(
                format!("{}/pdf-page-{}.jpg", output_path, index),
                image::ImageFormat::Jpeg,
            ) // ... and saves it to a file.
            .map_err(|_| PdfiumError::ImageError)?;
    }

    Ok(())
}*/

/// `pdf2image` 将pdf文件, 转换成图片保存
fn export_pdf_to_jpegs(pdf_path: &str, output_path: &str) -> anyhow::Result<()> {
    crate::utils::ensure_dir_exist(output_path);
    
    let pdf = PDF::from_file(pdf_path)?;
    let pages = pdf.render(
        pdf2image::Pages::Range(1..=8),
        RenderOptionsBuilder::default().pdftocairo(true).build()?,
    )?;

    for (i, page) in pages.iter().enumerate() {
        page.save_with_format(
            format!("{output_path}/pdf-page-{}.jpg", i + 1),
            image::ImageFormat::Jpeg,
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{pdf, ptl};

    #[test]
    fn test_export_pdf_to_jpegs() {
        let pdf_path = "/Users/angcyo/Pictures/LaserABC/易雕科技-营业执照.pdf";
        let output_path = "/Users/angcyo/Pictures/LaserABC/";
        pdf::export_pdf_to_jpegs(pdf_path, output_path).unwrap();
        ptl!("->{output_path}");
    }
}
