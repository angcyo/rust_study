use crate::ydd::gcode_to_ydd_bytes;
use clap_builder::Parser;
use rc_basis::files::save_bytes_to_file;
use std::path::Path;

mod args;
mod ydd;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/03
///
fn main() {
    let args = args::Args::parse();
    //--
    if (args.debug) {
        println!("当前工作路径->{:?}", std::env::current_dir().unwrap());
        println!(
            "输入文件路径->{:?}",
            std::fs::canonicalize(&args.input).unwrap()
        );
    }
    //输出文件全路径
    let output = match args.output {
        Some(output) => output,
        None => format!(
            "{}.ydd",
            Path::new(&args.input)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap_or("gcode")
        ),
    };
    //--
    let gcode = std::fs::read_to_string(args.input).unwrap();
    let bytes = gcode_to_ydd_bytes(
        &gcode,
        args.precision,
        args.tolerance,
        args.interval,
        args.le,
    );
    save_bytes_to_file(&output, &bytes).unwrap();

    //println!("{}", gcode);
    if (args.debug) {
        println!(
            "输出文件路径->{:?}",
            output /*std::fs::canonicalize(&output).unwrap()*/
        );
    }
}
