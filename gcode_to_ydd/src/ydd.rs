use rc_bytes::writer::ByteWriter;
use rc_gcode::handler::GCodeValueHandlerPath;
use rc_gcode::lines::each_path_line;
use rc_gcode::parser::GCodeParser;
use rc_gcode::path_bounds;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/04
///
/// 将[Path]转换成ydd数据
/// - [le] 是否使用小端序
fn path_to_ydd_bytes(
    path: &lyon_path::Path,
    precision: usize,
    tolerance: f32,
    interval: f32,
    le: bool,
) -> Vec<u8> {
    let mut bytes_writer = ByteWriter::default();
    let mut points: Vec<(f32, f32)> = vec![];

    let pe = precision as f32;

    //--
    let append_points = |writer: &mut ByteWriter, points: &Vec<(f32, f32)>| {
        if !points.is_empty() {
            //part1
            writer.write_int8(0, le);
            //part2
            writer.write_int16(points.len() as i16, le);
            for p in points.iter() {
                writer.write_int16((p.0 * pe) as i16, le);
                writer.write_int16((p.1 * pe) as i16, le);
            }
        }
    };

    //--
    each_path_line(path, tolerance, interval, |new_line, p| {
        //新的线段
        if new_line {
            append_points(&mut bytes_writer, &points);
            points = vec![];
        }

        //添加点
        if let Some(p) = p {
            points.push((p.0, p.1));
        }
    });
    append_points(&mut bytes_writer, &points);

    bytes_writer.bytes
}

/// 将GCode转换成ydd数据
/// - [gcode] gcode文本
/// - [precision] 数值精度, 默认为100
/// - [tolerance] 公差, 默认为0.01
/// - [interval] 是否间隔采样, >0生效
/// - [le] 是否使用小端序
pub fn gcode_to_ydd_bytes(
    gcode: &String,
    precision: usize,
    tolerance: f32,
    interval: f32,
    le: bool,
) -> Vec<u8> {
    let pe = precision as f32;

    let mut parser = GCodeParser::new(gcode);

    let mut handler = GCodeValueHandlerPath::default();
    parser.parse(&mut handler);

    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    let mut group_writer = ByteWriter::default();

    let mut count = 0;
    for (i, layer) in handler.layers.iter().enumerate() {
        let z = layer.z_f32();
        let path = &layer.path;

        let item_bounds = path_bounds(path);
        min_x = min_x.min(item_bounds.0);
        min_y = min_y.min(item_bounds.1);
        max_x = max_x.max(item_bounds.2);
        max_y = max_y.max(item_bounds.3);

        let bytes = path_to_ydd_bytes(path, precision, tolerance, interval, le);

        //part1
        let mut item_part1_writer = ByteWriter::default();
        item_part1_writer.write_int16(0x10, le); //数据类型
        item_part1_writer.write_int16((item_bounds.0 * pe) as i16, le);
        item_part1_writer.write_int16((item_bounds.1 * pe) as i16, le);
        let w = item_bounds.2 - item_bounds.0;
        item_part1_writer.write_int16((w * pe) as i16, le);
        let h = item_bounds.3 - item_bounds.1;
        item_part1_writer.write_int16((h * pe) as i16, le);
        //fill-dpi
        item_part1_writer.write_int16(0, le);

        //part2
        let mut item_part2_writer = ByteWriter::default();
        item_part2_writer.write_int16(0, le); //激光功率
        item_part2_writer.write_int32(60 * 1000, le); //雕刻速度mm/min
        item_part2_writer.write_int8(1, le); //激光类型, 0:450激光 1:1064激光
        item_part2_writer.write_int16(60, le); //激光频率
        item_part2_writer.write_int16(20, le); //激光脉宽
        item_part2_writer.write_int16(1, le); //重复次数
        item_part2_writer.write_int16((z * pe) as i16, le); //支架高度

        //单个元素数据
        let mut item_writer = ByteWriter::default();
        item_writer.write_int8(item_part1_writer.bytes.len() as i8, le);
        item_writer.write_vec(&item_part1_writer.bytes);
        item_writer.write_int8(item_part2_writer.bytes.len() as i8, le);
        item_writer.write_vec(&item_part2_writer.bytes);
        item_writer.write_int32(bytes.len() as i32, le);
        item_writer.write_vec(&bytes);

        //group
        group_writer.write_vec(&item_writer.bytes);

        count += 1;
    }

    //result
    let mut result_writer = ByteWriter::default();
    result_writer.write_ascii_string("YDMG");

    let mut result_part1_writer = ByteWriter::default();
    result_part1_writer.write_int8(1, le);
    result_part1_writer.write_int8(0, le);

    result_writer.write_int8(result_part1_writer.bytes.len() as i8, le);
    result_writer.write_vec(&result_part1_writer.bytes);

    let mut result_part2_writer = ByteWriter::default();
    result_part2_writer.write_int16(count, le);
    result_part2_writer.write_int8(0, le);
    result_part2_writer.write_int16((min_x * pe) as i16, le);
    result_part2_writer.write_int16((min_y * pe) as i16, le);
    let w = max_x - min_x;
    result_part2_writer.write_int16((w * pe) as i16, le);
    let h = max_y - min_y;
    result_part2_writer.write_int16((h * pe) as i16, le);
    result_part2_writer.write_int32(group_writer.bytes.len() as i32, le); //组内数据总字节数

    result_writer.write_int8(result_part2_writer.bytes.len() as i8, le);
    result_writer.write_vec(&result_part2_writer.bytes);

    //--
    result_writer.write_vec(&group_writer.bytes); //组内数据

    result_writer.bytes
}

#[cfg(test)]
mod tests {
    use crate::ydd::gcode_to_ydd_bytes;

    #[test]
    fn test_gcode_to_ydd_bytes() {
        let input = "../rust_crates/tests/.output/path_to_gcode.gcode";
        let gcode = std::fs::read_to_string(&input).unwrap();
        let bytes = gcode_to_ydd_bytes(&gcode, 100, 0.01, 0.0, true);
        println!("{:?}", bytes);
    }
}
