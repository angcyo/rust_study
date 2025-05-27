///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/22
///
#[macro_export]
macro_rules! ptl {
    () => {
        eprintln!("[{}:{}:{}]", file!(), line!(), column!())
    };
      ($($arg:tt)*) => {{
        eprintln!("[{}:{}:{}]->{}", file!(), line!(), column!(), format!($($arg)*))
    }};
}
