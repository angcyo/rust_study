#[cfg(test)]
mod tests {
    use crate::test::{get_test_file_path, save_and_open_file};
    use geo::{
        line_string, point, ConcaveHull, Euclidean, EuclideanDistance, EuclideanLength,
        Length, LineString,
    };
    use geo_offset::Offset;
    use geo_svg_io2::geo_svg_reader::{svg_to_geometry, svg_to_geometry_collection};
    use geo_svg_io2::geo_svg_writer::{ToSvg, ToSvgString};
    use rc_basis::files::read_file_to_string;

    /// 几何图形测试
    #[test]
    fn test_geo() {
        // 创建一个折线路径
        let line = LineString::from(vec![(0.0, 0.0), (3.0, 4.0), (6.0, 0.0)]);
        let line2 = line_string![(x: 0.0, y:0.0), (x:3.0, y:4.0), (x:6.0, y:0.0)];
        // 计算总长度（欧几里得长度）
        let length = line.euclidean_length();
        let length2 = Euclidean.length(&line2);

        println!("路径长度: {}, {}", length, length2); // 输出 10

        let p1 = point!(x: 0.0, y: 0.0);
        let p2 = point!(x: 3.0, y: 4.0);

        let distance = p1.euclidean_distance(&p2);
        println!("两点距离: {}", distance);

        for (i, point) in line.points()/*points_iter()*/.enumerate() {
            println!("第{}个点: ({}, {})", i, point.x(), point.y());
        }
    }

    /// 测试单个凹壳
    #[test]
    fn test_geo_concave_hull() {
        let name = "love.svg";
        let svg = read_file_to_string(get_test_file_path(name).as_str()).unwrap();
        //println!("{}", svg);
        //open_file_with_sys(&svg);

        let parsed_svg = svg_to_geometry(&svg).ok().unwrap();
        let parsed_poly = parsed_svg.into_polygon().unwrap();
        let concave_hull = parsed_poly.concave_hull(2.0);
        let svg = concave_hull.to_svg(); //svg <path> xml , 只有<path>标签的xml文档
        let svg_path = concave_hull.to_svg_string(); //svg path

        save_and_open_file(name, svg.as_bytes());

        // a square shape
        /*let poly = polygon![
            (x: 0.0, y: 0.0),
            (x: 4.0, y: 0.0),
            (x: 4.0, y: 4.0),
            (x: 0.0, y: 4.0),
        ];
        poly.concave_hull(2.0);*/
    }

    /// 多个几何图形凹壳
    #[test]
    fn test_geo_collection_concave_hull() {
        let name = "love.svg";
        let svg = read_file_to_string(get_test_file_path(name).as_str()).unwrap();
        //println!("{}", svg);
        //open_file_with_sys(&svg);

        let parsed_svg = svg_to_geometry_collection(&svg).ok().unwrap();
        let parsed_poly = parsed_svg
            .into_iter()
            .map(|poly| poly.into_polygon().unwrap().concave_hull(2.0).to_svg())
            .collect::<String>();

        save_and_open_file(name, parsed_poly.as_bytes());
    }

    /// 测试凸包
    #[test]
    fn test_geo_convex_hull() {
        let name = "star.svg";
        let svg = read_file_to_string(get_test_file_path(name).as_str()).unwrap();
        //println!("{}", svg);
        //open_file_with_sys(&svg);

        let parsed_svg = svg_to_geometry(&svg).ok().unwrap();
        let parsed_poly = parsed_svg.into_polygon().unwrap();
        //let convex_hull = parsed_poly.convex_hull();
        let convex_hull = parsed_poly.concave_hull(-10.0);
        let svg = convex_hull.to_svg(); //svg <path> xml , 只有<path>标签的xml文档
        let svg_path = convex_hull.to_svg_string(); //svg path

        save_and_open_file(name, svg.as_bytes());
    }

    /// 测试偏移
    #[test]
    fn test_geo_offset() {
        /*let name = "star.svg";
        let output_name = "star_offset.svg";*/

        /*let name = "love.svg";
        let output_name = "love_offset.svg";*/

        /*let name = "love2.svg";
        let output_name = "love2_offset.svg";*/

        let name = "xtool-export-star.svg";
        let output_name = "xtool-export-star-offset.svg";

        let svg = read_file_to_string(get_test_file_path(name).as_str()).unwrap();
        //println!("{}", svg);
        //open_file_with_sys(&svg);

        let parsed_svg = svg_to_geometry(&svg).ok().unwrap();
        let parsed_poly = parsed_svg.into_polygon().unwrap();
        let convex_hull = parsed_poly.offset(10.0).ok().unwrap();
        let svg = convex_hull.to_svg(); //svg <path> xml , 只有<path>标签的xml文档
        let svg_path = convex_hull.to_svg_string(); //svg path

        save_and_open_file(output_name, svg_path.as_bytes());
    }
}
