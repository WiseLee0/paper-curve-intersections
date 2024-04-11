import { CollisionDetection } from './utils/collisionDetection'
type Point = [number, number];
const GEOMETRIC_EPSILON = 1e-7
const CURVETIME_EPSILON = 1e-8
const FATLINE_EPSILON = 1e-9
const EPSILON = 1e-12
const MACHINE_EPSILON = 1.12e-16

/// 分割贝塞尔曲线
export function splitCubicBezier(bez: number[], t: number) {
    const [p1x, p1y, c1x, c1y, c2x, c2y, p2x, p2y] = bez;
    const u = 1 - t;
    const p3x = u * p1x + t * c1x;
    const p3y = u * p1y + t * c1y;
    const p4x = u * c1x + t * c2x;
    const p4y = u * c1y + t * c2y;
    const p5x = u * c2x + t * p2x;
    const p5y = u * c2y + t * p2y;
    const p6x = u * p3x + t * p4x;
    const p6y = u * p3y + t * p4y;
    const p7x = u * p4x + t * p5x;
    const p7y = u * p4y + t * p5y;
    const p8x = u * p6x + t * p7x;
    const p8y = u * p6y + t * p7y;
    return [
        [p1x, p1y, p3x, p3y, p6x, p6y, p8x, p8y],
        [p8x, p8y, p7x, p7y, p5x, p5y, p2x, p2y]
    ];
}

/// 切割部分曲线[t1,t2]
export function splitCubicBezierPart(v: number[], t1: number, t2: number) {
    if (t1 > 0) {
        v = splitCubicBezier(v, t1)[1];
    }
    if (t2 < 1) {
        v = splitCubicBezier(v, (t2 - t1) / (1.0 - t1))[0];
    }
    return v;
}

/// 计算贝塞尔凸包
function getConvexHull(dq0: number, dq1: number, dq2: number, dq3: number): number[][][] {
    const p0 = [0, dq0];
    const p1 = [1 / 3, dq1];
    const p2 = [2 / 3, dq2];
    const p3 = [1, dq3];
    const dist1: number = dq1 - (2 * dq0 + dq3) / 3;
    const dist2: number = dq2 - (dq0 + 2 * dq3) / 3;

    let hull: number[][][];

    if (dist1 * dist2 < 0) {
        // 凸包包括两个三角形
        hull = [[p0, p1, p3], [p0, p2, p3]];
    } else {
        const distRatio = dist1 / dist2;

        if (distRatio >= 2) {
            // 凸包包括一个三角形和一条线段
            hull = [[p0, p1, p3], [p0, p3]];
        } else if (distRatio <= 0.5) {
            // 凸包包括一个三角形和一条线段
            hull = [[p0, p2, p3], [p0, p3]];
        } else {
            // 凸包包括一个四边形和一条线段
            hull = [[p0, p1, p2, p3], [p0, p3]];
        }
    }

    return (dist1 || dist2) < 0 ? hull.reverse() : hull;
}

/// 凸包裁剪
function clipConvexHull(hullTop: Point[], hullBottom: Point[], dMin: number, dMax: number): number | null {
    if (hullTop[0][1] < dMin) {
        return clipConvexHullPart(hullTop, true, dMin);
    } else if (hullBottom[0][1] > dMax) {
        return clipConvexHullPart(hullBottom, false, dMax);
    } else {
        return hullTop[0][0];
    }
}
function clipConvexHullPart(part: Point[], isTop: boolean, threshold: number): number | null {
    let [prevX, prevY] = part[0];
    for (let i = 1; i < part.length; i++) {
        const [currentX, currentY] = part[i];
        // 根据是顶部还是底部以及阈值，决定是否应该裁剪
        if ((isTop ? currentY >= threshold : currentY <= threshold)) {
            // 如果当前点的Y坐标恰好等于阈值，则直接返回X坐标
            if (currentY === threshold) {
                return currentX;
            }
            // 否则，使用线性插值计算裁剪点的X坐标
            return prevX + (threshold - prevY) * (currentX - prevX) / (currentY - prevY);
        }
        // 更新前一个点的坐标
        prevX = currentX;
        prevY = currentY;
    }
    // 如果没有找到交点，则返回null
    return null;
}

