use image::{DynamicImage, ImageError, ImageReader};

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/23
///
/// 从文件中读取图片数据
#[allow(dead_code)]
pub fn read_image_file(image_file_path: &str) -> Result<DynamicImage, ImageError> {
    ImageReader::open(image_file_path)?.decode()
}

/// 调整图片大小
#[allow(dead_code)]
pub fn resize_image(image: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    image.resize(width, height, image::imageops::FilterType::Nearest)
}

/// 调整指定路径图片的大小, 并输出
#[allow(dead_code)]
pub fn resize_image_file(
    image_file_path: &str,
    width: u32,
    height: u32,
    output_file_path: &str,
) -> Result<(), ImageError> {
    if let Ok(image) = read_image_file(image_file_path) {
        let resized_image = resize_image(&image, width, height);
        resized_image.save(output_file_path)
    } else {
        Err(ImageError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to read image",
        )))
    }
}
