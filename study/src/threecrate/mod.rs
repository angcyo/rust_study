use rand::{thread_rng, Rng};
use threecrate_core::{Point3f, PointCloud};
use threecrate_visualization::InteractiveViewer;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/16
///
pub fn test_three_crate_core() {
    // Create point cloud
    /*let points = vec![
        Point3f::new(0.0, 0.0, 0.0),
        Point3f::new(1.0, 0.0, 0.0),
        Point3f::new(0.0, 1.0, 0.0),
    ];*/

    /*// Create a simple point cloud for demonstration
    let mut points = Vec::new();

    // Create a simple 3D cube point cloud
    for x in -5..=5 {
        for y in -5..=5 {
            for z in -5..=5 {
                if x == -5 || x == 5 || y == -5 || y == 5 || z == -5 || z == 5 {
                    points.push(Point3f::new(x as f32 * 0.1, y as f32 * 0.1, z as f32 * 0.1));
                }
            }
        }
    }*/

    //let cloud = PointCloud::from_points(points);

    //let cloud = create_simple_planar_cloud();
    //let cloud = create_noisy_planar_cloud();
    //let cloud = create_tilted_plane_with_outliers();
    let cloud = create_multiple_planes();

    // Create and run viewer
    let mut viewer = InteractiveViewer::new().unwrap();
    viewer.set_point_cloud(&cloud);
    viewer.run().unwrap();
}


/// Create a simple planar point cloud on the XY plane
fn create_simple_planar_cloud() -> PointCloud<Point3f> {
    let mut cloud = PointCloud::new();

    // Create a 10x10 grid on the XY plane (z=0)
    for i in 0..10 {
        for j in 0..10 {
            cloud.push(Point3f::new(i as f32, j as f32, 0.0));
        }
    }

    // Add a few outliers
    cloud.push(Point3f::new(5.0, 5.0, 10.0));
    cloud.push(Point3f::new(5.0, 5.0, -10.0));
    cloud.push(Point3f::new(15.0, 15.0, 5.0));

    cloud
}

/// Create a noisy planar point cloud
fn create_noisy_planar_cloud() -> PointCloud<Point3f> {
    let mut cloud = PointCloud::new();
    let mut rng = thread_rng();

    // Create a 20x20 grid on the XY plane with noise
    for i in 0..20 {
        for j in 0..20 {
            let x = i as f32;
            let y = j as f32;
            let z = rng.gen_range(-0.03..0.03); // Add noise to z coordinate
            cloud.push(Point3f::new(x, y, z));
        }
    }

    // Add outliers
    for _ in 0..30 {
        let x = rng.gen_range(0.0..20.0);
        let y = rng.gen_range(0.0..20.0);
        let z = rng.gen_range(2.0..8.0); // Outliers above the plane
        cloud.push(Point3f::new(x, y, z));
    }

    cloud
}

/// Create a tilted plane with outliers
fn create_tilted_plane_with_outliers() -> PointCloud<Point3f> {
    let mut cloud = PointCloud::new();
    let mut rng = thread_rng();

    // Create a tilted plane: x + y + z = 0
    for i in 0..15 {
        for j in 0..15 {
            let x = i as f32;
            let y = j as f32;
            let z = -(x + y); // Points on the plane x + y + z = 0

            // Add some noise
            let noise_x = rng.gen_range(-0.02..0.02);
            let noise_y = rng.gen_range(-0.02..0.02);
            let noise_z = rng.gen_range(-0.02..0.02);

            cloud.push(Point3f::new(x + noise_x, y + noise_y, z + noise_z));
        }
    }

    // Add outliers
    for _ in 0..50 {
        let x = rng.gen_range(0.0..15.0);
        let y = rng.gen_range(0.0..15.0);
        let z = rng.gen_range(5.0..15.0); // Outliers above the plane
        cloud.push(Point3f::new(x, y, z));
    }

    cloud
}

/// Create multiple planes (for demonstrating single plane detection)
fn create_multiple_planes() -> PointCloud<Point3f> {
    let mut cloud = PointCloud::new();
    let mut rng = thread_rng();

    // First plane: z = 0 (largest)
    for i in 0..25 {
        for j in 0..25 {
            let x = i as f32;
            let y = j as f32;
            let z = rng.gen_range(-0.02..0.02); // Small noise
            cloud.push(Point3f::new(x, y, z));
        }
    }

    // Second plane: z = 5 (smaller)
    for i in 0..10 {
        for j in 0..10 {
            let x = i as f32;
            let y = j as f32;
            let z = 5.0 + rng.gen_range(-0.02..0.02); // Small noise
            cloud.push(Point3f::new(x, y, z));
        }
    }

    // Third plane: x = 0 (smallest)
    for i in 0..5 {
        for j in 0..5 {
            let x = rng.gen_range(-0.02..0.02); // Small noise
            let y = i as f32;
            let z = j as f32;
            cloud.push(Point3f::new(x, y, z));
        }
    }

    // Add some random outliers
    for _ in 0..20 {
        let x = rng.gen_range(-5.0..30.0);
        let y = rng.gen_range(-5.0..30.0);
        let z = rng.gen_range(-5.0..10.0);
        cloud.push(Point3f::new(x, y, z));
    }

    cloud
}

#[cfg(test)]
mod tests {
    use crate::threecrate::test_three_crate_core;

    #[test]
    fn test_three_crate() {
        test_three_crate_core();
    }
}
