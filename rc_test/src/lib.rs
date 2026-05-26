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

    /// 读取指定文件中的 Hex 数据, 转成二进制数据存储到新文件
    #[test]
    fn test_read_file_to_bytes() {
        println_current_dir();
        let input_path = "E:/log/mcu_300kb.log";
        let output_path = "E:/log/mcu_300kb.log.bin";
        //使用utf-8 读取文件数据
        let text = std::fs::read_to_string(input_path).unwrap();
        //使用空格分割, 将HEX数据转换成二进制数据
        let bytes: Vec<u8> = text
            .split_whitespace()
            .map(|s| u8::from_str_radix(s, 16).unwrap())
            .collect();
        write_test_file_bytes(output_path, &bytes);
    }

    /// 对比2个文件的数据是否一致
    #[test]
    fn test_compare_file() {
        println_current_dir();
        let path1 = "E:/projects/rust/rust_study/tests/image 1-image-stucki-1779331603635_300kb.ydd";
        let path2 = "E:/projects/rust/rust_study/tests/.output/mcu_300kb.log";
        assert_eq!(
            read_test_file_bytes(path1, false),
            read_test_file_bytes(path2, false)
        );
    }
}
