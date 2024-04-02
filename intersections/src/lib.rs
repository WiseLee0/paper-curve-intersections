extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
mod collision_detection;
use collision_detection::CollisionDetection;
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
pub fn get_convex_hull(
    dq0: f64,
    dq1: f64,
    dq2: f64,
    dq3: f64,
) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    let p0 = (0.0, dq0);
    let p1 = (1.0 / 3.0, dq1);
    let p2 = (2.0 / 3.0, dq2);
    let p3 = (1.0, dq3);
    let dist1: f64 = dq1 - (2.0 * dq0 + dq3) / 3.0;
    let dist2: f64 = dq2 - (dq0 + 2.0 * dq3) / 3.0;

    let hull: (Vec<(f64, f64)>, Vec<(f64, f64)>);

    if dist1 * dist2 < 0.0 {
        // 凸包包括两个三角形
        hull = (vec![p0, p1, p3], vec![p0, p2, p3]);
    } else {
        let dist_ratio = dist1 / dist2;

        if dist_ratio >= 2.0 {
            // 凸包包括一个三角形和一条线段
            hull = (vec![p0, p1, p3], vec![p0, p3]);
        } else if dist_ratio <= 0.5 {
            // 凸包包括一个三角形和一条线段
            hull = (vec![p0, p2, p3], vec![p0, p3]);
        } else {
            // 凸包包括一个四边形和一条线段
            hull = (vec![p0, p1, p2, p3], vec![p0, p3]);
        }
    }

    if dist1 > 0.0 {
        return hull;
    }
    if dist1 < 0.0 {
        return (hull.1, hull.0);
    }
    if dist2 < 0.0 {
        return (hull.1, hull.0);
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
                prev_x + ((threshold - prev_y) * (current_x - prev_x)) / (current_y - prev_y),
            );
        }
        prev_x = current_x;
        prev_y = current_y;
    }
    None
}

/// Fat Line
fn get_fatline(v: &[f64]) -> (f64, f64, f64, f64) {
    let q0x = v[0];
    let q0y = v[1];
    let q3x = v[6];
    let q3y = v[7];
    let d1 = signed_distance(q0x, q0y, q3x, q3y, v[2], v[3], false);
    let d2 = signed_distance(q0x, q0y, q3x, q3y, v[4], v[5], false);
    let factor = if d1 * d2 > 0.0 { 3.0 / 4.0 } else { 4.0 / 9.0 };
    let d_min = factor * d1.min(d2).min(0.0);
    let d_max = factor * d1.max(d2).max(0.0);
    (d_min, d_max, d1, d2)
}

fn signed_distance(
    px: f64,
    py: f64,
    mut vx: f64,
    mut vy: f64,
    x: f64,
    y: f64,
    as_vector: bool,
) -> f64 {
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
    result
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
pub fn evaluate(v: &[f64], t: f64, type_: u8) -> Option<[f64; 2]> {
    if t.is_nan() || t < 0.0 || t > 1.0 {
        return None;
    }
    let (x0, y0, mut x1, mut y1, mut x2, mut y2, x3, y3) =
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
        2 => Some([y, -x]),
        _ => Some([x, y]),
    }
}

fn is_machine_zero(val: f64) -> bool {
    val >= -MACHINE_EPSILON && val <= MACHINE_EPSILON
}

/// 计算直线相交
pub fn line_intersection(
    p1x: f64,
    p1y: f64,
    mut v1x: f64,
    mut v1y: f64,
    p2x: f64,
    p2y: f64,
    mut v2x: f64,
    mut v2y: f64,
) -> Option<(f64, f64)> {
    v1x -= p1x;
    v1y -= p1y;
    v2x -= p2x;
    v2y -= p2y;
    let cross = v1x * v2y - v1y * v2x;
    if !is_machine_zero(cross) {
        let dx = p1x - p2x;
        let dy = p1y - p2y;
        let u1 = (v2x * dy - v2y * dx) / cross;
        let u2 = (v1x * dy - v1y * dx) / cross;
        let epsilon = MACHINE_EPSILON;
        let u_min = -epsilon;
        let u_max = 1.0 + epsilon;
        if u_min < u1 && u1 < u_max && u_min < u2 && u2 < u_max {
            let t = if u1 <= 0.0 {
                0.0
            } else if u1 >= 1.0 {
                1.0
            } else {
                u1
            };
            return Some((p1x + t * v1x, p1y + t * v1y));
        }
    }
    None
}

