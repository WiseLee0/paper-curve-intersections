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

    let res = intersections::evaluate(&bez, 0.3, 0);
    const TEST_RES: [f64; 2] = [2.651569154739377, 20.57090359497071];
    assert!(float_arrays_equal(&res, &TEST_RES));

    let res = intersections::evaluate(&bez, 0.9, 0);
    const TEST_RES_2: [f64; 2] = [58.25072064018254, 98.7359024963379];
    assert!(float_arrays_equal(&res, &TEST_RES_2));
}
