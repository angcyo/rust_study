use clap::Parser;

/// 将json文件中的指定key对应的值, 修改成指定的值
///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025-11-03
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

    /// 需要读取的文件
    #[arg(short, long)]
    pub input: String,

    /// 指定输出文件全路径, 不指定则是源文件[input]
    #[arg(short, long)]
    pub output: Option<String>,

    //--
    /// 需要修改的key
    #[arg(short, long)]
    pub key: String,

    /// 需要修改的对应值, 目前仅支持 [String] 类型
    #[arg(short, long)]
    pub value: String,
}
