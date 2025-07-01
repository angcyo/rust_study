///
/// @author <a href="mailto: angcyo@126.com">angcyo</a> \
/// @date 2025/06/04
///
#[cfg(test)]
mod tests {
    use badges::{BadgeBuilder, BadgeColor, BadgeStyle};
    use rc_basis::files::{open_file_with_sys, save_string_to_file};

    /// 生成.svg格式化的徽章
    #[test]
    fn test_badges() {
        let style = BadgeStyle::Flat; //带圆角的扁平
        let badge_svg = BadgeBuilder::new()
            .style(style) //带圆角的扁平
            //.style(BadgeStyle::FlatSquare) //四方扁平
            //.label("badge badge")
            .label("P神")
            .label_color(BadgeColor::Grey)
            //.message("rendered rendered")
            .message("v我50")
            .message_color(BadgeColor::Green)
            .render()
            .expect("failed to render badge");
        //--
        println!(
            "{badge_svg}\n{}",
            open_file_with_sys(
                &save_string_to_file(
                    if style == BadgeStyle::Flat {
                        "../.output/badge_flat.svg"
                    } else {
                        "../.output/badge_flat_square.svg"
                    },
                    &badge_svg
                )
                .unwrap()
            )
        );
    }
}
