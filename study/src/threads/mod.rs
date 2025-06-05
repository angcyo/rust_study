///
/// @author <a href="mailto: angcyo@126.com">angcyo</a> \
/// @date 2025/06/05
///
#[cfg(test)]
mod tests {
    use std::thread;

    /// Main thread name: "threads::tests::test_thread"
    /// Current thread name: "MyThread"
    #[test]
    fn test_thread() {
        // 创建一个新的线程并为其设置名称
        let handle = thread::Builder::new()
            .name("MyThread".to_string())
            .spawn(move || {
                // 获取并打印当前线程名称
                let current_thread = thread::current();
                println!("Current thread name: {:?}", current_thread.name().unwrap_or(""));
            })
            .unwrap();

        // 在主线程中获取当前线程名称
        let main_thread = thread::current();
        println!("Main thread name: {:?}", main_thread.name().unwrap_or(""));

        // 等待子线程完成
        handle.join().unwrap();
    }
}
