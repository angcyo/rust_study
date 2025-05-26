#[allow(dead_code)]
use image::{DynamicImage, ImageError, ImageReader};

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/23
///
/// 从文件中读取图片数据
pub fn read_image_file(image_file_path: &str) -> Result<DynamicImage, ImageError> {
    ImageReader::open(image_file_path)?.decode()
}

/// 从字节数据中读取图片
pub fn read_image_bytes(image_bytes: &[u8]) -> Result<DynamicImage, ImageError> {
    image::load_from_memory(image_bytes)
    //ImageReader::new(image_bytes)?.decode()
}

/// 调整图片大小
pub fn resize_image(image: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    image.resize(width, height, image::imageops::FilterType::Nearest)
}

/// 将图片对象转换成base64字符串数据
pub fn image_to_base64(img: &DynamicImage) -> anyhow::Result<String> {
    // 直接写入 Vec<u8>
    //let mut buffer = Vec::new();
    let mut buffer = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buffer, image::ImageFormat::Png)?;

    // 编码为 Base64
    let base64_string = crate::utils::base64_encode(&buffer.into_inner());
    Ok(base64_string)
}

/// 调整指定路径图片的大小, 并输出
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

/// 从文件中读取图片,并输出对应的base64协议图片数据
pub fn read_image_file_to_base64(image_file_path: &str) -> anyhow::Result<String> {
    if let Ok(image) = read_image_file(image_file_path) {
        //let image_bytes = image.into_bytes(); //颜色字节数组
        //let base64_data = crate::utils::base64_encode(&image_bytes);
        //Ok(format!("data:image/jpeg;base64,{base64_data}"))
        let base64_data = image_to_base64(&image)?;
        Ok(format!("data:image/png;base64,{base64_data}"))
    } else {
        Err(anyhow::anyhow!("Failed to read image"))
    }
}

///将base64图片转换成图片
pub fn base64_to_image(base64_data: &str) -> anyhow::Result<DynamicImage> {
    let base64_data = if base64_data.contains(",") {
        base64_data.split(',').last().unwrap()
    } else {
        base64_data
    };
    let bytes = crate::utils::base64_decode(base64_data)?;
    Ok(read_image_bytes(bytes.as_slice())?)
}

#[cfg(test)]
mod tests {
    use crate::utils::bytes_to_string;

    #[test]
    fn test_image_to_base64() {
        //let image_path = "tests/Looks Good To Me.png";
        let image_path = "tests/img.png";
        let output_path = ".output/img_base64.txt";
        let base64 = crate::image::read_image_file_to_base64(image_path).unwrap();
        crate::utils::file_utils::save_string_to_file(output_path, base64.as_str()).unwrap();

        let output_path2 = ".output/img_base64.png";
        let bytes = crate::utils::file_utils::read_file_bytes(output_path).unwrap();
        crate::image::base64_to_image(bytes_to_string(&bytes).as_str())
            .unwrap()
            .save(output_path2)
            .unwrap();
    }
}
