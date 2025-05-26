use std::fs::File;
use std::io::{Read, Write};

#[allow(dead_code)]
///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/26
///
/// 将字符串保存到文件
pub fn save_string_to_file(file_path: &str, content: &str) -> anyhow::Result<()> {
    crate::utils::ensure_parent_dir_exist(file_path);
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// 从文件中读取字节数据
pub fn read_file_bytes(file_path: &str) -> anyhow::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
