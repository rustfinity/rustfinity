use advanced_traits::*;

#[test]
fn test_add_millimeters_and_meters() {
    let mm = Millimeters(500);
    let m = Meters(2);

    let result = mm + m;
    assert_eq!(result.0, 2500);
}

#[test]
fn test_zero_addition() {
    let mm = Millimeters(0);
    let m = Meters(5);

    let result = mm + m;
    assert_eq!(result.0, 5000);
}

#[test]
fn test_large_numbers() {
    let mm = Millimeters(1_000_000);
    let m = Meters(2_000);

    let result = mm + m;
    assert_eq!(result.0, 3_000_000);
}

#[test]
fn test_addition_with_zero_meters() {
    let mm = Millimeters(1000);
    let m = Meters(0);

    let result = mm + m;
    assert_eq!(result.0, 1000);
}

#[test]
fn test_addition_with_zero_millimeters() {
    let mm = Millimeters(0);
    let m = Meters(10);

    let result = mm + m;
    assert_eq!(result.0, 10000);
}

#[test]
fn test_addition_with_both_zero() {
    let mm = Millimeters(0);
    let m = Meters(0);

    let result = mm + m;
    assert_eq!(result.0, 0);
}

#[test]
fn test_addition_with_large_meters() {
    let mm = Millimeters(500);
    let m = Meters(1_000_000);

    let result = mm + m;
    assert_eq!(result.0, 1_000_000_500);
}

#[test]
fn test_multiple_additions() {
    let mm = Millimeters(1000);
    let m1 = Meters(1);
    let m2 = Meters(2);

    let result = (mm + m1) + m2;
    assert_eq!(result.0, 4000);
}
