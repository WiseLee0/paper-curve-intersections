const GEOMETRIC_EPSILON: f64 = 1e-7;
const CURVETIME_EPSILON: f64 = 1e-8;
const FATLINE_EPSILON: f64 = 1e-9;
const EPSILON: f64 = 1e-12;
const MACHINE_EPSILON: f64 = 1.12e-16;

/// 分割贝塞尔曲线
pub fn split_cubic_bezier(bez: &[f64; 8], t: f64) -> ([f64; 8], [f64; 8]) {
    let (p1x, p1y, c1x, c1y, c2x, c2y, p2x, p2y) = (
        bez[0], bez[1], bez[2], bez[3], bez[4], bez[5], bez[6], bez[7],
    );

    let u = 1.0 - t;
    let p3x = u * p1x + t * c1x;
    let p3y = u * p1y + t * c1y;
    let p4x = u * c1x + t * c2x;
    let p4y = u * c1y + t * c2y;
    let p5x = u * c2x + t * p2x;
    let p5y = u * c2y + t * p2y;
    let p6x = u * p3x + t * p4x;
    let p6y = u * p3y + t * p4y;
    let p7x = u * p4x + t * p5x;
    let p7y = u * p4y + t * p5y;
    let p8x = u * p6x + t * p7x;
    let p8y = u * p6y + t * p7y;

    (
        [p1x, p1y, p3x, p3y, p6x, p6y, p8x, p8y],
        [p8x, p8y, p7x, p7y, p5x, p5y, p2x, p2y],
    )
}

/// 切割部分曲线[t1,t2]
pub fn split_cubic_bezier_part(v: &[f64; 8], t1: f64, t2: f64) -> [f64; 8] {
    let mut v_part = v.clone();

    if t1 > 0.0 {
        v_part = split_cubic_bezier(&v_part, t1).1;
    }
    if t2 < 1.0 {
        let t = (t2 - t1) / (1.0 - t1);
        v_part = split_cubic_bezier(&v_part, t).0;
    }

    v_part
}

/// 计算贝塞尔凸包
pub fn get_convex_hull(dq0: f64, dq1: f64, dq2: f64, dq3: f64) -> Vec<Vec<(f64, f64)>> {
    let p0 = (0.0, dq0);
    let p1 = (1.0 / 3.0, dq1);
    let p2 = (2.0 / 3.0, dq2);
    let p3 = (1.0, dq3);
    let dist1: f64 = dq1 - (2.0 * dq0 + dq3) / 3.0;
    let dist2: f64 = dq2 - (dq0 + 2.0 * dq3) / 3.0;

    let mut hull: Vec<Vec<(f64, f64)>>;

    if dist1 * dist2 < 0.0 {
        // 凸包包括两个三角形
        hull = vec![vec![p0, p1, p3], vec![p0, p2, p3]];
    } else {
        let dist_ratio = dist1 / dist2;

        if dist_ratio >= 2.0 {
            // 凸包包括一个三角形和一条线段
            hull = vec![vec![p0, p1, p3], vec![p0, p3]];
        } else if dist_ratio <= 0.5 {
            // 凸包包括一个三角形和一条线段
            hull = vec![vec![p0, p2, p3], vec![p0, p3]];
        } else {
            // 凸包包括一个四边形和一条线段
            hull = vec![vec![p0, p1, p2, p3], vec![p0, p3]];
        }
    }

    if (dist1 < 0.0) || (dist2 < 0.0) {
        hull.reverse();
    }

    hull
}

type Point = (f64, f64);
/// 凸包裁剪
fn clip_convex_hull(
    hull_top: &[Point],
    hull_bottom: &[Point],
    d_min: f64,
    d_max: f64,
) -> Option<f64> {
    if hull_top[0].1 < d_min {
        clip_convex_hull_part(hull_top, true, d_min)
    } else if hull_bottom[0].1 > d_max {
        clip_convex_hull_part(hull_bottom, false, d_max)
    } else {
        Some(hull_top[0].0)
    }
}

fn clip_convex_hull_part(part: &[Point], is_top: bool, threshold: f64) -> Option<f64> {
    let (mut prev_x, mut prev_y) = part[0];
    for &(current_x, current_y) in part.iter().skip(1) {
        if (is_top && current_y >= threshold) || (!is_top && current_y <= threshold) {
            if current_y == threshold {
                return Some(current_x);
            }
            return Some(
                prev_x + (threshold - prev_y) * (current_x - prev_x) / (current_y - prev_y),
            );
        }
        prev_x = current_x;
        prev_y = current_y;
    }
    None
}

