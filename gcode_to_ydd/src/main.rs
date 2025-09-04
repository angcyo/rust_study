use clap_builder::Parser;
use rc_gcode::handler::GCodeValueHandlerPath;
use rc_gcode::parser::GCodeParser;
use std::path::Path;

mod args;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/03
///
fn main() {
    let args = args::Args::parse();
    //--
    if (args.debug) {
        println!("当前工作路径->{:?}", std::env::current_dir().unwrap());
        println!(
            "输入文件路径->{:?}",
            std::fs::canonicalize(&args.input).unwrap()
        );
    }
    //输出文件全路径
    let output = match args.output {
        Some(output) => output,
        None => format!(
            "{}.ydd",
            Path::new(&args.input)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap_or("gcode")
        ),
    };
    //--
    let gcode = std::fs::read_to_string(args.input).unwrap();
    let mut parser = GCodeParser::new(gcode);

    let mut handler = GCodeValueHandlerPath::default();
    parser.parse(&mut handler);

    for (i, layer) in handler.layers.iter().enumerate() {
        let path = &layer.path;
    }
    //println!("{}", gcode);
    if (args.debug) {
        println!(
            "输出文件路径->{:?}",
            output /*std::fs::canonicalize(&output).unwrap()*/
        );
    }
}
