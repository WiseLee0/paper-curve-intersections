pub struct CollisionDetection;
/// 检测两组曲线之间的边界碰撞
/// 
/// 通过比较边界框的最小和最大坐标值，对合并后的边界框按照 x 最小值进行排序
/// 找出所有可能相交的曲线对
impl CollisionDetection {
    pub fn get_bounds(curves: &[[f64; 8]]) -> Vec<[f64; 4]> {
        curves
            .iter()
            .map(|&v| {
                let (mut min_x, mut min_y, mut max_x, mut max_y) = (
                    f64::INFINITY,
                    f64::INFINITY,
                    f64::NEG_INFINITY,
                    f64::NEG_INFINITY,
                );
                for (i, &val) in v.iter().enumerate() {
                    if i % 2 == 0 {
                        min_x = min_x.min(val);
                        max_x = max_x.max(val);
                    } else {
                        min_y = min_y.min(val);
                        max_y = max_y.max(val);
                    }
                }
                [min_x, min_y, max_x, max_y]
            })
            .collect()
    }
    pub fn find_curve_bounds_collisions(
        curves1: &Vec<[f64; 8]>,
        curves2: &Vec<[f64; 8]>,
        is_self: bool,
        tolerance: f64,
    ) -> Vec<Vec<i32>> {
        let bounds1 = Self::get_bounds(curves1);
        if is_self {
            Self::find_bounds_collisions(&bounds1, &bounds1, is_self, tolerance)
        } else {
            let bounds2 = Self::get_bounds(curves2);
            Self::find_bounds_collisions(&bounds1, &bounds2, is_self, tolerance)
        }
    }

    pub fn binary_search(
        indices: &Vec<usize>,
        bounds: &Vec<[f64; 4]>,
        coord: usize,
        value: f64,
    ) -> Option<usize> {
        indices
            .binary_search_by(|&i| {
                bounds[i][coord]
                    .partial_cmp(&value)
                    .unwrap_or(std::cmp::Ordering::Less)
            })
            .map_or_else(|err| if err > 0 { Some(err - 1) } else { None }, Some)
    }

    pub fn find_bounds_collisions(
        bounds_a: &Vec<[f64; 4]>,
        bounds_b: &Vec<[f64; 4]>,
        is_self: bool,
        tolerance: f64,
    ) -> Vec<Vec<i32>> {
        let all_bounds = if is_self {
            bounds_a.clone()
        } else {
            [bounds_a.as_slice(), bounds_b.as_slice()].concat()
        };
        let length_a = bounds_a.len();
        let mut all_indices_by_pri0: Vec<usize> = (0..all_bounds.len()).collect();

        all_indices_by_pri0.sort_unstable_by(|&i1, &i2| {
            all_bounds[i1][0].partial_cmp(&all_bounds[i2][0]).unwrap()
        });

        let mut active_indices_by_pri1: Vec<usize> = Vec::new();
        let mut all_collisions: Vec<Vec<i32>> = vec![Vec::new(); length_a];

        for &cur_index in all_indices_by_pri0.iter() {
            let cur_bounds = &all_bounds[cur_index];
            let orig_index = if is_self {
                cur_index as i32
            } else {
                cur_index as i32 - length_a as i32
            };
            let is_current_a = cur_index < length_a;
            let is_current_b = is_self || !is_current_a;
            let mut cur_collisions: Vec<i32> = if is_current_a { Vec::new() } else { Vec::new() };

            if !active_indices_by_pri1.is_empty() {
                let prune_count: usize = match Self::binary_search(
                    &active_indices_by_pri1,
                    &all_bounds,
                    2,
                    cur_bounds[0] - tolerance,
                ) {
                    Some(count) => count + 1,
                    None => 0,
                };
                active_indices_by_pri1.drain(..prune_count);

                let cur_sec1 = cur_bounds[3];
                let cur_sec0 = cur_bounds[1];

                for &active_index in active_indices_by_pri1.iter() {
                    let active_bounds = &all_bounds[active_index];
                    let is_active_a = active_index < length_a;
                    let is_active_b = is_self || active_index >= length_a;

                    if ((is_current_a && is_active_b) || (is_current_b && is_active_a))
                        && (cur_sec1 >= active_bounds[1] - tolerance
                            && cur_sec0 <= active_bounds[3] + tolerance)
                    {
                        if is_current_a && is_active_b {
                            cur_collisions.push(if is_self {
                                active_index as i32
                            } else {
                                (active_index - length_a) as i32
                            });
                        }
                        if is_current_b && is_active_a {
                            all_collisions[active_index].push(orig_index);
                        }
                    }
                }
            }

            if is_current_a {
                if bounds_a == bounds_b {
                    cur_collisions.push(cur_index as i32);
                }
                all_collisions[cur_index] = cur_collisions;
            }

            let cur_pri1 = cur_bounds[2];
            let index = match Self::binary_search(&active_indices_by_pri1, &all_bounds, 2, cur_pri1)
            {
                Some(count) => count + 1,
                None => 0,
            };
            active_indices_by_pri1.insert(index, cur_index);
        }

        for collisions in all_collisions.iter_mut() {
            if !collisions.is_empty() {
                collisions.sort_unstable();
            }
        }

        all_collisions
    }
}