fn bezier_coeffs(p0: f64, p1: f64, p2: f64, p3: f64) -> [f64; 4] {
    [
        -p0 + 3.0 * p1 - 3.0 * p2 + p3,
        3.0 * p0 - 6.0 * p1 + 3.0 * p2,
        -3.0 * p0 + 3.0 * p1,
        p0,
    ]
}
fn sgn(x: f64) -> f64 {
    if x < 0.0 {
        -1.0
    } else {
        1.0
    }
}
fn sort_special(mut a: Vec<f64>) -> Vec<f64> {
    let mut flipped = true;
    while flipped {
        flipped = false;
        for i in 0..a.len() - 1 {
            if (a[i + 1] >= 0.0 && a[i] > a[i + 1]) || (a[i] < 0.0 && a[i + 1] >= 0.0) {
                a.swap(i, i + 1);
                flipped = true;
            }
        }
    }
    a
}
fn cubic_roots(p: &[f64; 4]) -> Vec<f64> {
    let a = p[1] / p[0];
    let b = p[2] / p[0];
    let c = p[3] / p[0];

    let q = (3.0 * b - a * a) / 9.0;
    let r = (9.0 * a * b - 27.0 * c - 2.0 * a * a * a) / 54.0;
    let d = q * q * q + r * r;

    let mut t = vec![-1.0; 3];

    if d >= 0.0 {
        let d_sqrt = d.sqrt();
        let s = sgn(r + d_sqrt) * (r + d_sqrt).abs().powf(1.0 / 3.0);
        let t_val = sgn(r - d_sqrt) * (r - d_sqrt).abs().powf(1.0 / 3.0);

        t[0] = -a / 3.0 + (s + t_val); // real root
        t[1] = -a / 3.0 - (s + t_val) / 2.0; // real part of complex root
        t[2] = -a / 3.0 - (s + t_val) / 2.0; // real part of complex root
        let im = ((3.0_f64).sqrt() * (s - t_val)) / 2.0; // complex part of root pair

        if im != 0.0 {
            t[1] = -1.0;
            t[2] = -1.0;
        }
    } else {
        let q_sqrt = (-q).sqrt();
        let q_t = (-q.powi(3)).sqrt();
        let th = (r / q_t).acos();

        t[0] = 2.0 * q_sqrt * (th / 3.0).cos() - a / 3.0;
        t[1] = 2.0 * q_sqrt * ((th + 2.0 * std::f64::consts::PI) / 3.0).cos() - a / 3.0;
        t[2] = 2.0 * q_sqrt * ((th + 4.0 * std::f64::consts::PI) / 3.0).cos() - a / 3.0;
    }

    for i in 0..3 {
        if t[i] < 0.0 || t[i] > 1.0 {
            t[i] = -1.0;
        }
    }

    sort_special(t)
}

/// 计算直线和曲线相交
fn line_and_curve_intersection(v: &[f64], line: &[f64]) -> Vec<(f64, f64, f64, i32, f64, f64)> {
    let px = [v[0], v[2], v[4], v[6]];
    let py = [v[1], v[3], v[5], v[7]];
    let lx = [line[0], line[2]];
    let ly = [line[1], line[3]];
    let a = ly[1] - ly[0]; // A = y2 - y1
    let b = lx[0] - lx[1]; // B = x1 - x2
    let c = lx[0] * (ly[0] - ly[1]) + ly[0] * (lx[1] - lx[0]); // C = x1 * (y1 - y2) + y1 * (x2 - x1)

    let bx = bezier_coeffs(px[0], px[1], px[2], px[3]); // Implement this function
    let by = bezier_coeffs(py[0], py[1], py[2], py[3]); // Implement this function

    let p = [
        a * bx[0] + b * by[0],     // t^3
        a * bx[1] + b * by[1],     // t^2
        a * bx[2] + b * by[2],     // t
        a * bx[3] + b * by[3] + c, // 1
    ];

    let r = cubic_roots(&p);
    let mut res = Vec::new();
    for &t in &r {
        let intersection_x = bx[0] * t.powi(3) + bx[1] * t.powi(2) + bx[2] * t + bx[3];
        let intersection_y = by[0] * t.powi(3) + by[1] * t.powi(2) + by[2] * t + by[3];

        let s: f64;
        if lx[1] != lx[0] {
            s = (intersection_x - lx[0]) / (lx[1] - lx[0]);
        } else {
            s = (intersection_y - ly[0]) / (ly[1] - ly[0]);
        }

        if !(t < 0.0 || t > 1.0 || s < 0.0 || s > 1.0) {
            res.push((
                t,
                intersection_x,
                intersection_y,
                -1,
                intersection_x,
                intersection_y,
            ));
        }
    }
    res
}

