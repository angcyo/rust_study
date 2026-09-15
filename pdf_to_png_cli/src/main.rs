use crate::args::Args;
use clap_builder::Parser;
use pdfrs::pdf::PdfDocument;
use pdfrs::raster;

mod args;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2026-9-15
///
/// 将`pdf`导出成`png`图片
fn main() {
    let args = Args::parse();
    export_pdf_to_png(args.input.as_str(), args.output.as_deref(), args.dpi).unwrap();
}

/// 导出pdf为png
/// - [input] pdf文件路径
/// - [output] 输出文件夹路径
fn export_pdf_to_png(
    input: &str,
    output: Option<&str>,
    dpi: Option<u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. 读取 PDF 文件为字节数组 (Buffer)
    let bytes = std::fs::read(input)?;
    let output_path = output.unwrap_or(
        std::path::Path::new(input)
            .parent()
            .unwrap()
            .to_str()
            .unwrap(),
    );
    match raster::rasterize_all(&bytes, dpi.unwrap_or(96)) {
        Ok(pages) => {
            let out_path = std::path::Path::new(output_path);
            if pages.len() == 1 {
                let file = out_path.join(format!("page-{:04}.png", 1));
                let file_path = file.to_str().unwrap_or_default();
                match pages[0].write_png(file_path) {
                    Ok(_) => println!("[{}/{}] 保存至-> {}", 1, pages.len(), file_path),
                    Err(e) => eprintln!("Error writing PNG: {}", e),
                }
            } else {
                std::fs::create_dir_all(out_path).ok();
                for (i, p) in pages.iter().enumerate() {
                    let file = out_path.join(format!("page-{:04}.png", i + 1));
                    let file_path = file.to_str().unwrap_or_default();
                    match p.write_png(file_path) {
                        Ok(_) => println!("[{}/{}] 保存至-> {}", i + 1, pages.len(), file_path),
                        Err(e) => eprintln!("Error writing page {}: {}", i + 1, e),
                    }
                }
                println!("Wrote {} page(s) to {}", pages.len(), output_path);
            }
        }
        Err(e) => eprintln!("Error rasterizing PDF: {}", e),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_pdf_to_png() {
        let input = r"E:\易雕科技\KopLaser\company.pdf";
        //let input = r"E:\易雕科技\KopLaser\S36C-626091417540.pdf";
        //let input = r"E:\易雕科技\KopLaser\计算机软件著作权登记申请确认签章页.pdf";
        export_pdf_to_png(input, None, None).unwrap();
    }
}