/// Fat Line
function getFatline(v: number[]) {
    const q0x = v[0];
    const q0y = v[1];
    const q3x = v[6];
    const q3y = v[7];
    const d1 = signedDistance(q0x, q0y, q3x, q3y, v[2], v[3]) || 0;
    const d2 = signedDistance(q0x, q0y, q3x, q3y, v[4], v[5]) || 0;
    const factor = d1 * d2 > 0 ? 3.0 / 4.0 : 4.0 / 9.0;
    const dMin = factor * Math.min(0, d1, d2);
    const dMax = factor * Math.max(0, d1, d2);
    return [dMin, dMax, d1, d2, factor];
}
function signedDistance(px: number, py: number, vx: number, vy: number, x: number, y: number, asVector = false) {
    if (!asVector) {
        vx -= px;
        vy -= py;
    }
    return vx === 0 ? (vy > 0 ? x - px : px - x)
        : vy === 0 ? (vx < 0 ? y - py : py - y)
            : ((x - px) * vy - (y - py) * vx) / (
                vy > vx
                    ? vy * Math.sqrt(1 + (vx * vx) / (vy * vy))
                    : vx * Math.sqrt(1 + (vy * vy) / (vx * vx))
            );
}

const isZero = (val: number) => val >= -EPSILON && val <= EPSILON;
/// 计算贝塞尔曲线上的点、切线、法线和曲率
/// type = 0时，计算曲线上参数t所对应的点
/// type = 1时，计算曲线上的切线
/// type = 2时，计算曲线上的法线
/// type = 3时，计算曲线上的曲率
export function evaluate(v: number[], t: number) {
    if (t == null || t < 0 || t > 1) return [];
    let [x0, y0, x1, y1, x2, y2, x3, y3] = v;
    if (isZero(x1 - x0) && isZero(y1 - y0)) {
        x1 = x0;
        y1 = y0;
    }
    if (isZero(x2 - x3) && isZero(y2 - y3)) {
        x2 = x3;
        y2 = y3;
    }
    const cx = 3 * (x1 - x0)
    const bx = 3 * (x2 - x1) - cx
    const ax = x3 - x0 - cx - bx
    const cy = 3 * (y1 - y0)
    const by = 3 * (y2 - y1) - cy
    const ay = y3 - y0 - cy - by
    let x, y
    x = t === 0 ? x0 : t === 1 ? x3 : ((ax * t + bx) * t + cx) * t + x0;
    y = t === 0 ? y0 : t === 1 ? y3 : ((ay * t + by) * t + cy) * t + y0;
    return [x, y];
}
const isMachineZero = (val: number) => val >= -MACHINE_EPSILON && val <= MACHINE_EPSILON;

/// 计算直线相交
function lineIntersection(p1x: number, p1y: number, v1x: number, v1y: number, p2x: number, p2y: number, v2x: number, v2y: number) {
    v1x -= p1x;
    v1y -= p1y;
    v2x -= p2x;
    v2y -= p2y;
    const cross = v1x * v2y - v1y * v2x;
    if (!isMachineZero(cross)) {
        const dx = p1x - p2x
        const dy = p1y - p2y
        const u1 = (v2x * dy - v2y * dx) / cross
        const u2 = (v1x * dy - v1y * dx) / cross
        const epsilon = EPSILON
        const uMin = -epsilon
        const uMax = 1 + epsilon
        if (uMin < u1 && u1 < uMax && uMin < u2 && u2 < uMax) {
            const t = u1 <= 0 ? 0 : u1 >= 1 ? 1 : u1;
            return [p1x + t * v1x, p1y + t * v1y]
        }
    }
}

