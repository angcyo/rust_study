use clap::Parser;

/// 将GCode数据转换成ydd数据
///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/03
///
/// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
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

    /// 需要解析的GCode文件路径
    #[arg(short, long)]
    pub input: String,

    /// 指定输出文件全路径, 不指定则输出在当前目录下
    #[arg(short, long)]
    pub output: Option<String>,

    //--
    /// 公差
    #[arg(long, default_value_t = 0.01)]
    pub tolerance: f32,

    /// 是否使用打点间隔采样, >0生效
    #[arg(long, default_value_t = 0.0)]
    pub interval: f32,

    /// 写入字节数值时, 数值需要放大的倍数
    #[arg(long, default_value_t = 100)]
    pub precision: usize,

    /// 是否使用小端字节序, 默认:true
    #[arg(long, default_value_t = true)]
    pub le: bool,

    /// 输出数据版本, 默认1
    ///     - 0x01 :坐标使用u16类型数据
    ///     - 0x02 :坐标使用f32类型数据
    #[arg(long, default_value_t = 1)]
    pub data_version: u8,
}
