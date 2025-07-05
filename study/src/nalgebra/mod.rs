///
/// @author <a href="mailto: angcyo@126.com">angcyo</a> \
/// @date 2025/07/05
///
use nalgebra::{Matrix3, Vector3};


/// 计算四点透视变换矩阵
/// src 和 dst 都是 4 个 (x, y) 点
pub fn get_perspective_transform(src: &[(f64, f64); 4], dst: &[(f64, f64); 4]) -> Matrix3<f64> {
    let mut a = [[0.0; 8]; 8];
    let mut b = [0.0; 8];

    for i in 0..4 {
        let (x, y) = src[i];
        let (u, v) = dst[i];
        a[i * 2][0] = x;
        a[i * 2][1] = y;
        a[i * 2][2] = 1.0;
        a[i * 2][3] = 0.0;
        a[i * 2][4] = 0.0;
        a[i * 2][5] = 0.0;
        a[i * 2][6] = -x * u;
        a[i * 2][7] = -y * u;
        b[i * 2] = u;

        a[i * 2 + 1][0] = 0.0;
        a[i * 2 + 1][1] = 0.0;
        a[i * 2 + 1][2] = 0.0;
        a[i * 2 + 1][3] = x;
        a[i * 2 + 1][4] = y;
        a[i * 2 + 1][5] = 1.0;
        a[i * 2 + 1][6] = -x * v;
        a[i * 2 + 1][7] = -y * v;
        b[i * 2 + 1] = v;
    }

    // 求解线性方程组
    let a_mat = nalgebra::DMatrix::from_row_slice(8, 8, &a.concat());
    let b_vec = nalgebra::DVector::from_row_slice(&b);

    let x = a_mat.lu().solve(&b_vec).expect("解不存在");

    let m = [x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7], 1.0];

    Matrix3::from_row_slice(&m)
}

/// 应用透视变换到坐标点
pub fn perspective_transform_point(mat: &Matrix3<f64>, pt: (f64, f64)) -> (f64, f64) {
    let v = Vector3::new(pt.0, pt.1, 1.0);
    let res = mat * v;
    (res[0] / res[2], res[1] / res[2])
}

//--

/// 4点透视变换求解3x3变换矩阵, 不依赖外部库
fn get_perspective_transform2(src: &[(f64, f64); 4], dst: &[(f64, f64); 4]) -> [[f64; 3]; 3] {
    // 构造8x8矩阵和8x1常数项
    let mut a = [[0.0; 8]; 8];
    let mut b = [0.0; 8];

    for i in 0..4 {
        let (x, y) = src[i];
        let (u, v) = dst[i];
        a[i * 2][0] = x;
        a[i * 2][1] = y;
        a[i * 2][2] = 1.0;
        a[i * 2][3] = 0.0;
        a[i * 2][4] = 0.0;
        a[i * 2][5] = 0.0;
        a[i * 2][6] = -x * u;
        a[i * 2][7] = -y * u;
        b[i * 2] = u;
        a[i * 2 + 1][0] = 0.0;
        a[i * 2 + 1][1] = 0.0;
        a[i * 2 + 1][2] = 0.0;
        a[i * 2 + 1][3] = x;
        a[i * 2 + 1][4] = y;
        a[i * 2 + 1][5] = 1.0;
        a[i * 2 + 1][6] = -x * v;
        a[i * 2 + 1][7] = -y * v;
        b[i * 2 + 1] = v;
    }

    // 高斯消元法解8元一次方程组
    let x = gaussian_elimination(&mut a, &mut b);

    // 构造3x3矩阵
    [[x[0], x[1], x[2]], [x[3], x[4], x[5]], [x[6], x[7], 1.0]]
}

