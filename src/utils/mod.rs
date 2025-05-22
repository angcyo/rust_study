use chrono::Utc;

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
