use palette::Srgba;
use rc_basis::colors::RgbaColor;
use rc_basis::num::min_f32;
use rc_basis::ptl;
use rc_command::clap_builder::Parser;
use rc_image::image;
use rc_image::image::{EncodableLayout, Rgba};
use rusttype::{point, Font, Scale};
use std::cmp::{max, min};
use std::fmt::format;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

mod args;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/27
///

const FONT_DATA: &[u8] = include_bytes!("assets/SourceHanSansCN-Normal.otf");

fn main() {
    let args = args::Args::parse();
    //ptl!("{}->{:?}", rc_basis::get_current_dir(), args);

    //Srgba::from_str(args.color.as_str()).unwrap();
    //args.color.parse<Srgba>();
    //Argb::from(args.color.as_str()).unwrap();
    //Rgba::from_str(args.color.as_str()).unwrap();

    let rgba: Srgba<u8> = args
        .color
        .unwrap_or("#f6f6f6ff".to_string())
        .parse()
        .unwrap();
    let text_rgba: Srgba<u8> = args.text_color.parse().unwrap();
    let color = u32::from_rgba(rgba.red, rgba.green, rgba.blue, rgba.alpha);
    let text_color = u32::from_rgba(
        text_rgba.red,
        text_rgba.green,
        text_rgba.blue,
        text_rgba.alpha,
    );
    let width = args.width;
    let height = args.height;
    let mut img = rc_image::write::create_image(width, height, color);

    // 加载字体
    let mut font_data = FONT_DATA;
    //let mut font_data = Vec::new();
    // 你需要准备一个 TTF 字体文件，比如 "DejaVuSans.ttf"
    //let mut font_file = File::open("SourceHanSansCN-Normal.otf").expect("字体文件未找到");
    //font_file.read_to_end(&mut font_data).unwrap();
    let font = Font::try_from_vec(font_data.to_vec()).unwrap();

    let text = format!("{width}x{height}");
    let size = min(width, height);
    // 设置字体大小
    //let text_height = size as f32 / 3f32;
    let text_height = size as f32 / text.len() as f32 * 2f32;
    let scale = Scale::uniform(text_height);

    // 设置文本起始点
    let start = point(0.0, 0.0);

    //测量文本的宽度和高度
    let mut min_left = 0;
    let mut max_right = 0;
    let mut min_top = 0;
    let mut max_bottom = 0;
    for glyph in font.layout(text.as_str(), scale, start) {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let left = x as i32 + bb.min.x;
                let top = y as i32 + bb.min.y;
                let right = left + bb.width();
                let bottom = top + bb.height();
                min_left = min(min_left, left);
                min_top = min(min_top, top);
                max_right = max(max_right, right);
                max_bottom = max(max_bottom, bottom);
            });
        }
    }
    let measure_width = max_right - min_left;
    let measure_height = max_bottom - min_top;

    //移动的背景的中心
    let center_offset_x = width as f32 / 2f32 - measure_width as f32 / 2f32;
    let center_offset_y = height as f32 / 2f32 - measure_height as f32 / 2f32 + text_height;

    // 绘制文本
    for glyph in font.layout(text.as_str(), scale, start) {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as i32 + bb.min.x + center_offset_x as i32;
                let y = y as i32 + bb.min.y + center_offset_y as i32;
                if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 && v > 0.01 {
                    let px = img.get_pixel_mut(x as u32, y as u32);
                    //let gray = (v * 255.0) as u8;
                    let r = (v * text_color.r() as f32) as u8;
                    let g = (v * text_color.g() as f32) as u8;
                    let b = (v * text_color.b() as f32) as u8;
                    let a = (v * text_color.a() as f32) as u8;
                    let src = u32::from_rgba(r, g, b, a);
                    let dst = u32::from_rgba(px.0[0], px.0[1], px.0[2], px.0[3]);
                    //*px = Rgba([gray, gray, gray, gray]); // 红色文字
                    //*px = Rgba([255, 255, 255, 255]);
                    let color = src.mix(&dst);
                    *px = Rgba([color.r(), color.g(), color.b(), color.a()]);
                }
            });
        }
    }

    if let Some(output) = args.output {
        rc_basis::files::ensure_parent_dir_exist(&output);
        img.save_with_format(output, image::ImageFormat::Png)
            .unwrap();
    } else {
        println!(
            "data:image/png;base64,{}",
            rc_image::convert::image_buffer_to_base64(&img).unwrap()
        );
    }
}
