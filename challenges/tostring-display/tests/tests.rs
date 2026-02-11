use tostring_display::*;

// ==================== Point Tests ====================

#[test]
fn point_positive_coordinates() {
    let p = Point { x: 3, y: 4 };
    assert_eq!(p.to_string(), "(3, 4)");
}

#[test]
fn point_negative_coordinates() {
    let p = Point { x: -3, y: -4 };
    assert_eq!(p.to_string(), "(-3, -4)");
}

#[test]
fn point_mixed_coordinates() {
    let p = Point { x: 3, y: -4 };
    assert_eq!(p.to_string(), "(3, -4)");
}

#[test]
fn point_zero_coordinates() {
    let p = Point { x: 0, y: 0 };
    assert_eq!(p.to_string(), "(0, 0)");
}

#[test]
fn point_format_macro() {
    let p = Point { x: 10, y: 20 };
    assert_eq!(format!("{}", p), "(10, 20)");
}

#[test]
fn point_large_values() {
    let p = Point { x: i32::MAX, y: i32::MIN };
    assert_eq!(p.to_string(), format!("({}, {})", i32::MAX, i32::MIN));
}

// ==================== Color Tests ====================

#[test]
fn color_red() {
    let c = Color::Red;
    assert_eq!(c.to_string(), "Red");
}

#[test]
fn color_green() {
    let c = Color::Green;
    assert_eq!(c.to_string(), "Green");
}

#[test]
fn color_blue() {
    let c = Color::Blue;
    assert_eq!(c.to_string(), "Blue");
}

#[test]
fn color_custom() {
    let c = Color::Custom(255, 128, 0);
    assert_eq!(c.to_string(), "RGB(255, 128, 0)");
}

#[test]
fn color_custom_black() {
    let c = Color::Custom(0, 0, 0);
    assert_eq!(c.to_string(), "RGB(0, 0, 0)");
}

#[test]
fn color_custom_white() {
    let c = Color::Custom(255, 255, 255);
    assert_eq!(c.to_string(), "RGB(255, 255, 255)");
}

#[test]
fn color_format_macro() {
    let c = Color::Custom(100, 150, 200);
    assert_eq!(format!("{}", c), "RGB(100, 150, 200)");
}

// ==================== Temperature Tests ====================

#[test]
fn temperature_celsius_integer() {
    let t = Temperature::Celsius(25.0);
    assert_eq!(t.to_string(), "25°C");
}

#[test]
fn temperature_celsius_decimal() {
    let t = Temperature::Celsius(25.5);
    assert_eq!(t.to_string(), "25.5°C");
}

#[test]
fn temperature_celsius_negative() {
    let t = Temperature::Celsius(-10.0);
    assert_eq!(t.to_string(), "-10°C");
}

#[test]
fn temperature_celsius_zero() {
    let t = Temperature::Celsius(0.0);
    assert_eq!(t.to_string(), "0°C");
}

#[test]
fn temperature_fahrenheit_integer() {
    let t = Temperature::Fahrenheit(77.0);
    assert_eq!(t.to_string(), "77°F");
}

#[test]
fn temperature_fahrenheit_decimal() {
    let t = Temperature::Fahrenheit(98.6);
    assert_eq!(t.to_string(), "98.6°F");
}

#[test]
fn temperature_fahrenheit_negative() {
    let t = Temperature::Fahrenheit(-40.0);
    assert_eq!(t.to_string(), "-40°F");
}

#[test]
fn temperature_format_macro() {
    let t = Temperature::Celsius(100.0);
    assert_eq!(format!("{}", t), "100°C");
}

// ==================== Money Tests ====================

#[test]
fn money_usd_simple() {
    let m = Money {
        amount: 1234,
        currency: "USD".to_string(),
    };
    assert_eq!(m.to_string(), "$12.34");
}

#[test]
fn money_usd_whole_dollars() {
    let m = Money {
        amount: 5000,
        currency: "USD".to_string(),
    };
    assert_eq!(m.to_string(), "$50.00");
}

#[test]
fn money_usd_cents_only() {
    let m = Money {
        amount: 99,
        currency: "USD".to_string(),
    };
    assert_eq!(m.to_string(), "$0.99");
}

