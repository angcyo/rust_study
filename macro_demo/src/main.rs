use rc_macro::{
    print_attribute_token_stream, print_token_stream, DerivePrintToken, DerivePrintTokenAttr,
};

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/29
///
#[print_attribute_token_stream]
fn main() {
    let a = MacroStruct {
        name: "angcyo".to_string(),
        name2: Some("angcyo2".to_string()),
    };
    println!("{:?}", a);
    test_fn();
    print_token_stream!(SELECT * FROM users WHERE age > 10);
}

#[derive(Debug, DerivePrintToken, DerivePrintTokenAttr)]
#[print_attribute_token_stream]
pub struct MacroStruct {
    #[DeriveAttr]
    name: String,
    #[DeriveAttr(des = "描述内容", value = 100, test)]
    pub name2: Option<String>,
}

#[print_attribute_token_stream(des = "描述内容", value = 100, test)]
fn test_fn() {
    println!("...test_fn...")
}
