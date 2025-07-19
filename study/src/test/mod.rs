///
/// @author <a href="mailto: angcyo@126.com">angcyo</a> \
/// @date 2025/07/19
///
/// 测试相关代码

/// 通过[file_name]文件名, 用来获取一个测试文件
/// @return 文件全路径
pub fn get_test_file_path(file_name: &str) -> String {
    let file_path = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("tests")
        .join(file_name)
        .to_str()
        .unwrap()
        .to_string();
    rc_basis::files::ensure_parent_dir_exist(file_path.as_str());
    file_path.to_string()
}

/// 通过[file_name]文件名, 用来获取一个测试输出文件
/// @return 文件全路径
pub fn get_test_output_file_path(file_name: &str) -> String {
    let file_path = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join(".output")
        .join(file_name)
        .to_str()
        .unwrap()
        .to_string();
    rc_basis::files::ensure_parent_dir_exist(file_path.as_str());
    file_path.to_string()
}

/// 保存和打开输出的文件
pub fn save_and_open_file(file_name: &str, bytes: &[u8]) {
    let file_path = get_test_output_file_path(file_name);
    rc_basis::files::save_bytes_to_file(file_path.as_str(), bytes).unwrap();
    rc_basis::files::open_file_with_sys(&file_path);

    println!("保存在->{}", file_path)
}
