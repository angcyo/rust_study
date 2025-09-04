use clap::Parser;

/// 占位数据生成工具 by angcyo.
///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/05/27
///
/// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    /// 需要生成的数据大小
    #[arg(short, long)]
    pub size: u64,

    /// 占位数据或占位数据对应的文件路径
    #[arg(short, long)]
    pub input: Option<String>,

    /// 输出文件路径, 不指定则输出在当前目录下
    #[arg(short, long)]
    pub output: Option<String>,
}