/// 单纯的高斯消元求解
fn gaussian_elimination(a: &mut [[f64; 8]; 8], b: &mut [f64; 8]) -> [f64; 8] {
    let n = 8;
    for i in 0..n {
        // 选主元
        let mut max_row = i;
        for k in i + 1..n {
            if a[k][i].abs() > a[max_row][i].abs() {
                max_row = k;
            }
        }
        a.swap(i, max_row);
        b.swap(i, max_row);

        // 消元
        for k in i + 1..n {
            let c = a[k][i] / a[i][i];
            for j in i..n {
                a[k][j] -= c * a[i][j];
            }
            b[k] -= c * b[i];
        }
    }

    // 回代
    let mut x = [0.0; 8];
    for i in (0..n).rev() {
        x[i] = b[i];
        for j in i + 1..n {
            x[i] -= a[i][j] * x[j];
        }
        x[i] /= a[i][i];
    }
    x
}

/// 应用变换
fn perspective_transform_point2(mat: &[[f64; 3]; 3], pt: (f64, f64)) -> (f64, f64) {
    let x = pt.0;
    let y = pt.1;
    let w = mat[2][0] * x + mat[2][1] * y + mat[2][2];
    let xp = (mat[0][0] * x + mat[0][1] * y + mat[0][2]) / w;
    let yp = (mat[1][0] * x + mat[1][1] * y + mat[1][2]) / w;
    (xp, yp)
}

#[cfg(test)]
mod tests {
    use crate::nalgebra::{get_perspective_transform, get_perspective_transform2};
    use nalgebra::Matrix3;

    #[test]
    fn test_matrix3() {
        //创建一个单位矩阵
        let mat = Matrix3::<f64>::identity();

        //可视化输出
        // ┌       ┐
        // │ 1 0 0 │
        // │ 0 1 0 │
        // │ 0 0 1 │
        // └       ┘
        println!("{}", mat);

        //内存数组排列
        //[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
        println!("{:?}", mat);

        //内存数组排列
        //[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        println!("{:?}", mat.data.as_slice());
    }

    #[test]
    fn test_perspective_transform() {
        let src = [(13.0, 13.0), (166.5, 18.5), (163.0, 163.5), (10.5, 160.5)];
        let dst = [(20.0, 20.0), (160.0, 20.0), (160.0, 160.0), (20.0, 160.0)];
        let result = get_perspective_transform(&src, &dst);

        //可视化输出
        // ┌                                                                         ┐
        // │      0.8922067941243021     0.01418820853094108       8.176286297216732 │
        // │    -0.03583605628807174      0.9391478601047357       8.216367882118266 │
        // │ -0.00010755186200365022 -0.00004851993901597602                       1 │
        // └                                                                         ┘
        println!("{}", result);

        //内存数组排列
        //[[0.8922067941243021, -0.03583605628807174, -0.00010755186200365022],
        // [0.01418820853094108, 0.9391478601047357, -4.851993901597602e-5],
        // [8.176286297216732, 8.216367882118266, 1.0]]
        println!("{:?}", result);

        //内存数组排列
        //[0.8922067941243021, -0.03583605628807174, -0.00010755186200365022,
        // 0.01418820853094108, 0.9391478601047357, -4.851993901597602e-5,
        // 8.176286297216732, 8.216367882118266, 1.0]
        let data = result.data.as_slice();
        println!("{:?}", data);
    }

    #[test]
    fn test_perspective_transform2() {
        let src = [(13.0, 13.0), (166.5, 18.5), (163.0, 163.5), (10.5, 160.5)];
        let dst = [(20.0, 20.0), (160.0, 20.0), (160.0, 160.0), (20.0, 160.0)];
        let result = get_perspective_transform2(&src, &dst);

        //可视化输出
        // ┌                                                                         ┐
        // │      0.8922067941243021     0.01418820853094108       8.176286297216732 │
        // │    -0.03583605628807174      0.9391478601047357       8.216367882118266 │
        // │ -0.00010755186200365022 -0.00004851993901597602                       1 │
        // └                                                                         ┘
        //println!("{}", result);

        //可视化输出
        //[[0.8922067941243021, 0.014188208530941085, 8.176286297216732],
        // [-0.03583605628807174, 0.9391478601047357, 8.216367882118266],
        // [-0.00010755186200365022, -4.851993901597602e-5, 1.0]]
        println!("{:?}", result);
    }
}
