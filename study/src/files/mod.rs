///
/// @author <a href="mailto: angcyo@126.com">angcyo</a> \
/// @date 2025/06/10
///
#[cfg(test)]
mod tests {

    #[test]
    fn test_read_file() {
        // let path = "/Users/angcyo/project/angcyo/3d/output/Toothy_Baby_Croc.gcode";
        let path = "/Users/angcyo/project/angcyo/3d/temp1-Toothy_Baby_Croc.gcode";
        //读取文件, 一行一行枚举
        //统计耗时
        let start = std::time::Instant::now();
        let content = std::fs::read_to_string(path).unwrap();
        let lines = content.lines();
        let mut count = 0;
        for line in lines {
            count += 1;
            //println!("{}", line.len());
            rc_basis::strings::regex_replace_string(
                line,
                r"G1 (X([-+]?\d*\.?\d+)+ Y([-+]?\d*\.?\d+)+ F([-+]?\d*\.?\d+)+)",
                "G0 $1",
            );
        }
        println!("...end:{},{}", count, start.elapsed().as_millis());
    }
}
