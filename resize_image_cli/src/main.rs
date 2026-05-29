use crate::args::Args;
use clap_builder::Parser;
use std::path::{Path, PathBuf};

mod args;

/// 图片大小调整工具 by angcyo.
///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2026-5-29
///
fn main() {
    let args = Args::parse();
    let input_path = Path::new(&args.input);
    if input_path.is_dir() {
        for entry in input_path.read_dir().expect("无法打开目录") {
            let entry = entry.expect("无法读取目录项");
            let path = entry.path();
            if path.is_file() {
                handle_resize_image(&path, &args, Some(&input_path.to_path_buf()));
            }
        }
    } else if input_path.is_file() {
        let output_path = handle_resize_image(&input_path.to_path_buf(), &args, None);
        if output_path.is_some() {
            println!("{}", output_path.unwrap());
        }
    } else {
        println!("无法处理的输入->{}不是一个文件或目录", input_path.display());
    }
}

fn handle_resize_image(
    path: &PathBuf,
    args: &Args,
    output_dir: Option<&PathBuf>,
) -> Option<String> {
    let image = image::open(path);
    match image {
        Ok(image) => {
            let filter_type = args
                .filter_type
                .as_ref()
                .unwrap_or(&args::FilterType::Lanczos3);
            let new_image = if args.keep_ratio == Some(true) {
                image.resize(args.width, args.height, filter_type.clone().into())
            } else {
                image.resize_exact(args.width, args.height, filter_type.clone().into())
            };
            if args.base64 == Some(true) {
                let base64 = format!(
                    "data:image/png;base64,{}",
                    rc_image::convert::image_to_base64(&new_image).unwrap()
                );
                println!("{}->\n{}", path.to_str().unwrap(), base64);
                Some(base64)
            } else {
                //分割.前后的字符串
                let (file_name, file_extension) = path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .split_once(".")
                    .unwrap();
                let new_file_name = format!(
                    "{}_{}x{}{}{}",
                    file_name, args.width, args.height, ".", file_extension
                );

                match output_dir {
                    Some(output_dir) => {
                        let output_dir = output_dir.join(format!(
                            "{}_{}x{}",
                            output_dir.file_name().unwrap().to_str().unwrap(),
                            args.width,
                            args.height
                        ));
                        if !output_dir.exists() {
                            let _ = std::fs::create_dir_all(output_dir.clone());
                        }
                        let output_file_path = output_dir.join(new_file_name);
                        image.save(&output_file_path).unwrap();
                        Some(output_file_path.to_str()?.to_string())
                    }
                    None => {
                        let new_file_path = path.with_file_name(new_file_name);
                        new_image.save(&new_file_path).unwrap();
                        Some(new_file_path.to_str()?.to_string())
                    }
                }
            }
        }
        Err(e) => {
            println!("无法打开图片[{}]->{}", path.to_str().unwrap(), e);
            None
        }
    }
}
