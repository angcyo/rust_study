use clap::{Parser, ValueEnum};

/// 图片大小调整工具 by angcyo.
///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2026-5-29
///
/// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    /// 输入数据
    /// - 支持单图片
    /// - 支持文件夹
    #[arg(short, long)]
    pub input: String,

    /// 需要调整到的宽度
    #[arg(long)]
    pub width: u32,

    /// 需要调整到的高度
    #[arg(long)]
    pub height: u32,

    /// 是否保持图片原始比例
    #[arg(short, long)]
    pub keep_ratio: Option<bool>,

    /// 是否以base64格式输出
    #[arg(long)]
    pub base64: Option<bool>,

    /// 压缩算法类型
    #[arg(short, long, value_enum)]
    pub filter_type: Option<FilterType>,
}
#[derive(ValueEnum, Clone, Debug)]
pub enum FilterType {
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}

impl Into<image::imageops::FilterType> for FilterType {
    fn into(self) -> image::imageops::FilterType {
        match self {
            FilterType::Nearest => image::imageops::FilterType::Nearest,
            FilterType::Triangle => image::imageops::FilterType::Triangle,
            FilterType::CatmullRom => image::imageops::FilterType::CatmullRom,
            FilterType::Gaussian => image::imageops::FilterType::Gaussian,
            FilterType::Lanczos3 => image::imageops::FilterType::Lanczos3,
        }
    }
}
