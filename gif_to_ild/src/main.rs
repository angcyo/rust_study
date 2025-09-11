use clap_builder::Parser;
use rc_basis::files::save_bytes_to_file;
use rc_gcode::ild::{gif_path_to_ild_bytes_2d_rgb, image_path_to_ild_bytes};
use std::path::Path;

mod args;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/09/09
///
fn main() {
    let args = args::Args::parse();
    //--
    if (args.debug) {
        println!("当前工作路径->{:?}", std::env::current_dir().unwrap());
        println!(
            "输入文件路径->{:?}",
            std::fs::canonicalize(&args.input).expect("输入的文件不存在")
        );
    }
    //输出文件全路径
    let output = match args.output {
        Some(output) => output,
        None => format!(
            "{}.ild",
            Path::new(&args.input)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap_or("ild")
        ),
    };
    //--gif结尾
    let bytes = if args.input.to_lowercase().ends_with(".gif") {
        //Gif
        gif_path_to_ild_bytes_2d_rgb(
            &args.input,
            args.offset_x,
            args.offset_y,
            args.gray_threshold,
            args.alpha_threshold,
        )
    } else {
        //读取普通图片
        image_path_to_ild_bytes(
            &args.input,
            args.offset_x,
            args.offset_y,
            args.gray_threshold,
            args.alpha_threshold,
        )
    };
    //--
    save_bytes_to_file(&output, &bytes).unwrap();
    //println!("{}", gcode);
    if (args.debug) {
        println!(
            "输出文件路径->{:?}",
            std::fs::canonicalize(&output).unwrap()
        );
    }
}
