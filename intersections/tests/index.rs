use intersections;

/// 数组每一项保留两位小数
fn format_floats_to_two_decimals(floats: &[f64]) -> Vec<String> {
    floats.iter().map(|&num| format!("{:.2}", num)).collect()
}

/// 判断数组每一项是否相等
fn arrays_equal<T: PartialEq>(arr1: &[T], arr2: &[T]) -> bool {
    arr1.len() == arr2.len() && arr1.iter().zip(arr2).all(|(a, b)| a == b)
}
/// 判断float数组每一项是否相等
fn float_arrays_equal(_arr1: &[f64], _arr2: &[f64]) -> bool {
    let arr1 = format_floats_to_two_decimals(&_arr1);
    let arr2 = format_floats_to_two_decimals(&_arr2);
    arrays_equal(&arr1, &arr2)
}

/// 测试分割贝塞尔曲线
#[test]
fn test_split_cubic_bezier() {
    let bez = [
        106.13448333740234,
        52.76838684082031,
        -115.86551666259766,
        -44.73158264160156,
        83.74869728088379,
        102.76840209960938,
        56.13446044921875,
        102.76840209960938,
    ];
    let t = 0.3;
    let (left, right) = intersections::split_cubic_bezier(&bez, t);
    const TEST_RES: [f64; 8] = [
        106.13448333740234,
        52.76838684082031,
        39.534483337402335,
        23.518395996093748,
        10.879762592315668,
        16.31840103149414,
        2.6515691547393763,
        20.5709035949707,
    ];
    assert!(float_arrays_equal(&left, &TEST_RES));

    const TEST_RES_2: [f64; 8] = [
        2.6515691547393763,
        20.5709035949707,
        -16.54754886627197,
        30.493409576416017,
        75.46442623138427,
        102.76840209960938,
        56.13446044921875,
        102.76840209960938,
    ];
    assert!(float_arrays_equal(&right, &TEST_RES_2));
}

/// 测试切割部分曲线[t1,t2]
#[test]
fn test_split_cubic_bezier_part() {
    let bez = [
        106.13448333740234,
        52.76838684082031,
        -115.86551666259766,
        -44.73158264160156,
        83.74869728088379,
        102.76840209960938,
        56.13446044921875,
        102.76840209960938,
    ];
    let res = intersections::split_cubic_bezier_part(&bez, 0.3, 0.8);
    const TEST_RES: [f64; 8] = [
        2.6515691547393763,
        20.5709035949707,
        -11.062086574554444,
        27.65840786743164,
        31.964611328125002,
        66.5584052734375,
        50.62632977294923,
        88.20840344238283,
    ];
    assert!(float_arrays_equal(&res, &TEST_RES));
}

/// 测试计算贝塞尔曲线上的点、切线、法线和曲率
#[test]
fn test_evaluate() {
    let bez = [
        106.13448333740234,
        52.76838684082031,
        -115.86551666259766,
        -44.73158264160156,
        83.74869728088379,
        102.76840209960938,
        56.13446044921875,
        102.76840209960938,
    ];

    if let Some(res) = intersections::evaluate(&bez, 0.3, 0) {
        const TEST_RES: [f64; 2] = [2.651569154739377, 20.57090359497071];
        assert!(float_arrays_equal(&res, &TEST_RES));
    }

    if let Some(res) = intersections::evaluate(&bez, 0.9, 0) {
        const TEST_RES_2: [f64; 2] = [58.25072064018254, 98.7359024963379];
        assert!(float_arrays_equal(&res, &TEST_RES_2));
    }
}

/// 测试直线相交
#[test]
fn test_line_intersection() {
    // 有相交点，并且t值为-1
    let curves1 = vec![
        [38.5, 0.0, 38.5, 0.0, 62.0, 87.0, 62.0, 87.0],
        [0.0, 64.5, 0.0, 64.5, 80.0, 17.5, 80.0, 17.5],
        [80.0, 17.5, 80.0, 17.5, 0.0, 17.5, 0.0, 17.5],
        [0.0, 17.5, 0.0, 17.5, 78.5, 67.0, 78.5, 67.0],
        [78.5, 67.0, 78.5, 67.0, 0.0, 64.5, 0.0, 64.5],
    ];
    let curves2 = curves1.clone();
    const TEST_RES: [[f64; 6]; 5] = [
        [
            -1.0,
            48.26337652675305,
            36.14526629053258,
            -1.0,
            48.26337652675305,
            36.14526629053258,
        ],
        [-1.0, 43.22701149425287, 17.5, -1.0, 43.22701149425287, 17.5],
        [
            -1.0,
            52.101279505846016,
            50.353673064195895,
            -1.0,
            52.101279505846016,
            50.353673064195895,
        ],
        [
            -1.0,
            56.40765424805228,
            66.29642210981058,
            -1.0,
            56.40765424805228,
            66.29642210981058,
        ],
        [
            -1.0,
            38.58552846591281,
            41.831002026276224,
            -1.0,
            38.58552846591281,
            41.831002026276224,
        ],
    ];
    let mut locations = vec![];
    intersections::get_intersections(&curves1, &curves2, true, &mut locations);
    assert_eq!(locations.len(), 5);
    assert!(float_arrays_equal(&locations[0], &TEST_RES[0]));
    assert!(float_arrays_equal(&locations[1], &TEST_RES[1]));
    assert!(float_arrays_equal(&locations[2], &TEST_RES[2]));
    assert!(float_arrays_equal(&locations[3], &TEST_RES[3]));
    assert!(float_arrays_equal(&locations[4], &TEST_RES[4]));

    // 不存在相交
    let curves1 = vec![[0.0, 0.0, 0.0, 0.0, 5.0, 5.0, 5.0, 5.0]];
    let curves2 = vec![[0.0, 5.0, 0.0, 5.0, 5.0, 5.0, 5.0, 5.0]];
    let mut locations = vec![];
    intersections::get_intersections(&curves1, &curves2, false, &mut locations);
    assert_eq!(locations.len(), 0);
}

