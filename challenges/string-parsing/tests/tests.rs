use string_parsing::*;

// ============ parse_int tests ============

#[test]
fn parse_int_positive() {
    assert_eq!(parse_int("42"), Ok(42));
}

#[test]
fn parse_int_negative() {
    assert_eq!(parse_int("-17"), Ok(-17));
}

#[test]
fn parse_int_zero() {
    assert_eq!(parse_int("0"), Ok(0));
}

#[test]
fn parse_int_with_whitespace() {
    assert_eq!(parse_int("  123  "), Ok(123));
}

#[test]
fn parse_int_max() {
    assert_eq!(parse_int("2147483647"), Ok(i32::MAX));
}

#[test]
fn parse_int_min() {
    assert_eq!(parse_int("-2147483648"), Ok(i32::MIN));
}

#[test]
fn parse_int_invalid() {
    assert!(parse_int("not a number").is_err());
}

#[test]
fn parse_int_empty() {
    assert!(parse_int("").is_err());
}

#[test]
fn parse_int_float() {
    assert!(parse_int("3.14").is_err());
}

// ============ parse_bool tests ============

#[test]
fn parse_bool_true() {
    assert_eq!(parse_bool("true"), Ok(true));
}

#[test]
fn parse_bool_false() {
    assert_eq!(parse_bool("false"), Ok(false));
}

#[test]
fn parse_bool_yes() {
    assert_eq!(parse_bool("yes"), Ok(true));
}

#[test]
fn parse_bool_no() {
    assert_eq!(parse_bool("no"), Ok(false));
}

#[test]
fn parse_bool_one() {
    assert_eq!(parse_bool("1"), Ok(true));
}

#[test]
fn parse_bool_zero() {
    assert_eq!(parse_bool("0"), Ok(false));
}

#[test]
fn parse_bool_case_insensitive() {
    assert_eq!(parse_bool("TRUE"), Ok(true));
    assert_eq!(parse_bool("FALSE"), Ok(false));
    assert_eq!(parse_bool("Yes"), Ok(true));
    assert_eq!(parse_bool("NO"), Ok(false));
}

#[test]
fn parse_bool_with_whitespace() {
    assert_eq!(parse_bool("  true  "), Ok(true));
}

#[test]
fn parse_bool_invalid() {
    assert!(parse_bool("maybe").is_err());
    assert!(parse_bool("2").is_err());
    assert!(parse_bool("").is_err());
}

// ============ parse_key_value tests ============

#[test]
fn parse_key_value_simple() {
    assert_eq!(
        parse_key_value("name=Alice"),
        Ok(("name".to_string(), "Alice".to_string()))
    );
}

#[test]
fn parse_key_value_number() {
    assert_eq!(
        parse_key_value("count=42"),
        Ok(("count".to_string(), "42".to_string()))
    );
}

#[test]
fn parse_key_value_empty_value() {
    assert_eq!(
        parse_key_value("key="),
        Ok(("key".to_string(), "".to_string()))
    );
}

#[test]
fn parse_key_value_with_spaces() {
    assert_eq!(
        parse_key_value(" key = value "),
        Ok(("key".to_string(), "value".to_string()))
    );
}

#[test]
fn parse_key_value_multiple_equals() {
    // Should only split at the first '='
    assert_eq!(
        parse_key_value("equation=x=y+z"),
        Ok(("equation".to_string(), "x=y+z".to_string()))
    );
}

#[test]
fn parse_key_value_no_equals() {
    assert!(parse_key_value("no_equals_sign").is_err());
}

#[test]
fn parse_key_value_url_value() {
    assert_eq!(
        parse_key_value("url=https://example.com?foo=bar"),
        Ok((
            "url".to_string(),
            "https://example.com?foo=bar".to_string()
        ))
    );
}

// ============ Color FromStr tests ============

#[test]
fn color_parse_basic() {
    let color: Color = "255,128,0".parse().unwrap();
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 128);
    assert_eq!(color.b, 0);
}

#[test]
fn color_parse_black() {
    let color: Color = "0,0,0".parse().unwrap();
    assert_eq!(color, Color { r: 0, g: 0, b: 0 });
}

#[test]
fn color_parse_white() {
    let color: Color = "255,255,255".parse().unwrap();
    assert_eq!(
        color,
        Color {
            r: 255,
            g: 255,
            b: 255
        }
    );
}

#[test]
fn color_parse_with_spaces() {
    let color: Color = " 100 , 150 , 200 ".parse().unwrap();
    assert_eq!(
        color,
        Color {
            r: 100,
            g: 150,
            b: 200
        }
    );
}

#[test]
fn color_parse_invalid_format() {
    let result: Result<Color, _> = "255,128".parse();
    assert!(result.is_err());
}

#[test]
fn color_parse_too_many_values() {
    let result: Result<Color, _> = "255,128,0,255".parse();
    assert!(result.is_err());
}

#[test]
fn color_parse_out_of_range() {
    let result: Result<Color, _> = "256,0,0".parse();
    assert!(result.is_err());
}

#[test]
fn color_parse_negative() {
    let result: Result<Color, _> = "-1,0,0".parse();
    assert!(result.is_err());
}

#[test]
fn color_parse_non_numeric() {
    let result: Result<Color, _> = "red,green,blue".parse();
    assert!(result.is_err());
}

// ============ parse_list tests ============

#[test]
fn parse_list_integers() {
    assert_eq!(parse_list::<i32>("1,2,3", ','), Ok(vec![1, 2, 3]));
}

#[test]
fn parse_list_floats() {
    assert_eq!(
        parse_list::<f64>("1.5;2.5;3.5", ';'),
        Ok(vec![1.5, 2.5, 3.5])
    );
}

#[test]
fn parse_list_single_element() {
    assert_eq!(parse_list::<i32>("42", ','), Ok(vec![42]));
}

#[test]
fn parse_list_with_spaces() {
    assert_eq!(parse_list::<i32>(" 1 , 2 , 3 ", ','), Ok(vec![1, 2, 3]));
}

#[test]
fn parse_list_different_delimiter() {
    assert_eq!(parse_list::<i32>("1|2|3", '|'), Ok(vec![1, 2, 3]));
}

#[test]
fn parse_list_negative_numbers() {
    assert_eq!(parse_list::<i32>("-1,-2,-3", ','), Ok(vec![-1, -2, -3]));
}

#[test]
fn parse_list_invalid_element() {
    assert!(parse_list::<i32>("1,two,3", ',').is_err());
}

#[test]
fn parse_list_empty_element() {
    assert!(parse_list::<i32>("1,,3", ',').is_err());
}

#[test]
fn parse_list_unsigned() {
    assert_eq!(parse_list::<u32>("10,20,30", ','), Ok(vec![10, 20, 30]));
}