// 贝塞尔系数
function bezierCoeffs(P0: number, P1: number, P2: number, P3: number): number[] {
    return [-P0 + 3 * P1 + -3 * P2 + P3, 3 * P0 - 6 * P1 + 3 * P2, -3 * P0 + 3 * P1, P0];
}
function sgn(x: number): -1 | 1 {
    return x < 0 ? -1 : 1;
}
function sortSpecial(a: number[]): number[] {
    let flipped: boolean;
    let temp: number;

    do {
        flipped = false;
        for (let i = 0; i < a.length - 1; i++) {
            if ((a[i + 1] >= 0 && a[i] > a[i + 1]) || (a[i] < 0 && a[i + 1] >= 0)) {
                flipped = true;
                temp = a[i];
                a[i] = a[i + 1];
                a[i + 1] = temp;
            }
        }
    } while (flipped);
    return a;
}
function cubicRoots(P: number[]): number[] {
    if (P.length !== 4) throw new Error('Array must contain exactly four elements.');
    if (P[0] === 0) {
        if (P[1] === 0) {
            const t = [-1, -1, -1];
            t[0] = -1 * (P[3] / P[2]);
            t[1] = -1;
            t[2] = -1;

            /*discard out of spec roots*/
            for (let i = 0; i < 1; i++)
                if (t[i] < 0 || t[i] > 1.0 || isNaN(t[i])) t[i] = -1;

            /*sort but place -1 at the end*/
            return sortSpecial(t);
        }
        let DQ = P[2] * P[2] - 4 * P[1] * P[3]; // quadratic discriminant
        if (DQ >= 0) {
            DQ = Math.sqrt(DQ);
            const t = [-1, -1, -1];
            t[0] = -1 * ((DQ + P[2]) / (2 * P[1]));
            t[1] = ((DQ - P[2]) / (2 * P[1]));
            t[2] = -1;

            for (let i = 0; i < 2; i++)
                if (t[i] < 0 || t[i] > 1.0 || isNaN(t[i])) t[i] = -1;
            return sortSpecial(t);
        }
    }
    const [a, b, c, d] = P;

    const A = b / a;
    const B = c / a;
    const C = d / a;

    const Q = (3 * B - A ** 2) / 9;
    const R = (9 * A * B - 27 * C - 2 * A ** 3) / 54;
    const D = Q ** 3 + R ** 2;

    let t: number[] = [-1, -1, -1];

    if (D >= 0) {
        let d_sqrt = Math.sqrt(D);
        const S = sgn(R + d_sqrt) * Math.abs(R + d_sqrt) ** (1 / 3);
        const T = sgn(R - d_sqrt) * Math.abs(R - d_sqrt) ** (1 / 3);

        t[0] = -A / 3 + (S + T); // real root
        t[1] = -A / 3 - (S + T) / 2; // real part of complex root
        t[2] = -A / 3 - (S + T) / 2; // real part of complex root
        const Im = Math.sqrt(3) * (S - T) / 2; // complex part of root pair

        if (Im !== 0) {
            t[1] = -1;
            t[2] = -1;
        }
    } else {
        const q_sqrt = Math.sqrt(-Q)
        const th = Math.acos(R / Math.sqrt(-(Q ** 3)));

        t[0] = 2 * q_sqrt * Math.cos(th / 3) - A / 3;
        t[1] = 2 * q_sqrt * Math.cos((th + 2 * Math.PI) / 3) - A / 3;
        t[2] = 2 * q_sqrt * Math.cos((th + 4 * Math.PI) / 3) - A / 3;
    }

    for (let i = 0; i < 3; i++) {
        if (t[i] < 0 || t[i] > 1.0) t[i] = -1;
    }

    t = sortSpecial(t);

    return t;
}

function calculateTValue(x1: number, y1: number, x2: number, y2: number, x: number, y: number): number {
    // 计算向量 (x2 - x1, y2 - y1) 和 (x - x1, y - y1) 的点积
    const dotProduct = (x - x1) * (x2 - x1) + (y - y1) * (y2 - y1);
    // 计算向量 (x2 - x1, y2 - y1) 的长度的平方
    const lineLengthSquared = (x2 - x1) ** 2 + (y2 - y1) ** 2;

    // 计算参数 t 的值，这是点 (x, y) 在直线上的位置
    const t = dotProduct / lineLengthSquared;

    return t;
}