/// 测试直线与曲线相交
#[test]
fn test_line_and_curve_intersection() {
    let curves1 = vec![
        [
            1f64,
            131.842,
            72.3227,
            8.758089999999996,
            110.937,
            -81.02649999999998,
            203.5,
            117.342,
        ],
        [
            203.5, 117.342, 101.207, 207.919, 64.5398, 183.033, 1f64, 131.842,
        ],
    ];
    let curves2 = vec![
        [18f64, 251.5, 18f64, 251.5, 227f64, 90f64, 227f64, 90f64],
        [227f64, 90f64, 227f64, 90f64, 2f64, 74f64, 2f64, 74f64],
        [2f64, 74f64, 2f64, 74f64, 202f64, 251.5, 202f64, 251.5],
        [202f64, 251.5, 202f64, 251.5, 154.5, 2f64, 154.5, 2f64],
        [154.5, 2f64, 154.5, 2f64, 18f64, 251.5, 18f64, 251.5],
    ];
    let mut locations = vec![];
    intersections::get_intersections(&curves1, &curves2, false, &mut locations);
    assert_eq!(locations.len(), 10);
}

/// 测试曲线与曲线相交
#[test]
fn test_bezier_intersections() {
    let curves1 = vec![
        [
            100f64,
            50f64,
            100f64,
            22.38576316833496,
            22.38576316833496,
            100f64,
            50f64,
            100f64,
        ],
        [
            100f64,
            50f64,
            100f64,
            22.38576316833496,
            0f64,
            22.38576316833496,
            0f64,
            50f64,
        ],
        [
            100f64,
            50f64,
            100f64,
            77.61423683166504,
            22.38576316833496,
            0f64,
            50f64,
            0f64,
        ],
        [
            50f64,
            100f64,
            77.61423683166504,
            100f64,
            0f64,
            22.38576316833496,
            0f64,
            50f64,
        ],
        [
            50f64,
            100f64,
            22.38576316833496,
            100f64,
            77.61423683166504,
            0f64,
            50f64,
            0f64,
        ],
        [
            0f64,
            50f64,
            0f64,
            77.61423683166504,
            77.61423683166504,
            0f64,
            50f64,
            0f64,
        ],
    ];
    let curves2 = curves1.clone();
    let mut locations = vec![];
    intersections::get_intersections(&curves1, &curves2, true, &mut locations);
    assert_eq!(locations.len(), 13);
}

#[test]
fn test_line_intersections() {
    let curves1:Vec<[f64; 8]> = vec![
        [
            0.0,
            0.0,
            0.0,
            0.0,
            109.5,
            0.0,
            109.5,
            0.0
        ],
        [
            109.5,
            0.0,
            109.5,
            0.0,
            74.89366432468393,
            53.59184346845434,
            41.5,
            49.5
        ],
        [
            41.5,
            49.5,
            8.106335675316075,
            45.40815653154567,
            0.0,
            0.0,
            0.0,
            0.0
        ],
        [
            41.5,
            0.0,
            41.5,
            0.0,
            151.0,
            0.0,
            151.0,
            0.0
        ],
        [
            151.0,
            0.0,
            151.0,
            0.0,
            116.39366432468393,
            53.59184346845434,
            83.0,
            49.5
        ],
        [
            83.0,
            49.5,
            49.606335675316075,
            45.40815653154567,
            41.5,
            0.0,
            41.5,
            0.0
        ]
    ];
    let curves2 = curves1.clone();
    let mut locations = vec![];
    intersections::get_intersections(&curves1, &curves2, true, &mut locations);
}