#[test]
fn money_usd_zero() {
    let m = Money {
        amount: 0,
        currency: "USD".to_string(),
    };
    assert_eq!(m.to_string(), "$0.00");
}

#[test]
fn money_usd_negative() {
    let m = Money {
        amount: -1234,
        currency: "USD".to_string(),
    };
    assert_eq!(m.to_string(), "-$12.34");
}

#[test]
fn money_eur_simple() {
    let m = Money {
        amount: 5000,
        currency: "EUR".to_string(),
    };
    assert_eq!(m.to_string(), "€50.00");
}

#[test]
fn money_eur_negative() {
    let m = Money {
        amount: -999,
        currency: "EUR".to_string(),
    };
    assert_eq!(m.to_string(), "-€9.99");
}

#[test]
fn money_other_currency() {
    let m = Money {
        amount: 999,
        currency: "GBP".to_string(),
    };
    assert_eq!(m.to_string(), "9.99 GBP");
}

#[test]
fn money_other_currency_large() {
    let m = Money {
        amount: 123456,
        currency: "JPY".to_string(),
    };
    assert_eq!(m.to_string(), "1234.56 JPY");
}

#[test]
fn money_other_currency_negative() {
    let m = Money {
        amount: -500,
        currency: "CAD".to_string(),
    };
    assert_eq!(m.to_string(), "-5.00 CAD");
}

#[test]
fn money_format_macro() {
    let m = Money {
        amount: 4200,
        currency: "USD".to_string(),
    };
    assert_eq!(format!("{}", m), "$42.00");
}

// ==================== Person Tests ====================

#[test]
fn person_basic() {
    let p = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    assert_eq!(p.to_string(), "Alice (age 30)");
}

#[test]
fn person_zero_age() {
    let p = Person {
        name: "Baby".to_string(),
        age: 0,
    };
    assert_eq!(p.to_string(), "Baby (age 0)");
}

#[test]
fn person_long_name() {
    let p = Person {
        name: "Alexander Hamilton".to_string(),
        age: 47,
    };
    assert_eq!(p.to_string(), "Alexander Hamilton (age 47)");
}

#[test]
fn person_old_age() {
    let p = Person {
        name: "Elder".to_string(),
        age: 120,
    };
    assert_eq!(p.to_string(), "Elder (age 120)");
}

#[test]
fn person_format_macro() {
    let p = Person {
        name: "Bob".to_string(),
        age: 25,
    };
    assert_eq!(format!("{}", p), "Bob (age 25)");
}

// ==================== list_to_string Tests ====================

#[test]
fn list_to_string_integers() {
    let numbers = vec![1, 2, 3];
    assert_eq!(list_to_string(&numbers), "[1, 2, 3]");
}

#[test]
fn list_to_string_empty() {
    let empty: Vec<i32> = vec![];
    assert_eq!(list_to_string(&empty), "[]");
}

#[test]
fn list_to_string_single() {
    let single = vec![42];
    assert_eq!(list_to_string(&single), "[42]");
}

#[test]
fn list_to_string_strings() {
    let strings = vec!["hello", "world"];
    assert_eq!(list_to_string(&strings), "[hello, world]");
}

#[test]
fn list_to_string_points() {
    let points = vec![
        Point { x: 1, y: 2 },
        Point { x: 3, y: 4 },
    ];
    assert_eq!(list_to_string(&points), "[(1, 2), (3, 4)]");
}

#[test]
fn list_to_string_colors() {
    let colors = vec![Color::Red, Color::Green, Color::Custom(128, 128, 128)];
    assert_eq!(list_to_string(&colors), "[Red, Green, RGB(128, 128, 128)]");
}

#[test]
fn list_to_string_many_elements() {
    let numbers: Vec<i32> = (1..=10).collect();
    assert_eq!(list_to_string(&numbers), "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]");
}

// ==================== format_table Tests ====================

#[test]
fn format_table_basic() {
    let headers = vec!["Name", "Age"];
    let rows = vec![
        vec!["Alice".to_string(), "30".to_string()],
        vec!["Bob".to_string(), "25".to_string()],
    ];
    let table = format_table(&headers, &rows);
    assert!(table.contains("Name | Age"));
    assert!(table.contains("Alice | 30"));
    assert!(table.contains("Bob | 25"));
}

