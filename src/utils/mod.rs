#![allow(dead_code)]

pub mod file_utils;

use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::{DecodeError, Engine};
use chrono::Utc;
use rand::distr::uniform::{SampleRange, SampleUniform};
use rand::Rng;
use simple_logger::SimpleLogger;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/22
///
/// 获取当前工作目录

pub fn get_current_dir() -> String {
    std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

/// 获取当前13位毫秒时间戳
pub fn now_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

/// 获取当前日期的字符串

pub fn now_date_time() -> String {
    Utc::now().to_string()
}

/// 生成一个uuid
/// `03B5916C0B104D619BD43D5148837217`

pub fn uuid() -> String {
    uuid::Uuid::new_v4()
        .to_string()
        .to_uppercase()
        .replace("-", "")
}

/// 确保文件对应的文件夹存在

pub fn ensure_dir_exist(file_path: &str) {
    let dir = std::path::Path::new(file_path);
    if !dir.exists() {
        std::fs::create_dir_all(dir).unwrap();
    }
}

pub fn ensure_parent_dir_exist(file_path: &str) {
    let dir = std::path::Path::new(file_path).parent().unwrap();
    if !dir.exists() {
        std::fs::create_dir_all(dir).unwrap();
    }
}

/// 取"/"最后一节路径

pub fn last_path(file_path: &str) -> String {
    file_path.split("/").last().unwrap().to_string()
}

//--

/// 将字节数组转换成utf8字符串

pub fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).to_string()
    //String::from_utf8(bytes.to_vec()).unwrap()
}

/// 将utf8字符串转换成字节数组

pub fn string_to_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

/// 将字节数组进行base64加密
/// [STANDARD]
/// [STANDARD_NO_PAD]

pub fn base64_encode(bytes: &[u8]) -> String {
    STANDARD_NO_PAD.encode(bytes)
}

/// 将base64的字符串进行解密

pub fn base64_decode(s: &str) -> Result<Vec<u8>, DecodeError> {
    STANDARD_NO_PAD.decode(s)
}

/// 将字节数组进行md5加密
/// `93E11B05413C8F043BFFCFC5C3D6E68B`
pub fn md5_encode(bytes: &[u8]) -> String {
    format!("{:X}", md5::compute(bytes))
}

//--

/// 随机生成一个浮点数

pub fn random_f64() -> f64 {
    let mut rng = rand::rng();
    rng.random()
}

/// 在一个范围内随机
/// `random_range(0..100)`

pub fn random_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    let mut rng = rand::rng();
    rng.random_range(range)
}

//--

/// 初始化工具类

pub fn init_utils() {
    init_log();
}

//--

fn init_log() {
    //env_logger::init();
    //env_logger::Builder::from_default_env().init();

    // Quick start: use default initialization
    //colog::init();

    SimpleLogger::new().init().unwrap();

    // log::error!("error message");
    // log::error!("error with fmt: {}", 42);
    // log::warn!("warn message");
    // log::info!("info message");
    // log::debug!("debug message");
    // log::trace!("trace message");
}
