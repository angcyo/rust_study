use rc_basis::files::ensure_dir_exist;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025-11-07
///

/// 获取一个测试文件路径
/// ```
/// rust_study/tests/xxx
/// ```
fn test_file_path(name: &str) -> String {
    let mut path = std::env::current_dir().unwrap();
    path.push("..");
    path.push("..");
    path.push("tests");
    ensure_dir_exist(path.to_str().unwrap());
    path.push(name);
    path.canonicalize()
        .unwrap_or(path)
        .to_str()
        .unwrap()
        .to_string()
}

/// 获取一个输出文件路径
/// ```
/// rust_study/.output/xxx
/// ```
fn output_file_path(name: &str) -> String {
    let mut path = std::env::current_dir().unwrap();
    path.push("..");
    path.push("..");
    path.push(".output");
    ensure_dir_exist(path.to_str().unwrap());
    path.push(name);
    path.canonicalize()
        .unwrap_or(path)
        .to_str()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rc_basis::ptl_current_dir;

    #[test]
    fn it_works() {
        println!("{}", test_file_path("蒙古之花-剪辑.ogg"));
        println!("{}", output_file_path("test.txt"));
    }

    /// rust_study/studies/audio
    #[test]
    fn test() {
        ptl_current_dir!();
    }
}
