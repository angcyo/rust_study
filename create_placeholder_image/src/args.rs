use rc_command::clap;
use rc_command::clap::Parser;

/// 占位图生成工具 by angcyo.
///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/05/27
///

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    /// 图片颜色, 16进制RGBA色值 "#f6f6f6ff"
    #[arg(short, long)]
    pub color: Option<String>,

    /// 文本的颜色,16进制RGBA色值 "#000000ff"
    #[arg(long, default_value = "#000000ff")]
    pub text_color: String,

    /// 生成的图片宽度
    #[arg(long, default_value_t = 100)]
    pub width: u32,

    /// 生成的图片高度
    #[arg(long, default_value_t = 100)]
    pub height: u32,

    /// 输出文件路径, 不指定则输出base64数据到控制台
    #[arg(short, long)]
    pub output: Option<String>,
}
