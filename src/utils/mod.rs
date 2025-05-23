use base64::engine::general_purpose::STANDARD;
use base64::{DecodeError, Engine};
use chrono::Utc;
use simple_logger::SimpleLogger;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/22
///
/// 获取当前13位毫秒时间戳
#[allow(dead_code)]
pub fn now_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

/// 获取当前日期的字符串
#[allow(dead_code)]
pub fn now_date_time() -> String {
    Utc::now().to_string()
}

//--

/// 将字节数组转换成utf8字符串
#[allow(dead_code)]
pub fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).to_string()
    //String::from_utf8(bytes.to_vec()).unwrap()
}

/// 将utf8字符串转换成字节数组
#[allow(dead_code)]
pub fn string_to_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

/// 将字节数组进行base64加密
#[allow(dead_code)]
pub fn base64_encode(bytes: &[u8]) -> String {
    STANDARD.encode(bytes)
}

/// 将base64的字符串进行解密
#[allow(dead_code)]
pub fn base64_decode(s: &str) -> Result<Vec<u8>, DecodeError> {
    STANDARD.decode(s)
}

/// 将字节数组进行md5加密
#[allow(dead_code)]
pub fn md5_encode(bytes: &[u8]) -> String {
    format!("{:X}", md5::compute(bytes))
}

//--

/// 初始化工具类
#[allow(dead_code)]
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
