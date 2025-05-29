///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/23
///

fn main() -> std::io::Result<()> {
    #[cfg(feature = "enable_proto")]
    {
        prost_build::Config::new()
            .out_dir("src/protocol")
            .compile_protos(&["src/protocol/shirt.proto"], &["src/"])?;
        //prost_build::compile_protos(&["src/protocol/items.proto"], &["src/"])?;
    }
    Ok(())
}
