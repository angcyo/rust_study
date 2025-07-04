use clap_builder::Parser;
use std::io::Write;

mod args;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025-07-04
///
fn main() {
    let args = args::Args::parse();

    let size = args.size;

    let bytes = match args.input {
        Some(input) => std::fs::read(&input).expect(format!("无法打开文件: {}", &input).as_str()),
        None => vec![0; 1],
    };

    //输出文件全路径
    let output = match args.output {
        Some(output) => output,
        None => format!("{}", size),
    };

    println!("准备生成文件{} B大小的数据...", size);
    write_file_fill_data(bytes, size, &output);

    //获取全路径
    let output = std::fs::canonicalize(&output).unwrap();
    println!("生成完毕, 文件保存在->{}", output.to_str().unwrap());
}

//--

/// 向文件中写入指定字节大小的数据
fn write_file_fill_data(fill_data: Vec<u8>, size: u64, output: &str) {
    let mut file =
        std::fs::File::create(output).expect(format!("无法创建文件: {}", output).as_str());

    //已写入的字节大小
    let mut write_count = 0;

    while write_count < size {
        file.write_all(&fill_data)
            .expect(format!("写入数据失败: {}", output).as_str());
        write_count += fill_data.len() as u64;
    }
}
