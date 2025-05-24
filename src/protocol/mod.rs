use prost::Message;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/23
///
//pub mod protocol.items;
pub mod shirt;

#[allow(dead_code)]
pub fn test_protocol() {
    let shirt = shirt::Shirt {
        color: "blue".to_string(),
        size: 2,
    };
    // 将 Shirt 序列化为字节数组
    let mut buf = Vec::new();
    // let mut buf = vec![];
    shirt.encode(&mut buf).unwrap();

    crate::ptl!("Encoded shirt: {:?}", crate::utils::base64_encode(&buf));

    // 从字节数组反序列化为 Shirt 实例
    let decoded_shirt = shirt::Shirt::decode(&buf[..]).unwrap();

    // 输出反序列化出来的内容
    println!("Decoded shirt: {:?}", decoded_shirt);
}
