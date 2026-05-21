///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2026-5-21
///

/// 打印当前工作路径
pub fn println_current_dir() {
    println!("当前工作路径->{:?}", std::env::current_dir().unwrap());
}

/// 读取测试文件数据
pub fn read_test_file_bytes(test_file_name: &str, from_output: bool) -> Vec<u8> {
    let file_name = test_file_name;
    //文件路径拼接
    let mut file_path = std::env::current_dir().unwrap().join("..").join("tests");
    if from_output {
        file_path = file_path.join(".output")
    }
    file_path = file_path.join(file_name);
    use std::fs;
    let bytes = fs::read(&file_path).unwrap();
    println!("读取文件->{:?} {:?}B", &file_path, bytes.len());
    bytes
}

/// 将数据写入到测试输出文件
pub fn write_test_file_bytes(output_file_name: &str, bytes: &Vec<u8>) {
    let file_name = output_file_name;
    //文件路径拼接
    let file_path = std::env::current_dir()
        .unwrap()
        .join("..")
        .join("tests")
        .join(".output")
        .join(file_name);
    use std::fs;
    //确保文件夹存在
    fs::create_dir_all(file_path.parent().unwrap()).unwrap();
    fs::write(&file_path, bytes).unwrap();
    println!("写入文件->{:?} {:?}B", &file_path, bytes.len());
}

/// 将一个闭包包裹在测量耗时的方法中
pub fn measure_time<R, F>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let start = std::time::Instant::now();
    let result = f();
    println!("耗时: {:?}", start.elapsed());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println_current_dir()
    }
}
