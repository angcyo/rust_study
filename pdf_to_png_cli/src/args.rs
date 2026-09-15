use clap::Parser;

/// 将`pdf`导出成`png`图片
///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2026-9-15
///
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
#[command(flatten_help = true)]
//#[command(help_template = "{about}\n\n{usage}\n\n{all_args}\n\n{after_help}")]
// #[command(name = "myapp")]
// #[command(version = "1.0.0")]
// #[command(help = "显示帮助信息")]
//#[command(help_template = utils::FULL_TEMPLATE)]
pub(crate) struct Args {
    /// 需要读取的pdf文件
    #[arg(short, long)]
    pub input: String,

    /// 指定输出文件夹, 不指定则在原文件目录下输出
    #[arg(short, long)]
    pub output: Option<String>,

    /// 设置分辨率 (默认为 96, DPI 越高图片越清晰)
    #[arg(short, long)]
    pub dpi: Option<u32>,
}
