use the_unit_type::*;

#[test]
fn test_returns_unit() {
    let unit = print_message();

    assert_eq!(unit, (), "The function should return the unit type `()`");
}