/// 计算直线和曲线相交
function lineAndCurveIntersection(v: number[], line: number[]) {
    const px = [v[0], v[2], v[4], v[6]]
    const py = [v[1], v[3], v[5], v[7]]
    const lx = [line[0], line[2]]
    const ly = [line[1], line[3]]
    const A = ly[1] - ly[0]; // A = y2 - y1
    const B = lx[0] - lx[1]; // B = x1 - x2
    const C = lx[0] * (ly[0] - ly[1]) + ly[0] * (lx[1] - lx[0]); // C = x1 * (y1 - y2) + y1 * (x2 - x1)

    const bx = bezierCoeffs(px[0], px[1], px[2], px[3]);
    const by = bezierCoeffs(py[0], py[1], py[2], py[3]);

    const P: number[] = [
        A * bx[0] + B * by[0], // t^3
        A * bx[1] + B * by[1], // t^2
        A * bx[2] + B * by[2], // t
        A * bx[3] + B * by[3] + C // 1
    ];

    const r = cubicRoots(P);
    const res = []
    for (let i = 0; i < r.length; i++) {
        const t = r[i];
        const intersectionX = bx[0] * t ** 3 + bx[1] * t ** 2 + bx[2] * t + bx[3];
        const intersectionY = by[0] * t ** 3 + by[1] * t ** 2 + by[2] * t + by[3];

        let s: number;
        if (lx[1] !== lx[0]) {
            s = (intersectionX - lx[0]) / (lx[1] - lx[0]);
        } else {
            s = (intersectionY - ly[0]) / (ly[1] - ly[0]);
        }

        if (t < 0 || t > 1 || s < 0 || s > 1.0) {
            continue
        } else {
            const lineT = calculateTValue(line[0], line[1], line[2], line[3], intersectionX, intersectionY)
            if (t <= CURVETIME_EPSILON || t >= 1 - CURVETIME_EPSILON) {
                if (lineT <= CURVETIME_EPSILON && lineT >= 1 - CURVETIME_EPSILON) {
                    continue
                }
            }
            res.push([t, intersectionX, intersectionY, lineT, intersectionX, intersectionY])
        }
    }
    return res
}

/// 计算曲线相交
const bezierIntersections = (v1: number[], v2: number[], c1: number[], c2: number[], locations: number[][], flip = false, recursion = 0, calls = 0, tMin = 0.0, tMax = 1.0, uMin = 0.0, uMax = 1.0) => {
    // 避免更深层次的递归
    if (++calls >= 4096 || ++recursion >= 40) {
        return calls
    }

    const fatLineEpsilon = FATLINE_EPSILON
    const q0x = v2[0], q0y = v2[1], q3x = v2[6], q3y = v2[7];
    // 计算Fat Line，用于下面剪辑凸包
    const [dMin, dMax, d1, d2] = getFatline(v2);
    const dp0 = signedDistance(q0x, q0y, q3x, q3y, v1[0], v1[1]);
    const dp1 = signedDistance(q0x, q0y, q3x, q3y, v1[2], v1[3]);
    const dp2 = signedDistance(q0x, q0y, q3x, q3y, v1[4], v1[5]);
    const dp3 = signedDistance(q0x, q0y, q3x, q3y, v1[6], v1[7]);
    // 计算凸包，分成上下两部分
    const [top, bottom] = getConvexHull(dp0, dp1, dp2, dp3) as any
    // 如果所有点共线，则直接退出
    if (d1 === 0 && d2 === 0 && dp0 === 0 && dp1 === 0 && dp2 === 0 && dp3 === 0) {
        return calls
    }
    // 使用dMin和dMax剪辑凸包，如果其中一个结果为空，则不会有交集
    const tMinClip = clipConvexHull(top, bottom, dMin, dMax);
    const tMaxClip = clipConvexHull(top.reverse(), bottom.reverse(), dMin, dMax);
    if (tMinClip == null || tMaxClip == null) {
        return calls
    }

    // tMin和tMax在范围（0,1）内，将其投影回v2的参数范围内
    const tMinNew = tMin + (tMax - tMin) * tMinClip
    const tMaxNew = tMin + (tMax - tMin) * tMaxClip

    if (Math.max(uMax - uMin, tMaxNew - tMinNew) < fatLineEpsilon) {
        const t = (tMinNew + tMaxNew) / 2;
        const u = (uMin + uMax) / 2;
        const t1 = flip ? u : t
        const t2 = flip ? t : u
        const cc1 = flip ? c2 : c1
        const cc2 = flip ? c1 : c2

        if (t1 > 1 - CURVETIME_EPSILON || t1 < CURVETIME_EPSILON || t2 > 1 - CURVETIME_EPSILON || t2 < CURVETIME_EPSILON) {
            return calls
        }
        let intersections: any
        const [x1, y1] = evaluate(cc1, t1)
        const [x2, y2] = evaluate(cc2, t2)
        intersections = [t1, x1, y1, t2, x2, y2];
        locations.push(intersections)
    } else {
        v1 = splitCubicBezierPart(v1, tMinClip, tMaxClip);
        const uDiff = uMax - uMin;
        if (tMaxClip - tMinClip > 0.8) {
            // 细分收敛最小的曲线
            if (tMaxNew - tMinNew > uDiff) {
                const parts = splitCubicBezier(v1, 0.5)
                const t = (tMinNew + tMaxNew) / 2;
                calls = bezierIntersections(v2, parts[0], c2, c1, locations, !flip, recursion, calls, uMin, uMax, tMinNew, t);
                calls = bezierIntersections(v2, parts[1], c2, c1, locations, !flip, recursion, calls, uMin, uMax, t, tMaxNew);
            } else {
                const parts = splitCubicBezier(v2, 0.5)
                const u = (uMin + uMax) / 2;
                calls = bezierIntersections(parts[0], v1, c2, c1, locations, !flip, recursion, calls, uMin, u, tMinNew, tMaxNew);
                calls = bezierIntersections(parts[1], v1, c2, c1, locations, !flip, recursion, calls, u, uMax, tMinNew, tMaxNew);
            }
        } else {
            if (uDiff === 0 || uDiff >= fatLineEpsilon) {
                calls = bezierIntersections(v2, v1, c2, c1, locations, !flip, recursion, calls, uMin, uMax, tMinNew, tMaxNew);
            } else {
                calls = bezierIntersections(v1, v2, c1, c2, locations, flip, recursion, calls, tMinNew, tMaxNew, uMin, uMax);
            }
        }
    }
    return calls;
}

