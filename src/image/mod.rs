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