fn bezier_intersections(
    v1: &[f64; 8],
    v2: &[f64; 8],
    locations: &mut Vec<[f64; 6]>,
    flip: bool,
    mut recursion: u8,
    mut calls: u16,
    t_min: f64,
    t_max: f64,
    u_min: f64,
    u_max: f64,
) -> u16 {
    calls += 1;
    recursion += 1;
    if calls >= 4096 || recursion >= 40 {
        return calls;
    }

    let fat_line_epsilon = FATLINE_EPSILON;
    let (q0x, q0y, q3x, q3y) = (v2[0], v2[1], v2[6], v2[7]);
    let (d_min, d_max, d1, d2) = get_fatline(v2);
    let dp0 = signed_distance(q0x, q0y, q3x, q3y, v1[0], v1[1], false);
    let dp1 = signed_distance(q0x, q0y, q3x, q3y, v1[2], v1[3], false);
    let dp2 = signed_distance(q0x, q0y, q3x, q3y, v1[4], v1[5], false);
    let dp3 = signed_distance(q0x, q0y, q3x, q3y, v1[6], v1[7], false);
    let (top, bottom) = get_convex_hull(dp0, dp1, dp2, dp3);

    if d1 == 0.0 && d2 == 0.0 && dp0 == 0.0 && dp1 == 0.0 && dp2 == 0.0 && dp3 == 0.0 {
        return calls;
    }

    let t_min_clip = clip_convex_hull(&top, &bottom, d_min, d_max);
    let t_max_clip = clip_convex_hull(
        &top.iter().rev().cloned().collect::<Vec<_>>(),
        &bottom.iter().rev().cloned().collect::<Vec<_>>(),
        d_min,
        d_max,
    );

    let t_min_clip = match t_min_clip {
        Some(clip) => clip,
        None => return calls,
    };

    let t_max_clip = match t_max_clip {
        Some(clip) => clip,
        None => return calls,
    };

    let t_min_new = t_min + (t_max - t_min) * t_min_clip;
    let t_max_new = t_min + (t_max - t_min) * t_max_clip;

    if (u_max - u_min).max(t_max_new - t_min_new) < fat_line_epsilon {
        let t = (t_min_new + t_max_new) / 2.0;
        let u = (u_min + u_max) / 2.0;
        let (t1, t2) = if flip { (u, t) } else { (t, u) };
        if t1 < CURVETIME_EPSILON
            || t1 > 1.0 - CURVETIME_EPSILON
            || t2 < CURVETIME_EPSILON
            || t2 > 1.0 - CURVETIME_EPSILON
        {
            return calls;
        }

        if flip {
            if let Some([x1, y1]) = evaluate(v2, t2, 0) {
                if let Some([x2, y2]) = evaluate(v1, t1, 0) {
                    locations.push([t2, x1, y1, t1, x2, y2])
                }
            }
        } else {
            if let Some([x1, y1]) = evaluate(v1, t1, 0) {
                if let Some([x2, y2]) = evaluate(v2, t2, 0) {
                    locations.push([t1, x1, y1, t2, x2, y2])
                }
            }
        };
    } else {
        let v1 = split_cubic_bezier_part(v1, t_min_clip, t_max_clip);
        let u_diff = u_max - u_min;

        if t_max_clip - t_min_clip > 0.8 {
            if t_max_new - t_min_new > u_diff {
                let parts = split_cubic_bezier(&v1, 0.5);
                let t = (t_min_new + t_max_new) / 2.0;
                calls = bezier_intersections(
                    v2, &parts.0, locations, !flip, recursion, calls, u_min, u_max, t_min_new, t,
                );
                calls = bezier_intersections(
                    v2, &parts.1, locations, !flip, recursion, calls, u_min, u_max, t, t_max_new,
                );
            } else {
                let parts = split_cubic_bezier(v2, 0.5);
                let u = (u_min + u_max) / 2.0;
                calls = bezier_intersections(
                    &parts.0, &v1, locations, !flip, recursion, calls, u_min, u, t_min_new,
                    t_max_new,
                );
                calls = bezier_intersections(
                    &parts.1, &v1, locations, !flip, recursion, calls, u, u_max, t_min_new,
                    t_max_new,
                );
            }
        } else {
            if u_diff == 0.0 || u_diff >= fat_line_epsilon {
                calls = bezier_intersections(
                    v2, &v1, locations, !flip, recursion, calls, u_min, u_max, t_min_new, t_max_new,
                );
            } else {
                calls = bezier_intersections(
                    &v1, v2, locations, flip, recursion, calls, t_min_new, t_max_new, u_min, u_max,
                );
            }
        }
    }

    return calls;
}