/// Fat Line
fn get_fatline(v: &[f64]) -> Vec<f64> {
    let q0x = v[0];
    let q0y = v[1];
    let q3x = v[6];
    let q3y = v[7];
    let d1 = signed_distance(q0x, q0y, q3x, q3y, v[2], v[3], false).unwrap_or(0.0);
    let d2 = signed_distance(q0x, q0y, q3x, q3y, v[4], v[5], false).unwrap_or(0.0);
    let factor = if d1 * d2 > 0.0 { 3.0 / 4.0 } else { 4.0 / 9.0 };
    let d_min = factor * d1.min(d2).min(0.0);
    let d_max = factor * d1.max(d2).max(0.0);
    vec![d_min, d_max, d1, d2, factor]
}

fn signed_distance(
    px: f64,
    py: f64,
    mut vx: f64,
    mut vy: f64,
    x: f64,
    y: f64,
    as_vector: bool,
) -> Option<f64> {
    if !as_vector {
        vx -= px;
        vy -= py;
    }
    let result = if vx == 0.0 {
        if vy > 0.0 {
            x - px
        } else {
            px - x
        }
    } else if vy == 0.0 {
        if vx < 0.0 {
            y - py
        } else {
            py - y
        }
    } else {
        let dist = (x - px) * vy - (y - py) * vx;
        let denom = if vy > vx {
            vy * (1.0 + (vx * vx) / (vy * vy)).sqrt()
        } else {
            vx * (1.0 + (vy * vy) / (vx * vx)).sqrt()
        };
        dist / denom
    };
    Some(result)
}

fn is_zero(val: f64) -> bool {
    val >= -EPSILON && val <= EPSILON
}
/// 计算贝塞尔曲线上的点、切线、法线和曲率
///
/// type = 0时，计算曲线上参数t所对应的点
/// type = 1时，计算曲线上的切线
/// type = 2时，计算曲线上的法线
/// type = 3时，计算曲线上的曲率
pub fn evaluate(v: &[f64], t: f64, type_: u8) -> Vec<f64> {
    if t.is_nan() || t < 0.0 || t > 1.0 {
        return vec![];
    }
    let (mut x0, mut y0, mut x1, mut y1, mut x2, mut y2, x3, y3) =
        (v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]);

    if is_zero(x1 - x0) && is_zero(y1 - y0) {
        x1 = x0;
        y1 = y0;
    }
    if is_zero(x2 - x3) && is_zero(y2 - y3) {
        x2 = x3;
        y2 = y3;
    }

    let cx = 3.0 * (x1 - x0);
    let bx = 3.0 * (x2 - x1) - cx;
    let ax = x3 - x0 - cx - bx;
    let cy = 3.0 * (y1 - y0);
    let by = 3.0 * (y2 - y1) - cy;
    let ay = y3 - y0 - cy - by;

    let mut x;
    let mut y;

    match type_ {
        0 => {
            x = if t == 0.0 {
                x0
            } else if t == 1.0 {
                x3
            } else {
                ((ax * t + bx) * t + cx) * t + x0
            };
            y = if t == 0.0 {
                y0
            } else if t == 1.0 {
                y3
            } else {
                ((ay * t + by) * t + cy) * t + y0
            };
        }
        _ => {
            let t_min = CURVETIME_EPSILON;
            let t_max = 1.0 - t_min;

            if t < t_min {
                x = cx;
                y = cy;
            } else if t > t_max {
                x = 3.0 * (x3 - x2);
                y = 3.0 * (y3 - y2);
            } else {
                x = (3.0 * ax * t + 2.0 * bx) * t + cx;
                y = (3.0 * ay * t + 2.0 * by) * t + cy;
            }

            if type_ == 3 {
                let x2 = 6.0 * ax * t + 2.0 * bx;
                let y2 = 6.0 * ay * t + 2.0 * by;
                let d = (x * x + y * y).powf(1.5);
                x = if d != 0.0 { (x * y2 - y * x2) / d } else { 0.0 };
                y = 0.0;
            }
        }
    }

    match type_ {
        2 => vec![y, -x],
        _ => vec![x, y],
    }
}
