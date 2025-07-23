use ttf_parser::{Face, GlyphId, OutlineBuilder};

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a> \
/// @date 2025/07/23
///

pub fn get_glyph_svg(face: &Face, glyph_id: u16) -> String {
    let mut outline = SvgOutline::new();
    match face.outline_glyph(Some(glyph_id).map(GlyphId).unwrap(), &mut outline) {
        Some(_) => outline.to_svg_path(),
        None => "".to_string(),
    }
}

struct SvgOutline {
    paths: Vec<String>,
}

impl SvgOutline {
    fn new() -> Self {
        SvgOutline { paths: Vec::new() }
    }

    fn to_svg_path(&self) -> String {
        self.paths.join(" ")
    }
}

/// Implement `OutlineBuilder` for SvgOutline to collect TTF paths.
/// y轴方向是向下的，需要转换为向上
impl OutlineBuilder for SvgOutline {
    fn move_to(&mut self, x: f32, y: f32) {
        self.paths.push(format!("M{} {}", x, -y));
        // self.paths.push(format!("M{} {}", x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.paths.push(format!("L{} {}", x, -y));
        // self.paths.push(format!("L{} {}", x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.paths.push(format!("Q{} {} {} {}", x1, -y1, x, -y));
        // self.paths.push(format!("Q{} {} {} {}", x1, y1, x, y));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.paths
            .push(format!("C{} {} {} {} {} {}", x1, -y1, x2, -y2, x, -y));
        // self.paths.push(format!("C{} {} {} {} {} {}", x1, y1, x2, -y2, x, y));
    }

    fn close(&mut self) {
        self.paths.push("Z".to_string());
    }
}

#[cfg(test)]
mod tests {
    use crate::font::get_glyph_svg;
    use rustybuzz::{Face, UnicodeBuffer};
    use unicode_segmentation::UnicodeSegmentation;

    #[test]
    fn test_ttf_parser() {
        let path1 = "E:/IdeaProjects/rust-font-kit-demo/tests/JetBrainsMono-Bold.ttf".to_string(); //不支持中文
        let path2 = "2E:/IdeaProjects/rust-font-kit-demo/tests/微软雅黑-Bold.ttc".to_string();
        let path3 =
            "3E:/IdeaProjects/rust-font-kit-demo/tests/SourceHanSansCN-Normal.otf".to_string();
        let path4 = "../../yd_core/tests/SourceHanSansCN-Normal.otf".to_string();
        let path5 = "../../yd_core/tests/NotoColorEmoji-Regular.ttf".to_string();

        //let text = "我❤️😄".to_string(); //中国人?>
        //let text = "中国".to_string(); //中国人?>
        //let text = "❤".to_string(); //中国人?>
        let text = "❤️".to_string();
        //let text = "😄".to_string();
    }

    /// 测试使用 `unicode-segmentation` 可以将一串文本正确地分割为“用户感知字符”，包括复合 emoji。
    #[test]
    fn test_unicode_segmentation() {
        //[0]->❤
        //char->❤
        //[1]->❤️
        //char->❤
        //char->️
        //[2]->👩‍💻
        //char->👩
        //char->‍
        //char->💻
        //[3]->👍🏽
        //char->👍
        //char->🏽
        //[4]->🇨🇳
        //char->🇨
        //char->🇳
        let text = "❤❤️👩‍💻👍🏽🇨🇳";
        for grapheme in text.graphemes(true).enumerate() {
            println!("[{}]->{}", grapheme.0, grapheme.1);
            grapheme.1.chars().for_each(|c| println!("char->{}", c));
        }
    }

    /// 测试 harfbuzz 塑造算法
    #[test]
    fn test_rust_y_buzz() {
        let path4 = "../../yd_core/tests/SourceHanSansCN-Normal.otf".to_string();
        let path5 = "../../yd_core/tests/NotoColorEmoji-Regular.ttf".to_string();

        let font_data = std::fs::read(path4).unwrap();
        let face = Face::from_slice(&font_data, 0).unwrap();

        // 输入复合 emoji，比如 "👩‍💻"
        //let text = "👩‍💻";
        //let text = "😃";
        let text = "我";
        let mut buffer = UnicodeBuffer::new();
        buffer.push_str(text);

        // Shaping 处理
        let glyph_buffer = rustybuzz::shape(&face, &[], buffer);

        // 获取每个 glyph 信息
        for glyph in glyph_buffer.glyph_infos() {
            println!(
                "Glyph id: {}, cluster: {}->{}",
                glyph.glyph_id,
                glyph.cluster,
                get_glyph_svg(&face, glyph.glyph_id as u16)
            );
        }

        //--

        for grapheme in text.graphemes(true).enumerate() {
            let char = grapheme.1.chars().next().unwrap();
            //let svg = ttf_glyph_to_svg(&face, char);
            //println!("{}->Glyph id: {:?}->{}", char, face.glyph_index(char), svg);
        }
    }
}
