///
/// @author <a href="mailto: angcyo@126.com">angcyo</a> \
/// @date 2025/07/21
///
#[cfg(test)]
mod tests {
    use lyon::geom::Point;
    use lyon_algorithms::length::approximate_length;
    use lyon_algorithms::measure::{PathMeasurements, SampleType};
    use lyon_path::math::point;
    use lyon_path::Path;

    fn build_path() -> Path {
        // Build a simple path.
        let mut builder = Path::builder();
        builder.begin(point(0.0, 0.0));
        builder.line_to(point(3.0, 4.0));
        builder.line_to(point(6.0, 0.0));
        //builder.end(false); //10.0
        builder.close(); //16.0

        // Generate the actual path object.
        let path = builder.build();
        path
    }

    fn build_svg_path() -> Path {
        //let path = build_path();
        let mut builder = Path::svg_builder();
        builder.move_to(point(0.0, 0.0));
        builder.line_to(point(3.0, 4.0));
        builder.line_to(point(6.0, 0.0));
        builder.close();
        let svg = builder.build();
        svg
    }

    #[test]
    fn test_lyon() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 2.0),
            Point::new(2.0, 1.0),
            Point::new(0.0, 3.0),
        ];
        /*let hull = convex_hull(&points);
        println!("Convex hull: {:?}", hull);*/
    }

    /// 欧几里得几何图形长度
    #[test]
    fn test_lyon_path_length() {
        let path = build_path();

        // Begin { at: (0.0, 0.0) }
        // Line { from: (0.0, 0.0), to: (1.0, 2.0) }
        // Line { from: (1.0, 2.0), to: (2.0, 0.0) }
        // Line { from: (2.0, 0.0), to: (1.0, 1.0) }
        // End { last: (1.0, 1.0), first: (0.0, 0.0), close: true }
        for event in &path {
            println!("{:?}", event);
        }

        //path
        let length = approximate_length(&path, 0.01);
        println!("路径长度: {:?}", length);
    }

    /// 测试路径采样
    #[test]
    fn test_lyon_path_measure() {
        let path = build_path();
        // Build the acceleration structure.
        let measurements = PathMeasurements::from_path(&path, 0.01);
        let mut sampler = measurements.create_sampler(&path, SampleType::Normalized); //按照进度采样

        let length = sampler.length();
        //路径长度: 16.0
        println!("路径长度: {:?}", length);

        let sample = sampler.sample(0.5);
        //Mid-point position: (4.8, 1.5999999), tangent: (0.6, -0.8)
        println!(
            "Mid-point position: {:?}, tangent: {:?}",
            sample.position(),
            sample.tangent()
        );

        let mut second_half = Path::builder();
        sampler.split_range(0.5..1.0, &mut second_half);
        let second_half = second_half.build();
        assert!((length / 2.0 - approximate_length(&second_half, 1e-3)).abs() < 1e-3);

        //--

        let measurements = PathMeasurements::from_path(&path, 0.01);
        let mut sampler = measurements.create_sampler(&path, SampleType::Distance); //按照距离采样
        let sample = sampler.sample(length / 2.0);
        //Mid-point position2: (4.8, 1.5999999), tangent: (0.6, -0.8)
        println!(
            "Mid-point position2: {:?}, tangent: {:?}",
            sample.position(),
            sample.tangent()
        );

        //--

        let sample = sampler.sample(0.5);
        //sample position: (0.3, 0.4), tangent: (0.6, 0.8)
        println!(
            "sample position: {:?}, tangent: {:?}",
            sample.position(),
            sample.tangent()
        );
    }
}
