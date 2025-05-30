use rc_macro::{print_token_stream, DerivePrintToken};

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/29
///
fn main() {
    let a = MacroStruct {
        name: "angcyo".to_string(),
        name2: Some("angcyo2".to_string()),
    };
    println!("{:?}", a);
    print_token_stream!(SELECT * FROM users WHERE age > 10);
}

#[derive(Debug, DerivePrintToken)]
pub struct MacroStruct {
    name: String,
    pub name2: Option<String>,
}
