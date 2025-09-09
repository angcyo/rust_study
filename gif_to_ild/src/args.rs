use clap::Parser;

/// 将Gif或图片转换成ild数据, 图片尽量仅使用单线颜色图片.
///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/09/09
///
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
#[command(flatten_help = true)]
//#[command(help_template = utils::FULL_TEMPLATE)]
pub(crate) struct Args {
    /// 是否开启调试输出, 默认:false
    #[arg(long, default_value_t = false)]
    pub debug: bool,

    /// 需要解析的图片文件路径
    #[arg(short, long)]
    pub input: String,

    /// 指定输出文件全路径, 不指定则输出在当前目录下
    #[arg(short, long)]
    pub output: Option<String>,

    //--
    /// 像素坐标偏移量X
    #[arg(short = 'x', long, default_value_t = 0)]
    pub offset_x: i16,

    /// 像素坐标偏移量Y
    #[arg(short = 'y', long, default_value_t = 0)]
    pub offset_y: i16,

    /// 灰度阈值, >这个值的像素, 视为白色255, 无数据
    #[arg(short, long, default_value_t = 250)]
    pub gray_threshold: u8,

    /// 透明阈值, 透明通道<=这个值的像素, 视为白色255, 无数据
    #[arg(short, long, default_value_t = 250)]
    pub alpha_threshold: u8,
}