// 获取自相交点
const getSelfIntersection = (v: number[]) => {
    const [x0, y0, x1, y1, x2, y2, x3, y3] = v;
    const hasIntersection = lineIntersection(x0, y0, x1, y1, x2, y2, x3, y3)
    if (!hasIntersection) return null

    const a1 = x0 * (y3 - y2) + y0 * (x2 - x3) + x3 * y2 - y3 * x2
    const a2 = x1 * (y0 - y3) + y1 * (x3 - x0) + x0 * y3 - y0 * x3
    const a3 = x2 * (y1 - y0) + y2 * (x0 - x1) + x1 * y0 - y1 * x0
    let d3 = 3 * a3
    let d2 = d3 - a2
    let d1 = d2 - a2 + a1
    const l = Math.sqrt(d1 * d1 + d2 * d2 + d3 * d3)
    const s = l !== 0 ? 1 / l : 0
    d1 *= s;
    d2 *= s;
    d3 *= s;
    if (isZero(d1)) return null;
    const d = 3 * d2 * d2 - 4 * d1 * d3;
    if (d >= 0) return null;
    const f1 = d > 0 ? Math.sqrt(d / 3) : Math.sqrt(-d)
    const f2 = 2 * d1;
    const t1 = (d2 + f1) / f2
    const t2 = (d2 - f1) / f2
    const hasRoots = t1 !== undefined
    const t1Ok = hasRoots && t1 > 0 && t1 < 1
    const t2Ok = hasRoots && t2 > 0 && t2 < 1
    if (!(t1Ok && t2Ok)) {
        return null;
    }
    return t1Ok || t2Ok ? t1Ok && t2Ok ? t1 < t2 ? [t1, t2] : [t2, t1] : [t1Ok ? t1 : t2] : null
}

