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

    log::error!("error message");
    log::error!("error with fmt: {}", 42);
    log::warn!("warn message");
    log::info!("info message");
    log::debug!("debug message");
    log::trace!("trace message");
}