#[test]
fn format_table_single_row() {
    let headers = vec!["Item", "Price"];
    let rows = vec![vec!["Apple".to_string(), "$1.00".to_string()]];
    let table = format_table(&headers, &rows);
    assert!(table.contains("Item | Price"));
    assert!(table.contains("Apple | $1.00"));
}

#[test]
fn format_table_empty_rows() {
    let headers = vec!["Col1", "Col2"];
    let rows: Vec<Vec<String>> = vec![];
    let table = format_table(&headers, &rows);
    assert!(table.contains("Col1 | Col2"));
    // Should still have header and separator
}

#[test]
fn format_table_single_column() {
    let headers = vec!["Name"];
    let rows = vec![
        vec!["Alice".to_string()],
        vec!["Bob".to_string()],
    ];
    let table = format_table(&headers, &rows);
    assert!(table.contains("Name"));
    assert!(table.contains("Alice"));
    assert!(table.contains("Bob"));
}

#[test]
fn format_table_three_columns() {
    let headers = vec!["ID", "Name", "Status"];
    let rows = vec![
        vec!["1".to_string(), "Task A".to_string(), "Done".to_string()],
        vec!["2".to_string(), "Task B".to_string(), "Pending".to_string()],
    ];
    let table = format_table(&headers, &rows);
    assert!(table.contains("ID | Name | Status"));
    assert!(table.contains("1 | Task A | Done"));
    assert!(table.contains("2 | Task B | Pending"));
}

#[test]
fn format_table_has_separator() {
    let headers = vec!["A", "B"];
    let rows = vec![vec!["x".to_string(), "y".to_string()]];
    let table = format_table(&headers, &rows);
    // Check that there's a separator line with dashes
    assert!(table.contains("-"));
}

// ==================== Integration Tests ====================

#[test]
fn integration_display_in_format_string() {
    let point = Point { x: 5, y: 10 };
    let person = Person {
        name: "Alice".to_string(),
        age: 25,
    };
    let message = format!("Person {} is at {}", person, point);
    assert_eq!(message, "Person Alice (age 25) is at (5, 10)");
}

#[test]
fn integration_nested_display() {
    let temp = Temperature::Celsius(20.0);
    let message = format!("Current temperature: {}", temp);
    assert_eq!(message, "Current temperature: 20°C");
}

#[test]
fn integration_list_of_custom_types() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
    ];
    let result = list_to_string(&people);
    assert_eq!(result, "[Alice (age 30), Bob (age 25)]");
}

#[test]
fn integration_money_in_table() {
    let headers = vec!["Item", "Price"];
    let m1 = Money { amount: 1099, currency: "USD".to_string() };
    let m2 = Money { amount: 2500, currency: "USD".to_string() };
    let rows = vec![
        vec!["Coffee".to_string(), m1.to_string()],
        vec!["Sandwich".to_string(), m2.to_string()],
    ];
    let table = format_table(&headers, &rows);
    assert!(table.contains("Coffee | $10.99"));
    assert!(table.contains("Sandwich | $25.00"));
}

#[test]
fn integration_temperatures_list() {
    let temps = vec![
        Temperature::Celsius(0.0),
        Temperature::Celsius(25.0),
        Temperature::Celsius(100.0),
    ];
    let result = list_to_string(&temps);
    assert_eq!(result, "[0°C, 25°C, 100°C]");
}

#[test]
fn integration_to_string_vs_format() {
    let color = Color::Custom(255, 0, 128);
    // Both should produce the same result
    assert_eq!(color.to_string(), format!("{}", color));

    let money = Money { amount: 4567, currency: "EUR".to_string() };
    assert_eq!(money.to_string(), format!("{}", money));
}

#[test]
fn integration_display_trait_object() {
    // Test that Display works with trait objects
    let point = Point { x: 1, y: 2 };
    let displayable: &dyn std::fmt::Display = &point;
    assert_eq!(format!("{}", displayable), "(1, 2)");
}
