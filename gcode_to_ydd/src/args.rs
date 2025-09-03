use clap::Parser;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/03
///
///

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    /// 是否开启调试输出
    #[arg(short, long, default_value = "false")]
    pub debug: bool,

    /// 需要解析的GCode文件路径
    #[arg(short, long)]
    pub input: String,

    /// 指定输出文件全路径, 不指定则输出在当前目录下
    #[arg(short, long)]
    pub output: Option<String>,
}
