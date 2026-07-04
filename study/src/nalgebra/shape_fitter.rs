///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2026/07/03
/// 形状拟合
use geo::{BoundingRect, Coord, LineString, MinimumRotatedRect, Simplify};
use nalgebra::{DMatrix, DVector, Vector2};
use std::f64::consts::PI;

/// 定义基础点结构
#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    /// 计算两点间的欧氏距离
    #[inline]
    pub fn distance(&self, other: &Point2D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

/// 轨迹闭合性检测器
pub struct ClosureDetector;

impl ClosureDetector {
    /// 判断给定的连续点集是否构成闭合图形
    ///
    /// # Arguments
    /// * `points` - 轨迹点集
    /// * `threshold_ratio` - 容忍闭合的首尾相对误差比例（推荐值：0.15 即 15%）
    pub fn is_closed(points: &[Point2D], threshold_ratio: f64) -> bool {
        let n = points.len();
        if n < 3 {
            // 少于3个点无法构成有意义的闭合几何
            return false;
        }

        let last_point = &points[n - 1];

        // 1. 计算轨迹的 2D 包围盒 (AABB)
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;

        for p in points {
            if p.x < min_x {
                min_x = p.x;
            }
            if p.y < min_y {
                min_y = p.y;
            }
            if p.x > max_x {
                max_x = p.x;
            }
            if p.y > max_y {
                max_y = p.y;
            }
        }

        // 2. 计算包围盒对角线长度，作为当前图形的相对尺度基准
        let bbox_diagonal =
            Point2D { x: min_x, y: min_y }.distance(&Point2D { x: max_x, y: max_y });

        // 防御性编程：如果图形极小（比如只是在原地轻点了一下），直接视为闭合的“点”
        if bbox_diagonal < 1e-5 {
            return true;
        }

        let absolute_threshold = bbox_diagonal * threshold_ratio;

        // 3. 基础检测：首尾点距离
        let first_point = &points[0];
        if first_point.distance(last_point) <= absolute_threshold {
            return true;
        }

        // 4. 进阶检测：防过冲 (Overshoot) 与“画过头”现象
        // 用户画圆时，终点往往不在起点停下，而是越过了起点。
        // 我们取轨迹最前面的 10% 的点，如果终点落在这些点附近，也算闭合。
        let check_limit = (n as f64 * 0.1).ceil() as usize;
        let check_limit = check_limit.max(1); // 至少检查第一个点

        for i in 0..check_limit {
            if points[i].distance(last_point) <= absolute_threshold {
                return true;
            }
        }

        false
    }
}

/// 拟合结果枚举
/// 均方根误差 (RMSE, Root Mean Squared Error)。
#[derive(Debug)]
pub enum FittedShape {
    Line {
        slope: f64,
        intercept: f64,
        rmse: f64,
    },
    Circle {
        center_x: f64,
        center_y: f64,
        radius: f64,
        rmse: f64,
    },
    Rectangle {
        min_x: f64,
        min_y: f64,
        max_x: f64,
        max_y: f64,
        rmse: f64,
    },
    /// 带角度矩形 (包含4个按顺序排列的顶点及其 OBB 边的 RMSE)
    RotatedRectangle {
        vertices: Vec<Point2D>,
        rmse: f64,
    },
    /// 标准椭圆参数 (中心点、半长轴 A、半短轴 B、旋转弧度 angle、Sampson 近似 RMSE)
    Ellipse {
        center_x: f64,
        center_y: f64,
        axis_a: f64,
        axis_b: f64,
        angle: f64,
        rmse: f64,
    },
    Polygon {
        vertices: Vec<Point2D>,
    },
    // → 箭头结构(非V字箭头)：包含主干起点、箭头尖端、左翼尖、右翼尖以及均方根误差
    Arrow {
        shaft_start: Point2D,
        tip: Point2D,
        wing_left: Point2D,
        wing_right: Point2D,
        rmse: f64,
    },
    // 精准适配 3个点、2条边的 V 字形箭头
    VArrow {
        tip: Point2D,
        wing_left: Point2D,
        wing_right: Point2D,
        rmse: f64,
    },
    /// 五角形结构（包含五角形中心点、半径、旋转角度、5个顶点及其 OBB 边的 RMSE）
    Pentagon {
        center: Point2D,
        radius: f64,
        angle: f64,
        vertices: Vec<Point2D>,
        rmse: f64,
    },
    /// 心形结构
    Heart {
        center: Point2D,
        width: f64,
        height: f64,
        rmse: f64,
    },
    Error(String),
}

pub struct ShapeFitter;

impl ShapeFitter {
    /// --- 核心智能决策函数 ---
    /// 核心选择原理：残差评估与奥卡姆剃刀
    /// 在计算机视觉和模式识别中，评价点集与几何形状的拟合程度，
    /// 最标准的方法是计算每个原始点到拟合出的标准几何边缘的最短几何距离（正交残差）。
    ///
    /// - [polygon_epsilon]（多边形简化拟合度）：
    /// 通常设为 2.0 ~ 4.0。用于过滤手写多边形（如三角形、五角星）时手指的高频抖动。
    /// - [threshold]（形状判定硬阈值）：代表允许的最大平均像素偏差。
    /// 如果设为 10.0 ~ 15.0（像素）：比较严格。用户必须画得相对比较像圆或矩形，系统才会收敛为标准图形，否则会放行判定为多边形。
    /// 如果设为 30.0（像素）：非常宽松。哪怕用户画了一个极其扭曲、像鸭蛋一样的圈，系统也会强行把它矫正为标准的圆。
    pub fn classify_and_fit(
        points: &[Point2D],
        polygon_epsilon: f64,
        threshold: f64,
    ) -> FittedShape {
        if points.len() < 2 {
            return FittedShape::Error("点数过少，无法识别".to_string());
        }

        let mut best_shape = FittedShape::Error("无法拟合任何形状".to_string());
        let mut min_rmse = f64::MAX;
        //惩罚系数
        let mut penalty_factor = 1.0;

        // 1. 测试直线
        if let FittedShape::Line {
            slope,
            intercept,
            rmse,
        } = Self::fit_line(points)
        {
            if rmse < min_rmse {
                min_rmse = rmse;
                penalty_factor = 1.5;
                best_shape = FittedShape::Line {
                    slope,
                    intercept,
                    rmse,
                };
            }
        }

        // 2. 测试圆
        if points.len() >= 3 {
            if let FittedShape::Circle {
                center_x,
                center_y,
                radius,
                rmse,
            } = Self::fit_circle(points)
            {
                if rmse < min_rmse {
                    let circle_shape = FittedShape::Circle {
                        center_x,
                        center_y,
                        radius,
                        rmse,
                    };
                    let line_or_circle = match best_shape {
                        FittedShape::Line {
                            slope,
                            intercept,
                            rmse,
                        } => Self::arbitrate_line_vs_circle(points, best_shape, circle_shape, rmse),
                        _ => circle_shape,
                    };
                    best_shape = line_or_circle;
                    match best_shape {
                        FittedShape::Circle {
                            center_x,
                            center_y,
                            radius,
                            rmse,
                        } => {
                            min_rmse = rmse;
                        }
                        _ => {}
                    };
                }
            }
        }

        // 3. 测试矩形
        if let FittedShape::Rectangle {
            min_x,
            min_y,
            max_x,
            max_y,
            rmse,
        } = Self::fit_rectangle(points)
        {
            if rmse < min_rmse {
                min_rmse = rmse;
                best_shape = FittedShape::Rectangle {
                    min_x,
                    min_y,
                    max_x,
                    max_y,
                    rmse,
                };
            }
        }

        // ====== 核心触发：测试带角度矩形 ======
        if points.len() >= 3 {
            if let FittedShape::RotatedRectangle { vertices, rmse } =
                Self::fit_rotated_rectangle(points)
            {
                if rmse * penalty_factor < min_rmse {
                    min_rmse = rmse;
                    best_shape = FittedShape::RotatedRectangle { vertices, rmse };
                }
            }
        }

        // ====== 核心触发：测试椭圆 ======
        if points.len() >= 5 {
            if let FittedShape::Ellipse {
                center_x,
                center_y,
                axis_a,
                axis_b,
                angle,
                rmse,
            } = Self::fit_ellipse(points)
            {
                if rmse < min_rmse {
                    min_rmse = rmse;
                    best_shape = FittedShape::Ellipse {
                        center_x,
                        center_y,
                        axis_a,
                        axis_b,
                        angle,
                        rmse,
                    };
                }
            }
        }

        // 测试箭头 (通常传入稍微大一点的 epsilon 以提取核心骨架)
        if points.len() >= 4 {
            if let FittedShape::Arrow {
                shaft_start,
                tip,
                wing_left,
                wing_right,
                rmse,
            } = Self::fit_arrow(points, polygon_epsilon)
            {
                if rmse < min_rmse {
                    min_rmse = rmse;
                    best_shape = FittedShape::Arrow {
                        shaft_start,
                        tip,
                        wing_left,
                        wing_right,
                        rmse,
                    };
                }
            }
        }

        // 测试V箭头 (通常传入稍微大一点的 epsilon 以提取核心骨架)
        if points.len() >= 3 {
            if let FittedShape::VArrow {
                tip,
                wing_left,
                wing_right,
                rmse,
            } = Self::fit_v_arrow(points)
            {
                if rmse < min_rmse {
                    min_rmse = rmse;
                    best_shape = FittedShape::VArrow {
                        tip,
                        wing_left,
                        wing_right,
                        rmse,
                    };
                }
            }
        }

        if points.len() >= 5 {
            if let FittedShape::Pentagon {
                center,
                radius,
                angle,
                vertices,
                rmse,
            } = Self::fit_regular_pentagon(points)
            {
                if rmse < min_rmse {
                    min_rmse = rmse;
                    best_shape = FittedShape::Pentagon {
                        center,
                        radius,
                        angle,
                        vertices,
                        rmse,
                    };
                }
            }
        }

        if points.len() >= 8 {
            if let FittedShape::Heart {
                center,
                width,
                height,
                rmse,
            } = Self::fit_heart(points)
            {
                if rmse < min_rmse {
                    min_rmse = rmse;
                    best_shape = FittedShape::Heart {
                        center,
                        width,
                        height,
                        rmse,
                    };
                }
            }
        }

        // 仲裁：如果最接近的标准图形误差依然大于阈值(说明是个复杂的折线或多边形)
        if min_rmse > threshold {
            return Self::fit_polygon(points, polygon_epsilon);
        }

        best_shape
    }

    /// 解决圆与直线误判的核心裁决函数
    pub fn arbitrate_line_vs_circle(
        points: &[Point2D],
        line_shape: FittedShape,
        circle_shape: FittedShape,
        line_rmse: f64,
    ) -> FittedShape {
        if let FittedShape::Circle {
            center_x,
            center_y,
            radius,
            rmse: circle_rmse,
        } = circle_shape
        {
            let n = points.len();

            // 计算点集的外接矩形对角线
            let mut min_x = f64::MAX;
            let mut max_x = f64::MIN;
            let mut min_y = f64::MAX;
            let mut max_y = f64::MIN;
            for p in points {
                if p.x < min_x {
                    min_x = p.x;
                }
                if p.x > max_x {
                    max_x = p.x;
                }
                if p.y < min_y {
                    min_y = p.y;
                }
                if p.y > max_y {
                    max_y = p.y;
                }
            }
            let bbox_diagonal = ((max_x - min_x).powi(2) + (max_y - min_y).powi(2)).sqrt();

            // -------------------------------------------------------------
            // 【闸门 1】：半径边界检查
            // 如果半径比数据本身大好几倍，说明是直线引起的退化圆
            // -------------------------------------------------------------
            if radius > bbox_diagonal * 4.0 {
                return line_shape;
            }

            // -------------------------------------------------------------
            // 【闸门 2】：圆心角跨度检验 (Arc Span) -> 最关键的几何拓扑检查
            // -------------------------------------------------------------
            let mut angles: Vec<f64> = points
                .iter()
                .map(|p| (p.y - center_y).atan2(p.x - center_x))
                .collect();

            // 对方位角进行排序
            angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

            // 计算最大圆心角跨度（考虑跨越 -PI 和 PI 的边界情况）
            let mut max_span = 0.0;
            for i in 0..n {
                let diff = if i == n - 1 {
                    (angles[0] - angles[i] + 2.0 * PI) % (2.0 * PI)
                } else {
                    angles[i + 1] - angles[i]
                };
                if diff > max_span {
                    max_span = diff;
                }
            }
            // 总跨度 = 2PI - 最大相邻间隔
            let total_span = 2.0 * PI - max_span;

            // 工业级阈值设定：如果圆心角跨度小于 60 度 (PI / 3.0)
            // 意味着这组点在圆周上只占了极小的一条弧，极大概率是直线或微弱弧线
            if total_span < (PI / 3.0) {
                return line_shape;
            }

            // -------------------------------------------------------------
            // 【闸门 3】：带权重的残差比较 (Bayes/AIC 思想简化版)
            // 因为圆多了1个自由度，我们对圆的 RMSE 施加 1.5 倍的惩罚因子
            // -------------------------------------------------------------
            if circle_rmse * 1.5 < line_rmse {
                circle_shape
            } else {
                line_shape
            }
        } else {
            line_shape
        }
    }

    /// 1. 直线拟合 (最小二乘法 y = mx + c)
    pub fn fit_line(points: &[Point2D]) -> FittedShape {
        let n = points.len();
        let mut a_data = Vec::with_capacity(n * 2);
        let mut b_data = Vec::with_capacity(n);
        for p in points {
            a_data.push(p.x);
            a_data.push(1.0);
            b_data.push(p.y);
        }
        let a = DMatrix::from_row_slice(n, 2, &a_data);
        let b = DVector::from_column_slice(&b_data);

        if let Ok(theta) = a.svd(true, true).solve(&b, 1e-7) {
            let slope = theta[0];
            let intercept = theta[1];

            // 计算点到直线的 RMSE: d = |mx - y + c| / sqrt(m^2 + 1)
            let mut sum_sq_err = 0.0;
            let denom = (slope.powi(2) + 1.0).sqrt();
            for p in points {
                let d = (slope * p.x - p.y + intercept).abs() / denom;
                sum_sq_err += d.powi(2);
            }
            let rmse = (sum_sq_err / n as f64).sqrt();

            FittedShape::Line {
                slope,
                intercept,
                rmse,
            }
        } else {
            FittedShape::Error("直线拟合失败".to_string())
        }
    }

    /// 2. 圆拟合 (Kåsa 代数法)
    pub fn fit_circle(points: &[Point2D]) -> FittedShape {
        let n = points.len();
        let mut a_data = Vec::with_capacity(n * 3);
        let mut b_data = Vec::with_capacity(n);
        for p in points {
            a_data.push(2.0 * p.x);
            a_data.push(2.0 * p.y);
            a_data.push(1.0);
            b_data.push(p.x * p.x + p.y * p.y);
        }
        let a = DMatrix::from_row_slice(n, 3, &a_data);
        let b = DVector::from_column_slice(&b_data);

        if let Ok(theta) = a.svd(true, true).solve(&b, 1e-7) {
            let center_x = theta[0];
            let center_y = theta[1];
            let radius = (center_x.powi(2) + center_y.powi(2) + theta[2]).sqrt();

            // 计算点到圆边缘的 RMSE: d = ||P - C|| - R|
            let mut sum_sq_err = 0.0;
            for p in points {
                let dist_to_center = ((p.x - center_x).powi(2) + (p.y - center_y).powi(2)).sqrt();
                let d = (dist_to_center - radius).abs();
                sum_sq_err += d.powi(2);
            }
            let rmse = (sum_sq_err / n as f64).sqrt();

            FittedShape::Circle {
                center_x,
                center_y,
                radius,
                rmse,
            }
        } else {
            FittedShape::Error("圆拟合失败".to_string())
        }
    }

    /// 3. 矩形拟合 (基于坐标轴对齐的最小外接矩形 AABB)
    pub fn fit_rectangle(points: &[Point2D]) -> FittedShape {
        let coords: Vec<Coord<f64>> = points.iter().map(|p| Coord { x: p.x, y: p.y }).collect();
        let line_string = LineString::new(coords);

        if let Some(rect) = line_string.bounding_rect() {
            let min_x = rect.min().x;
            let min_y = rect.min().y;
            let max_x = rect.max().x;
            let max_y = rect.max().y;

            // 计算点到矩形边界(Shell)的 RMSE
            let mut sum_sq_err = 0.0;
            for p in points {
                let d = Self::point_to_rect_shell_dist(*p, min_x, min_y, max_x, max_y);
                sum_sq_err += d.powi(2);
            }
            let rmse = (sum_sq_err / points.len() as f64).sqrt();

            FittedShape::Rectangle {
                min_x,
                min_y,
                max_x,
                max_y,
                rmse,
            }
        } else {
            FittedShape::Error("矩形拟合失败".to_string())
        }
    }

    /// 辅助函数：计算点到轴对齐矩形外壳的最短几何距离
    fn point_to_rect_shell_dist(p: Point2D, min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> f64 {
        let closest_x = p.x.clamp(min_x, max_x);
        let closest_y = p.y.clamp(min_y, max_y);

        let dx = p.x - closest_x;
        let dy = p.y - closest_y;
        let ext_dist = (dx * dx + dy * dy).sqrt();

        if ext_dist > 1e-9 {
            // 点在矩形外部
            ext_dist
        } else {
            // 点在矩形内部，计算到四条边最近的距离
            let d_left = p.x - min_x;
            let d_right = max_x - p.x;
            let d_top = p.y - min_y;
            let d_bottom = max_y - p.y;
            d_left.min(d_right).min(d_top).min(d_bottom)
        }
    }

    /// 4. 多边形简化 (Douglas-Peucker 算法)
    /// epsilon 控制简化的程度，值越大，点越少，形状越抽象。
    pub fn fit_polygon(points: &[Point2D], epsilon: f64) -> FittedShape {
        if points.len() < 3 {
            return FittedShape::Error("需要至少3个点构成多边形".to_string());
        }

        let coords: Vec<Coord<f64>> = points.iter().map(|p| Coord { x: p.x, y: p.y }).collect();
        let line_string = LineString::new(coords);

        // 调用 geo 库自带的 DP 算法进行抽稀拟合
        let simplified = line_string.simplify(&epsilon);

        let vertices = simplified
            .into_iter()
            .map(|c| Point2D { x: c.x, y: c.y })
            .collect();

        FittedShape::Polygon { vertices }
    }

    /// 新增方法：箭头拟合
    pub fn fit_arrow(points: &[Point2D], epsilon: f64) -> FittedShape {
        let coords: Vec<Coord<f64>> = points.iter().map(|p| Coord { x: p.x, y: p.y }).collect();
        let line_string = LineString::new(coords);
        let simplified = line_string.simplify(&epsilon);
        let v: Vec<Point2D> = simplified
            .into_iter()
            .map(|c| Point2D { x: c.x, y: c.y })
            .collect();

        // 骨架点过少或过多均不符合常规手写单笔画箭头特征
        if v.len() < 4 || v.len() > 10 {
            return FittedShape::Error("拓扑结构不符合箭头骨架特征".to_string());
        }

        // 步骤1：寻找最长的一段线段作为主干 (Shaft)
        let mut max_len_sq = 0.0;
        let mut shaft_idx = 0;
        for i in 0..v.len() - 1 {
            let len_sq = (v[i].x - v[i + 1].x).powi(2) + (v[i].y - v[i + 1].y).powi(2);
            if len_sq > max_len_sq {
                max_len_sq = len_sq;
                shaft_idx = i;
            }
        }
        let p_a = v[shaft_idx];
        let p_b = v[shaft_idx + 1];

        // 步骤2：通过质心聚类判别主干的 尾端(Tail) 和 尖端(Tip)
        let mut center_other = Point2D { x: 0.0, y: 0.0 };
        let mut count = 0;
        for (i, p) in v.iter().enumerate() {
            if i != shaft_idx && i != (shaft_idx + 1) {
                center_other.x += p.x;
                center_other.y += p.y;
                count += 1;
            }
        }
        if count == 0 {
            return FittedShape::Error("缺乏翼片分支数据".to_string());
        }
        center_other.x /= count as f64;
        center_other.y /= count as f64;

        let dist_a = (p_a.x - center_other.x).powi(2) + (p_a.y - center_other.y).powi(2);
        let dist_b = (p_b.x - center_other.x).powi(2) + (p_b.y - center_other.y).powi(2);
        let (shaft_start, tip) = if dist_a > dist_b {
            (p_a, p_b)
        } else {
            (p_b, p_a)
        };

        // 步骤3：【核心修复 2】舍弃纯欧氏距离，改用有符号交叉积（法向投影高度）锁定真正的左右翼尖
        let shaft_vec = Vector2::new(tip.x - shaft_start.x, tip.y - shaft_start.y);
        let mut wing_l = tip;
        let mut wing_r = tip;
        let mut max_cross_l = 0.0; // 左边寻找正向最大横向距离
        let mut min_cross_r = 0.0; // 右边寻找负向最大横向距离

        for (i, p) in v.iter().enumerate() {
            if i == shaft_idx || i == (shaft_idx + 1) {
                continue;
            }
            let target_vec = Vector2::new(p.x - tip.x, p.y - tip.y);
            // 2D 叉积: cross = x1*y2 - y1*x2
            let cross = shaft_vec.x * target_vec.y - shaft_vec.y * target_vec.x;

            if cross > 1e-5 {
                // 严格位于主干左侧
                if cross > max_cross_l {
                    max_cross_l = cross;
                    wing_l = *p;
                }
            } else if cross < -1e-5 {
                // 严格位于主干右侧
                if cross < min_cross_r {
                    min_cross_r = cross;
                    wing_r = *p;
                }
            }
        }

        // 边界仲裁：如果单侧翼片横向扩张度为0，说明只是普通折线
        if max_cross_l == 0.0 || min_cross_r == 0.0 {
            return FittedShape::Error("未检测到对称的双翼侧展结构".to_string());
        }

        // 步骤4：多线段分段距离场计算 RMSE
        let mut sum_sq_err = 0.0;
        for p in points {
            let d_shaft = Self::point_to_segment_dist(*p, shaft_start, tip);
            let d_wing_l = Self::point_to_segment_dist(*p, tip, wing_l);
            let d_wing_r = Self::point_to_segment_dist(*p, tip, wing_r);
            let min_d = d_shaft.min(d_wing_l).min(d_wing_r);
            sum_sq_err += min_d.powi(2);
        }
        let rmse = (sum_sq_err / points.len() as f64).sqrt();

        FittedShape::Arrow {
            shaft_start,
            tip,
            wing_left: wing_l,
            wing_right: wing_r,
            rmse,
        }
    }

    /// 3点2边 V字形箭头高鲁棒性拟合
    pub fn fit_v_arrow(points: &[Point2D]) -> FittedShape {
        let n = points.len();
        if n < 3 {
            return FittedShape::Error("V字箭头拟合至少需要3个采样点".to_string());
        }

        // 1. 提取手写笔画的两端点作为两翼基础
        let p_start = points[0];
        let p_end = points[n - 1];

        // 2. 遍历中间点，寻找距离基线(p_start -> p_end)垂直距离最远的点作为箭尖(Tip)
        let mut max_dist = -1.0;
        let mut tip_idx = 0;

        for i in 1..n - 1 {
            let dist = Self::point_to_line_dist(points[i], p_start, p_end);
            if dist > max_dist {
                max_dist = dist;
                tip_idx = i;
            }
        }

        // 临界安全检查：如果最大距离几乎为0，说明用户画的是一条直线，而不是V字
        if max_dist < 1e-4 {
            return FittedShape::Error("点集近乎共线，无法构成V字折线箭头".to_string());
        }

        let tip = points[tip_idx];

        // 3. 区分左翼(Wing Left)与右翼(Wing Right)
        // 核心逻辑：计算箭头的整体物理朝向（从两翼中点指向箭尖）
        let mid_x = (p_start.x + p_end.x) / 2.0;
        let mid_y = (p_start.y + p_end.y) / 2.0;

        let dir_x = tip.x - mid_x;
        let dir_y = tip.y - mid_y;

        // 利用 2D 叉积 (Vector Cross Product) 判断 p_start 落在朝向向量的哪一侧
        let v_x = p_start.x - mid_x;
        let v_y = p_start.y - mid_y;
        let cross = dir_x * v_y - dir_y * v_x;

        // 根据右手定则裁定左右翼
        let (wing_left, wing_right) = if cross > 0.0 {
            (p_start, p_end)
        } else {
            (p_end, p_start)
        };

        // 4. 计算几何 RMSE（均方根误差）
        // 每个离散点到拟合出的两条边 (wing_left -> tip) 和 (wing_right -> tip) 的最短距离
        let mut sum_sq_err = 0.0;
        for p in points {
            let d_left = Self::point_to_segment_dist(*p, wing_left, tip);
            let d_right = Self::point_to_segment_dist(*p, wing_right, tip);
            let min_d = d_left.min(d_right);
            sum_sq_err += min_d.powi(2);
        }
        let rmse = (sum_sq_err / n as f64).sqrt();

        FittedShape::VArrow {
            tip,
            wing_left,
            wing_right,
            rmse,
        }
    }

    /// 辅助函数：计算点 P 到通过点 A 和 B 的无穷直线的绝对垂直距离
    fn point_to_line_dist(p: Point2D, a: Point2D, b: Point2D) -> f64 {
        let dx = b.x - a.x;
        let dy = b.y - a.y;
        let denominator = (dx * dx + dy * dy).sqrt();
        if denominator < 1e-9 {
            return ((p.x - a.x).powi(2) + (p.y - a.y).powi(2)).sqrt();
        }
        // 点到直线距离公式: |Ax + By + C| / sqrt(A^2 + B^2)
        let numerator = (dy * p.x - dx * p.y + b.x * a.y - b.y * a.x).abs();
        numerator / denominator
    }

    /// 辅助函数：计算点到线段的最短几何距离
    fn point_to_segment_dist(p: Point2D, s: Point2D, e: Point2D) -> f64 {
        let dx = e.x - s.x;
        let dy = e.y - s.y;
        let len_sq = dx * dx + dy * dy;
        if len_sq < 1e-9 {
            return ((p.x - s.x).powi(2) + (p.y - s.y).powi(2)).sqrt();
        }
        // 计算投影比例 t
        let t = ((p.x - s.x) * dx + (p.y - s.y) * dy) / len_sq;
        let t_clamped = t.clamp(0.0, 1.0);
        let proj_x = s.x + t_clamped * dx;
        let proj_y = s.y + t_clamped * dy;
        ((p.x - proj_x).powi(2) + (p.y - proj_y).powi(2)).sqrt()
    }

    /// 1. 实现带角度的矩形拟合 (OBB)
    pub fn fit_rotated_rectangle(points: &[Point2D]) -> FittedShape {
        if points.len() < 3 {
            return FittedShape::Error("至少需要3个点来拟合有角度矩形".to_string());
        }

        let coords: Vec<Coord<f64>> = points.iter().map(|p| Coord { x: p.x, y: p.y }).collect();
        let line_string = LineString::new(coords);

        // 调用 geo 库内置的高效旋转卡壳法计算最小外接矩形
        if let Some(poly) = line_string.minimum_rotated_rect() {
            let exterior = poly.exterior();
            let vertices: Vec<Point2D> = exterior
                .coords()
                .map(|c| Point2D { x: c.x, y: c.y })
                .collect();

            // 计算点集到这 4 条旋转边段的最小几何距离平方和
            let mut sum_sq_err = 0.0;
            for p in points {
                let mut min_d = f64::MAX;
                for i in 0..vertices.len() - 1 {
                    let d = Self::point_to_segment_dist(*p, vertices[i], vertices[i + 1]);
                    if d < min_d {
                        min_d = d;
                    }
                }
                sum_sq_err += min_d.powi(2);
            }
            let rmse = (sum_sq_err / points.len() as f64).sqrt();

            //计算矩形的宽高
            //let (w, h) = Self::calc_rect_size(vertices);

            // exterior 返回的顶点是闭合的(5个点，首尾相同)，截取前4个非重复顶点返回
            let final_vertices = vertices[0..4].to_vec();
            FittedShape::RotatedRectangle {
                vertices: final_vertices,
                rmse,
            }
        } else {
            FittedShape::Error("OBB 矩形计算失败".to_string())
        }
    }

    /// 2. 实现高鲁棒性椭圆拟合 (SVD 代数法 + Sampson 几何残差)
    pub fn fit_ellipse(points: &[Point2D]) -> FittedShape {
        let n = points.len();
        if n < 5 {
            return FittedShape::Error("椭圆拟合至少需要5个离散点".to_string());
        }

        let mut a_data = Vec::with_capacity(n * 5);
        let mut b_data = Vec::with_capacity(n);
        for p in points {
            a_data.push(p.x * p.x);
            a_data.push(p.x * p.y);
            a_data.push(p.y * p.y);
            a_data.push(p.x);
            a_data.push(p.y);
            b_data.push(-1.0); // 约束常数项 f = 1
        }

        let a_mat = DMatrix::from_row_slice(n, 5, &a_data);
        let b_vec = DVector::from_column_slice(&b_data);

        if let Ok(theta) = a_mat.svd(true, true).solve(&b_vec, 1e-7) {
            let a = theta[0];
            let b = theta[1];
            let c = theta[2];
            let d = theta[3];
            let e = theta[4];
            let f = 1.0;

            let discriminant = b * b - 4.0 * a * c;
            if discriminant >= 0.0 {
                return FittedShape::Error("退化或拟合出非椭圆二次曲线(抛物线/双曲线)".to_string());
            }

            // 【核心修复 1】：更正克莱姆法则推导的中心点坐标正负号
            let center_x = (2.0 * c * d - b * e) / discriminant;
            let center_y = (2.0 * a * e - b * d) / discriminant;

            // 计算长轴倾斜角
            let angle = 0.5 * b.atan2(a - c);

            // 重新计算椭圆中心化后的常数项 f_prime
            let f_prime = a * center_x * center_x
                + b * center_x * center_y
                + c * center_y * center_y
                + d * center_x
                + e * center_y
                + f;

            // 利用二次型矩阵本征值求解半轴长
            let trace = a + c;
            let diff = a - c;
            let term = (diff * diff + b * b).sqrt();
            let lambda1 = 0.5 * (trace + term);
            let lambda2 = 0.5 * (trace - term);

            // 【安全性检查】：确保分式合法，防止对负数开方导致 NaN
            let check_a = -f_prime / lambda1;
            let check_b = -f_prime / lambda2;
            if check_a < 0.0 || check_b < 0.0 {
                return FittedShape::Error("几何拓扑虚化，无法构成实数椭圆".to_string());
            }

            let axis_a = check_a.sqrt();
            let axis_b = check_b.sqrt();

            // Sampson 距离逼近几何 RMSE
            let mut sum_sq_err = 0.0;
            for p in points {
                let f_val = a * p.x * p.x + b * p.x * p.y + c * p.y * p.y + d * p.x + e * p.y + f;
                let grad_x = 2.0 * a * p.x + b * p.y + d;
                let grad_y = b * p.x + 2.0 * c * p.y + e;
                let grad_norm_sq = grad_x * grad_x + grad_y * grad_y;

                let sampson_dist = if grad_norm_sq > 1e-9 {
                    f_val.abs() / grad_norm_sq.sqrt()
                } else {
                    0.0
                };
                sum_sq_err += sampson_dist.powi(2);
            }
            let rmse = (sum_sq_err / n as f64).sqrt();

            FittedShape::Ellipse {
                center_x,
                center_y,
                axis_a,
                axis_b,
                angle,
                rmse,
            }
        } else {
            FittedShape::Error("矩阵奇异，椭圆系数求解失败".to_string())
        }
    }

    /// 正五边形高鲁棒性快速拟合
    pub fn fit_regular_pentagon(points: &[Point2D]) -> FittedShape {
        let n = points.len();
        if n < 5 {
            return FittedShape::Error("数据点太少，无法拟合五边形".to_string());
        }

        // 步骤 A: 计算几何质心 (Centroid)
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        for p in points {
            sum_x += p.x;
            sum_y += p.y;
        }
        let center = Point2D {
            x: sum_x / n as f64,
            y: sum_y / n as f64,
        };

        // 步骤 B: 寻找距离质心最远的点，作为外接圆半径 R 和初始旋转角 angle 的基准
        let mut max_r_sq = -1.0;
        let mut farest_point = points[0];
        for p in points {
            let r_sq = (p.x - center.x).powi(2) + (p.y - center.y).powi(2);
            if r_sq > max_r_sq {
                max_r_sq = r_sq;
                farest_point = *p;
            }
        }
        let radius = max_r_sq.sqrt();
        let base_angle = (farest_point.y - center.y).atan2(farest_point.x - center.x);

        // 步骤 C: 根据正五边形对称性，生成5个顶点
        let mut vertices = Vec::with_capacity(5);
        for i in 0..5 {
            let angle = base_angle + (i as f64) * (2.0 * PI / 5.0);
            vertices.push(Point2D {
                x: center.x + radius * angle.cos(),
                y: center.y + radius * angle.sin(),
            });
        }

        // 步骤 D: 计算所有原始点到五条边的最短几何距离的 RMSE
        let mut sum_sq_err = 0.0;
        for p in points {
            let mut min_edge_dist = f64::MAX;
            for j in 0..5 {
                let d = Self::point_to_segment_dist(*p, vertices[j], vertices[(j + 1) % 5]);
                if d < min_edge_dist {
                    min_edge_dist = d;
                }
            }
            sum_sq_err += min_edge_dist.powi(2);
        }
        let rmse = (sum_sq_err / n as f64).sqrt();

        FittedShape::Pentagon {
            center,
            radius,
            angle: base_angle,
            vertices,
            rmse,
        }
    }

    /// 心形曲线空间映射拟合
    pub fn fit_heart(points: &[Point2D]) -> FittedShape {
        let n = points.len();
        if n < 8 {
            return FittedShape::Error("数据点太少，无法拟合心形".to_string());
        }

        // 步骤 A: 计算点集的外接矩形 (AABB)
        let mut min_x = f64::MAX;
        let mut max_x = f64::MIN;
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;
        for p in points {
            if p.x < min_x {
                min_x = p.x;
            }
            if p.x > max_x {
                max_x = p.x;
            }
            if p.y < min_y {
                min_y = p.y;
            }
            if p.y > max_y {
                max_y = p.y;
            }
        }
        let width = max_x - min_x;
        let height = max_y - min_y;
        let center = Point2D {
            x: (min_x + max_x) / 2.0,
            y: (min_y + max_y) / 2.0,
        };

        if width < 2.0 || height < 2.0 {
            return FittedShape::Error("图形太小，无法构成有效心形".to_string());
        }

        // 步骤 B: 离散化标准心形方程骨架 (使用64个点连成骨架线段闭环)
        const SAMPLE_COUNT: usize = 64;
        let mut heart_skeleton = Vec::with_capacity(SAMPLE_COUNT);

        // 标准方程的缩放因子
        let scale_x = width / 32.0;
        let scale_y = height / 29.0;
        // 标准方程的中心 y 坐标偏置在大约 -2.5 处，为了对齐 AABB 中心进行微调
        let y_offset = 2.5 * scale_y;

        for i in 0..SAMPLE_COUNT {
            let t = (i as f64) * (2.0 * PI / SAMPLE_COUNT as f64);

            // 经典心形参数公式
            let std_x = 16.0 * t.sin().powi(3);
            let std_y = 13.0 * t.cos() - 5.0 * 2.0 * t.cos() - 2.0 * 3.0 * t.cos() - 4.0 * t.cos();
            // 注：原方程为了在传统数学坐标系（Y轴向上）开口朝上，如果是屏幕坐标系（Y轴向下），
            // 请根据实际渲染体系决定是否对 std_y 取反。这里假设为标准数学坐标系。
            let std_y_fixed =
                13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos();

            // 映射到用户点集空间
            heart_skeleton.push(Point2D {
                x: center.x + std_x * scale_x,
                y: center.y - std_y_fixed * scale_y + y_offset, // 减法用于匹配常规数学坐标系朝向
            });
        }

        // 步骤 C: 计算手写输入点到心形骨架的离散 RMSE
        let mut sum_sq_err = 0.0;
        for p in points {
            let mut min_segment_dist = f64::MAX;
            for j in 0..SAMPLE_COUNT {
                let d = Self::point_to_segment_dist(
                    *p,
                    heart_skeleton[j],
                    heart_skeleton[(j + 1) % SAMPLE_COUNT],
                );
                if d < min_segment_dist {
                    min_segment_dist = d;
                }
            }
            sum_sq_err += min_segment_dist.powi(2);
        }
        let rmse = (sum_sq_err / n as f64).sqrt();

        FittedShape::Heart {
            center,
            width,
            height,
            rmse,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::nalgebra::shape_fitter::{ClosureDetector, FittedShape, Point2D, ShapeFitter};

    /// 测试笔迹是否闭合
    #[test]
    fn test_is_closed() {
        //该手写轨迹是否闭合: true
        /*let points = vec![
            Point2D { x: 10.0, y: 10.0 },
            Point2D { x: 15.0, y: 12.0 },
            Point2D { x: 20.0, y: 20.0 },
            Point2D { x: 25.0, y: 25.0 },
            Point2D { x: 10.0, y: 10.0 },
        ];*/
        //该手写轨迹是否闭合: false
        let points = vec![
            Point2D { x: 10.0, y: 10.0 },
            Point2D { x: 15.0, y: 12.0 },
            Point2D { x: 20.0, y: 20.0 },
            Point2D { x: 25.0, y: 25.0 },
            Point2D { x: 12.0, y: 15.0 },
        ];
        // 设置容忍比例为 15% (0.15)
        let is_closed = ClosureDetector::is_closed(&points, 0.15);
        println!("该手写轨迹是否闭合: {}", is_closed);
    }

    #[test]
    fn test_shape_fitter_line() {
        //Line { slope: 1.0599999999999974, intercept: -1.7999999999999525 }
        /*let line_points = vec![
            Point2D { x: 10.0, y: 10.0 },
            Point2D { x: 15.0, y: 12.0 },
            Point2D { x: 20.0, y: 20.0 },
            Point2D { x: 25.0, y: 25.0 },
        ];*/
        //Line { slope: 0.9999999999999979, intercept: 4.4346470939871097e-14 }
        /*let line_points = vec![
            Point2D { x: 10.0, y: 10.0 },
            Point2D { x: 15.0, y: 15.0 },
            Point2D { x: 20.0, y: 20.0 },
            Point2D { x: 25.0, y: 25.0 },
        ];*/
        //Line { slope: 0.3999999999999981, intercept: 6.000000000000036 }
        /*let line_points = vec![
            Point2D { x: 10.0, y: 10.0 },
            Point2D { x: 15.0, y: 12.0 },
            Point2D { x: 20.0, y: 14.0 },
            Point2D { x: 25.0, y: 16.0 },
        ];*/
        //Line { slope: 0.799999999999998, intercept: 1.00000000000004 }
        let line_points = vec![
            Point2D { x: 10.0, y: 10.0 },
            Point2D { x: 15.0, y: 10.0 },
            Point2D { x: 20.0, y: 20.0 },
            Point2D { x: 25.0, y: 20.0 },
        ];
        if let FittedShape::Line {
            slope,
            intercept,
            rmse,
        } = ShapeFitter::fit_line(&read_points())
        {
            let x = line_points.first().unwrap().x;
            let y = x * slope + intercept;
            println!("x = {} y = {}", x, y);

            let x = line_points.last().unwrap().x;
            let y = x * slope + intercept;
            println!("x = {} y = {}", x, y);

            println!("{},{}", slope, intercept);
        }
    }
    #[test]
    fn test_shape_fitter_circle() {
        //Circle { center_x: 8.880952380952351, center_y: 25.404761904761926, radius: 14.735344769890153 }
        /*let circle_points = vec![
            Point2D { x: 10.0, y: 10.0 },
            Point2D { x: 15.0, y: 12.0 },
            Point2D { x: 20.0, y: 20.0 },
            Point2D { x: 25.0, y: 25.0 },
        ];*/
        //Circle { center_x: 15.000000000000005, center_y: 20.277777777777768, radius: 11.05890513188576 }
        //Circle { center_x: 17.499999999999996, center_y: -8.728179551122192, radius: 19.544684937319182 }
        let circle_points = vec![
            Point2D { x: 10.0, y: 10.0 },
            Point2D { x: 15.0, y: 10.0 },
            Point2D { x: 20.0, y: 10.0 },
            Point2D { x: 25.0, y: 10.0 },
        ];
        if let FittedShape::Circle {
            center_x,
            center_y,
            radius,
            rmse,
        } = ShapeFitter::fit_circle(&read_points())
        {
            println!(
                "center_x = {} center_y = {} radius = {}",
                center_x, center_y, radius
            );
            println!("{},{},{}", center_x, center_y, radius);
        }
    }

    #[test]
    fn test_shape_fitter_rectangle() {
        //Rectangle { min_x: 10.0, min_y: 10.0, max_x: 25.0, max_y: 16.0 }
        let rectangle_points = vec![
            Point2D { x: 10.0, y: 10.0 },
            Point2D { x: 15.0, y: 12.0 },
            Point2D { x: 20.0, y: 14.0 },
            Point2D { x: 25.0, y: 16.0 },
        ];
        if let FittedShape::Rectangle {
            min_x,
            min_y,
            max_x,
            max_y,
            rmse,
        } = ShapeFitter::fit_rectangle(&read_points())
        {
            println!(
                "min_x = {} min_y = {} max_x = {} max_y = {}",
                min_x, min_y, max_x, max_y
            );
            println!("{},{},{},{}", min_x, min_y, max_x, max_y);
        }
    }

    #[test]
    fn test_shape_fitter_polygon() {
        //Polygon { vertices: [Point2D { x: 10.0, y: 10.0 }, Point2D { x: 25.0, y: 16.0 }, Point2D { x: 10.0, y: 20.0 }] }
        /*let polygon_points = vec![
            Point2D { x: 10.0, y: 10.0 },
            Point2D { x: 15.0, y: 12.0 },
            Point2D { x: 20.0, y: 14.0 },
            Point2D { x: 25.0, y: 16.0 },
            Point2D { x: 10.0, y: 20.0 },
        ];*/
        //Polygon { vertices: [Point2D { x: 10.0, y: 10.0 }, Point2D { x: 15.0, y: 15.0 }, Point2D { x: 12.0, y: 17.0 }, Point2D { x: 10.0, y: 20.0 }] }
        let polygon_points = vec![
            Point2D { x: 10.0, y: 10.0 },
            Point2D { x: 12.0, y: 12.0 },
            Point2D { x: 15.0, y: 15.0 },
            Point2D { x: 12.0, y: 17.0 },
            Point2D { x: 10.0, y: 20.0 },
        ];
        if let FittedShape::Polygon { vertices } = ShapeFitter::fit_polygon(&read_points(), 1.0) {
            //拼接字符串
            let mut s = String::new();
            for vertex in vertices {
                println!("x = {} y = {}", vertex.x, vertex.y);
                s.push_str(&format!("{},{},", vertex.x, vertex.y));
            }
            println!("{}", s);
        }
    }
    #[test]
    fn test_classify_and_fit() {
        let points = read_points();
        let result = ShapeFitter::classify_and_fit(&points, 2.0, 15.0);
        println!("{:?}", result);
        match result {
            FittedShape::Line {
                slope,
                intercept,
                rmse,
            } => {
                let x = points.first().unwrap().x;
                let y = x * slope + intercept;
                println!("x = {} y = {}", x, y);

                let x = points.last().unwrap().x;
                let y = x * slope + intercept;
                println!("x = {} y = {}", x, y);

                println!("{},{}", slope, intercept);
            }
            FittedShape::Circle {
                center_x,
                center_y,
                radius,
                rmse,
            } => {
                println!(
                    "center_x = {} center_y = {} radius = {}",
                    center_x, center_y, radius
                );
                println!("{},{},{}", center_x, center_y, radius);
            }
            FittedShape::Rectangle {
                min_x,
                min_y,
                max_x,
                max_y,
                rmse,
            } => {
                println!(
                    "min_x = {} min_y = {} max_x = {} max_y = {}",
                    min_x, min_y, max_x, max_y
                );
                println!("{},{},{},{}", min_x, min_y, max_x, max_y);
            }
            FittedShape::RotatedRectangle { vertices, rmse } => {
                let mut s = String::new();
                for vertex in vertices {
                    println!("x = {} y = {}", vertex.x, vertex.y);
                    s.push_str(&format!("{},{},", vertex.x, vertex.y));
                }
                println!("{}", s);
            }
            FittedShape::VArrow {
                tip,
                wing_left,
                wing_right,
                rmse,
            } => {
                println!(
                    "{},{},{},{},{},{}",
                    tip.x, tip.y, wing_left.x, wing_left.y, wing_right.x, wing_right.y
                );
            }
            FittedShape::Ellipse {
                center_x,
                center_y,
                axis_a,
                axis_b,
                angle,
                rmse,
            } => {
                println!(
                    "center_x = {} center_y = {} axis_a = {} axis_b = {} angle = {}",
                    center_x, center_y, axis_a, axis_b, angle
                );
                println!("{},{},{},{},{}", center_x, center_y, axis_a, axis_b, angle);
            }
            FittedShape::Polygon { vertices } => {
                //拼接字符串
                let mut s = String::new();
                for vertex in vertices {
                    println!("x = {} y = {}", vertex.x, vertex.y);
                    s.push_str(&format!("{},{},", vertex.x, vertex.y));
                }
                println!("{}", s);
            }
            _ => {}
        }
    }

    fn read_points() -> Vec<Point2D> {
        /*let str = "479.6,406.4
468.4,396.0
454.79999999999995,376.8
442.0,352.8
438.79999999999995,343.2
425.20000000000005,238.39999999999998
426.0,219.2
438.0,186.4
487.6,179.2
519.6,201.6
542.0,223.2
558.0,244.0
566.8,257.6
590.8,318.4
594.8,339.2
597.2,368.0
591.6,401.6
580.4,417.6
553.2,440.0
527.6,446.4
509.20000000000005,447.2
495.6,446.4
482.79999999999995,441.6
459.6,428.0
446.0,415.2
428.4,396.8
420.4,385.6
414.79999999999995,375.2
408.4,364.0";*/
        let str = "530.8,547.2
507.6,536.0
482.79999999999995,523.2
461.20000000000005,508.0
450.0,498.4
437.20000000000005,485.6
430.0,478.4
422.79999999999995,468.0
413.20000000000005,448.0
410.0,426.4
410.0,416.0
418.0,397.6
426.79999999999995,388.8
438.0,380.8
455.6,370.4
492.4,352.8
509.20000000000005,346.4
534.8,337.6
558.8,329.6
582.0,321.6
616.4,312.8
638.0,305.6
667.6,298.4
689.2,294.4
710.8,290.4
731.5999999999999,288.8
743.5999999999999,288.8
754.0,288.8
765.2,288.8
790.0,291.2
817.2,295.2
845.2,303.2
854.8,306.4
879.5999999999999,321.6
891.5999999999999,329.6
899.5999999999999,336.8
907.5999999999999,345.6
917.2,357.6
923.5999999999999,368.0
928.4000000000001,377.6
934.8,390.4
940.4000000000001,406.4
942.0,419.2
942.0,429.6
937.2,442.4
932.4000000000001,454.4
926.8,463.2
912.4000000000001,482.4
902.8,493.6
888.4000000000001,506.4
879.5999999999999,512.8
860.4000000000001,524.8
845.2,532.0
829.2,539.2
818.0,542.4
801.2,547.2
782.0,550.4
764.4000000000001,554.4
747.5999999999999,559.2
734.0,561.6
722.0,564.0
696.4,568.8
679.6,571.2
663.6,573.6
652.4,573.6
638.8,573.6
625.2,573.6
603.6,570.4
589.2,568.8
578.0,566.4
567.6,564.0
558.0,560.8
549.2,556.0
539.6,549.6
530.8,544.8
521.2,541.6";
        let points: Vec<Point2D> = str
            .split("\n")
            .map(|s| {
                let v: Vec<&str> = s.split(",").collect();
                Point2D {
                    x: v[0].parse::<f64>().unwrap(),
                    y: v[1].parse::<f64>().unwrap(),
                }
            })
            .collect();
        points
    }
}
