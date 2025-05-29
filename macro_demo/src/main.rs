use rc_macro::print_token_stream;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/29
///
fn main() {
    //println!("Hello, world!");
    print_token_stream!(SELECT * FROM users WHERE age > 10);
}