/// 寻找最大值，支持浮点数运算
fn find_max<I>(iterable: I) -> I::Item
where
    I: IntoIterator,
    I::Item: PartialOrd,
{
    iterable
        .into_iter()
        .fold(None, |max, x| match max {
            None => Some(x),
            Some(y) => Some(if x > y { x } else { y }),
        })
        .unwrap()
}
fn find_min<I>(iterable: I) -> I::Item
where
    I: IntoIterator,
    I::Item: PartialOrd,
{
    iterable
        .into_iter()
        .fold(None, |min, x| match min {
            None => Some(x),
            Some(y) => Some(if x < y { x } else { y }),
        })
        .unwrap()
}

fn get_curve_intersections(v1: &[f64; 8], v2: &[f64; 8], locations: &mut Vec<[f64; 6]>) {
    let epsilon = EPSILON; // EPSILON should be defined somewhere in the scope
    let v1_min_x = find_min(vec![v1[0], v1[2], v1[4], v1[6]]);
    let v1_max_x = find_max(vec![v1[0], v1[2], v1[4], v1[6]]);
    let v1_min_y = find_min(vec![v1[1], v1[3], v1[5], v1[7]]);
    let v1_max_y = find_max(vec![v1[1], v1[3], v1[5], v1[7]]);
    let v2_min_x = find_min(vec![v2[0], v2[2], v2[4], v2[6]]);
    let v2_max_x = find_max(vec![v2[0], v2[2], v2[4], v2[6]]);
    let v2_min_y = find_min(vec![v2[1], v2[3], v2[5], v2[7]]);
    let v2_max_y = find_max(vec![v2[1], v2[3], v2[5], v2[7]]);

    if v1_max_x + epsilon > v2_min_x
        && v1_min_x - epsilon < v2_max_x
        && v1_max_y + epsilon > v2_min_y
        && v1_min_y - epsilon < v2_max_y
    {
        let straight1 = v1[2] == v1[0] && v1[3] == v1[1] && v1[4] == v1[6] && v1[5] == v1[7];
        let straight2 = v2[2] == v2[0] && v2[3] == v2[1] && v2[4] == v2[6] && v2[5] == v2[7];
        let straight = straight1 && straight2;
        let flip = straight1 && !straight2;
        // 直线相交，控制点和起点/终点一致
        if straight {
            let pt = line_intersection(v1[0], v1[1], v1[6], v1[7], v2[0], v2[1], v2[6], v2[7]);
            match pt {
                None => (),
                Some((x, y)) => {
                    let mut count = 0;
                    if (x == v1[0] && y == v1[1]) || (x == v1[6] && y == v1[7]) {
                        count += 1;
                    }
                    if (x == v2[0] && y == v2[1]) || (x == v2[6] && y == v2[7]) {
                        count += 1;
                    }
                    // 过滤起点和终点重合情况
                    if count == 2 {
                        return;
                    }
                    locations.push([-1.0, x, y, -1.0, x, y]);
                }
            }
            return;
        }
        // 直线和曲线相交
        if straight1 || straight2 {
            let is_v1_line = v1[2] == 0_f64 && v1[3] == 0_f64 && v1[4] == 0_f64 && v1[5] == 0_f64;
            let curve = if is_v1_line { v2 } else { v1 };
            let line = if is_v1_line {
                [v1[0], v1[1], v1[6], v1[7]]
            } else {
                [v2[0], v2[1], v2[6], v2[7]]
            };
            let instersections = line_and_curve_intersection(curve, &line);
            for item in &instersections {
                if is_v1_line {
                    locations.push([item.3.into(), item.4, item.5, item.0.into(), item.1, item.2]);
                } else {
                    locations.push([item.0.into(), item.1, item.2, item.3.into(), item.4, item.5]);
                }
            }
            return;
        }
        let vv1 = if flip { v2 } else { v1 };
        let vv2 = if flip { v1 } else { v2 };

        bezier_intersections(vv1, vv2, locations, flip, 0, 0, 0.0, 1.0, 0.0, 1.0);
    }
}

