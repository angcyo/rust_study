use std::fmt::format;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2026-5-27
///
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let input_path = args.get(1).expect("请输入文件路径");

    //使用utf-8 读取文件数据
    let text = std::fs::read_to_string(input_path).unwrap();
    //使用空格分割, 将HEX数据转换成二进制数据
    let bytes: Vec<u8> = text
        .split_whitespace()
        .map(|s| u8::from_str_radix(s, 16).unwrap())
        .collect();

    let mut output_path = input_path.to_string();
    output_path.push_str(".bin");
    std::fs::write(&output_path, &bytes).unwrap();

    println!("转换完成[{}B]->{}", bytes.len(), output_path);
}