const getCurveIntersections = (v1: number[], v2: number[], locations: number[][]) => {
    const epsilon = EPSILON
    if (Math.max(v1[0], v1[2], v1[4], v1[6]) + epsilon >
        Math.min(v2[0], v2[2], v2[4], v2[6]) &&
        Math.min(v1[0], v1[2], v1[4], v1[6]) - epsilon <
        Math.max(v2[0], v2[2], v2[4], v2[6]) &&
        Math.max(v1[1], v1[3], v1[5], v1[7]) + epsilon >
        Math.min(v2[1], v2[3], v2[5], v2[7]) &&
        Math.min(v1[1], v1[3], v1[5], v1[7]) - epsilon <
        Math.max(v2[1], v2[3], v2[5], v2[7])) {
        const straight1 = (v1[2] === v1[0] && v1[3] === v1[1] && v1[4] === v1[6] && v1[5] === v1[7])
        const straight2 = (v2[2] === v2[0] && v2[3] === v2[1] && v2[4] === v2[6] && v2[5] === v2[7])
        const straight = straight1 && straight2
        const flip = straight1 && !straight2
        // 过滤共线情况
        if (v1[0] === v2[6] && v1[1] === v2[7] && v1[2] === v2[4] && v1[3] === v2[5] && v1[4] === v2[2] && v1[5] === v2[3]) {
            return
        }

        // 直线相交，控制点和起点/终点一致
        if (straight) {
            const pt = lineIntersection(v1[0], v1[1], v1[6], v1[7], v2[0], v2[1], v2[6], v2[7])
            if (pt) {
                let count = 0
                if ((pt[0] === v1[0] && pt[1] === v1[1]) || (pt[0] === v1[6] && pt[1] === v1[7])) count++
                if ((pt[0] === v2[0] && pt[1] === v2[1]) || (pt[0] === v2[6] && pt[1] === v2[7])) count++
                // 过滤起点和终点重合情况
                if (count === 2) return;
                const t1 = calculateTValue(v1[0], v1[1], v1[6], v1[7], pt[0], pt[1])
                const t2 = calculateTValue(v2[0], v2[1], v2[6], v2[7], pt[0], pt[1])
                if (t1 > 1 - GEOMETRIC_EPSILON || t1 < GEOMETRIC_EPSILON) count++
                if (t2 > 1 - GEOMETRIC_EPSILON || t2 < GEOMETRIC_EPSILON) count++
                if (count === 4) return;
                locations.push([t1, pt[0], pt[1], t2, pt[0], pt[1]])
            }
            return
        }
        // 直线和曲线相交
        if (straight1 || straight2) {
            const curve = straight1 ? v2 : v1
            const line = straight1 ? [v1[0], v1[1], v1[6], v1[7]] : [v2[0], v2[1], v2[6], v2[7]]
            const instersections = lineAndCurveIntersection(curve, line)
            for (let i = 0; i < instersections.length; i++) {
                const item = instersections[i];
                // 排除端点重合
                if (item[3] > 1 - GEOMETRIC_EPSILON || item[3] < GEOMETRIC_EPSILON) continue
                if (straight1) locations.push([item[3], item[4], item[5], item[0], item[1], item[2]])
                else locations.push(item)
            }
            return
        }

        // 曲线和曲线相交
        bezierIntersections(flip ? v2 : v1, flip ? v1 : v2, flip ? v2 : v1, flip ? v1 : v2, locations, flip)
    }
}

export const getIntersections = (curves1: number[][], curves2: number[][] | undefined, locations: number[][]) => {
    const is_self = !curves2;
    if (is_self) {
        curves2 = []
        for (let i = 0; i < curves1.length; i++) {
            curves2.push([...curves1[i]])
        }
    }
    const boundsCollisions = CollisionDetection.findCurveBoundsCollisions(curves1, curves2, is_self, GEOMETRIC_EPSILON);
    for (let i = 0; i < curves1.length; i++) {
        const curve1 = curves1[i]
        if (is_self) {
            const t = getSelfIntersection(curve1);
            if (t) {
                let [t1, t2] = t
                if (t1 && t2) {
                    const [x1, y1] = evaluate(curve1, t1)
                    const [x2, y2] = evaluate(curve1, t2)
                    locations.push([t1, x1, y1, t2, x2, y2])
                }
            }
        }
        const collisions = boundsCollisions[i];
        for (let j = 0; j < collisions.length; j++) {
            const index = collisions[j];
            if (!is_self || index > i) {
                getCurveIntersections(curve1, curves2![index], locations)
            }
        }
    }
}
