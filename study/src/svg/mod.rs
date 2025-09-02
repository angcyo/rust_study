///
/// @author <a href="mailto: angcyo@126.com">angcyo</a> \
/// @date 2025/07/21
///
#[cfg(test)]
mod tests {
    use rc_basis::files::read_file_to_string;
    use resvg::tiny_skia;
    use resvg::tiny_skia::Pixmap;
    use usvg::{Options, Tree, WriteOptions};
    use rc_basis::test::{get_test_file_path, get_test_output_file_path};

    #[test]
    fn test_usvg() {
        let tree = Tree::from_str(&"<svg></svg>", &Options::default()).unwrap();

        for node in tree.root().children() {
            println!("Node: {:?}", node);
        }

        let nodes = tree.root().children();
        //nodes.to_vec().push(Node::Path(usvg::Path::new()))

        /*self.nodes.push(usvg::Node::new(NodeKind::Path(
            lyon_path_to_svg_with_attributes(path, fill, stroke, transform)
                .ok_or(LyonTranslationError::SvgFailure)?,
        )));*/

        //output
        println!("{}", tree.to_string(&WriteOptions::default()));
    }

    /// 测试解析 SVG 文件
    #[test]
    fn test_parse_svg() {
        // 读取 SVG 文件为字符串
        let name = "star.svg";
        let svg = read_file_to_string(get_test_file_path(name).as_str()).unwrap();

        // 解析 SVG 文本为 usvg 的 Tree（DOM）
        let options = Options::default();
        // 可根据需要设置 options
        let tree = Tree::from_str(&svg, &options).unwrap();
        let root = tree.root();
        // 这里可以操作 tree，比如遍历元素、修改属性等
        for node in root.children() {
            println!("Node: {:?}", node);
        }

        //创建一个新的svg文档

        // 导出成 SVG 文档字符串
        let svg_string = tree.to_string(&WriteOptions::default());
        println!("{}", svg_string);
    }

    /// 测试svg转png
    #[test]
    fn test_svg_to_png() {
        let name = "5_Repeating_Patterns_0071-0-x-0.svg";
        let svg = read_file_to_string(get_test_file_path(name).as_str()).unwrap();
        let tree = Tree::from_str(&svg, &Options::default()).unwrap();

        let size = tree.size();
        let width = size.width();
        let height = size.height();

        let rect = tree.root().bounding_box();

        let target_width = 1024;
        let target_height = 1024;

        let mut pixmap = Pixmap::new(target_width, target_height).unwrap();

        resvg::render(
            &tree,
            tiny_skia::Transform::from_scale(
                target_width as f32 / width,
                target_height as f32 / height,
            ),
            &mut pixmap.as_mut(),
        );

        let output = get_test_output_file_path(format!("{}.png", name).as_str());
        pixmap.save_png(output.as_str()).unwrap();

        rc_basis::files::open_file_with_sys(&output);
    }
}