fn get_self_intersection(v: &[f64; 8]) -> Option<Vec<f64>> {
    let (x0, y0, x1, y1, x2, y2, x3, y3) = (v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]);

    let a1 = x0 * (y3 - y2) + y0 * (x2 - x3) + x3 * y2 - y3 * x2;
    let a2 = x1 * (y0 - y3) + y1 * (x3 - x0) + x0 * y3 - y0 * x3;
    let a3 = x2 * (y1 - y0) + y2 * (x0 - x1) + x1 * y0 - y1 * x0;

    let mut d3 = 3.0 * a3;
    let mut d2 = d3 - a2;
    let mut d1 = d2 - a2 + a1;

    let l = (d1 * d1 + d2 * d2 + d3 * d3).sqrt();
    let s = if l != 0.0 { 1.0 / l } else { 0.0 };

    d1 *= s;
    d2 *= s;
    d3 *= s;

    if is_zero(d1) {
        return None;
    }

    let d = 3.0 * d2 * d2 - 4.0 * d1 * d3;

    if d >= 0.0 {
        return None;
    }

    let f1 = if d > 0.0 {
        (d / 3.0).sqrt()
    } else {
        (-d).sqrt()
    };
    let f2 = 2.0 * d1;
    let t1 = (d2 + f1) / f2;
    let t2 = (d2 - f1) / f2;

    let has_roots = true;
    let t1_ok = has_roots && t1 > 0.0 && t1 < 1.0;
    let t2_ok = has_roots && t2 > 0.0 && t2 < 1.0;

    if !(t1_ok && t2_ok) {
        return None;
    }

    Some(if t1_ok || t2_ok {
        if t1_ok && t2_ok {
            if t1 < t2 {
                vec![t1, t2]
            } else {
                vec![t2, t1]
            }
        } else {
            vec![if t1_ok { t1 } else { t2 }]
        }
    } else {
        vec![]
    })
}

pub fn get_intersections(
    curves1: &Vec<[f64; 8]>,
    curves2: &Vec<[f64; 8]>,
    is_self: bool,
    locations: &mut Vec<[f64; 6]>,
) {
    let bounds_collisions = CollisionDetection::find_curve_bounds_collisions(
        &curves1,
        &curves2,
        is_self,
        GEOMETRIC_EPSILON,
    );
    for i in 0..curves1.len() {
        let curve1 = curves1[i];
        if is_self {
            match get_self_intersection(&curve1) {
                Some(t) => {
                    if !t[0].is_nan() && !t[1].is_nan() {
                        if let Some([x1, y1]) = evaluate(&curve1, t[0], 0) {
                            if let Some([x2, y2]) = evaluate(&curve1, t[1], 0) {
                                locations.push([t[0], x1, y1, t[1], x2, y2]);
                            }
                        }
                    }
                }
                None => (),
            }
        }
        for &index in &bounds_collisions[i] {
            if !is_self || index > i as i32 {
                let curve2 = curves2[index as usize];
                get_curve_intersections(&curve1, &curve2, locations);
            }
        }
    }
}

/// 寻找两条路径的相交点
#[wasm_bindgen]
pub fn rust_get_muti_intersections(curves1: JsValue, curves2: JsValue) -> Result<JsValue, JsValue> {
    let mut locations = vec![];
    let curves1_vec = serde_wasm_bindgen::from_value(curves1)?;
    let curves2_vec = serde_wasm_bindgen::from_value(curves2)?;

    get_intersections(&curves1_vec, &curves2_vec, false, &mut locations);

    Ok(serde_wasm_bindgen::to_value(&locations)?)
}

/// 寻找当前路径的相交点
#[wasm_bindgen]
pub fn rust_get_intersections(curves: JsValue) -> Result<JsValue, JsValue> {
    let mut locations = vec![];
    let curves_vec = serde_wasm_bindgen::from_value(curves)?;

    get_intersections(&curves_vec, &curves_vec, true, &mut locations);

    Ok(serde_wasm_bindgen::to_value(&locations)?)
}
