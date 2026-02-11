use string_building::*;

// ============ build_greeting tests ============

#[test]
fn greeting_basic() {
    assert_eq!(
        build_greeting("Alice", 30),
        "Hello, Alice! You are 30 years old."
    );
}

#[test]
fn greeting_different_name() {
    assert_eq!(
        build_greeting("Bob", 25),
        "Hello, Bob! You are 25 years old."
    );
}

#[test]
fn greeting_zero_age() {
    assert_eq!(
        build_greeting("Baby", 0),
        "Hello, Baby! You are 0 years old."
    );
}

#[test]
fn greeting_large_age() {
    assert_eq!(
        build_greeting("Elder", 100),
        "Hello, Elder! You are 100 years old."
    );
}

#[test]
fn greeting_empty_name() {
    assert_eq!(build_greeting("", 42), "Hello, ! You are 42 years old.");
}

// ============ build_list tests ============

#[test]
fn list_three_items() {
    assert_eq!(
        build_list(&["apple", "banana", "cherry"]),
        "1. apple\n2. banana\n3. cherry"
    );
}

#[test]
fn list_single_item() {
    assert_eq!(build_list(&["only"]), "1. only");
}

#[test]
fn list_empty() {
    assert_eq!(build_list(&[]), "");
}

#[test]
fn list_two_items() {
    assert_eq!(build_list(&["first", "second"]), "1. first\n2. second");
}

#[test]
fn list_many_items() {
    let items = &["a", "b", "c", "d", "e"];
    let expected = "1. a\n2. b\n3. c\n4. d\n5. e";
    assert_eq!(build_list(items), expected);
}

#[test]
fn list_with_spaces() {
    assert_eq!(
        build_list(&["hello world", "foo bar"]),
        "1. hello world\n2. foo bar"
    );
}

// ============ Person Display tests ============

#[test]
fn person_display_basic() {
    let person = Person {
        name: "Bob".to_string(),
        age: 25,
    };
    assert_eq!(format!("{}", person), "Bob (25 years old)");
}

#[test]
fn person_display_zero_age() {
    let person = Person {
        name: "Baby".to_string(),
        age: 0,
    };
    assert_eq!(format!("{}", person), "Baby (0 years old)");
}

#[test]
fn person_display_long_name() {
    let person = Person {
        name: "Alexander Hamilton".to_string(),
        age: 47,
    };
    assert_eq!(format!("{}", person), "Alexander Hamilton (47 years old)");
}

#[test]
fn person_in_format_string() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    let message = format!("Welcome, {}!", person);
    assert_eq!(message, "Welcome, Alice (30 years old)!");
}

#[test]
fn person_to_string() {
    let person = Person {
        name: "Charlie".to_string(),
        age: 35,
    };
    let s = person.to_string();
    assert_eq!(s, "Charlie (35 years old)");
}

// ============ build_table tests ============

#[test]
fn table_basic() {
    let headers = &["Name", "Age"];
    let rows = vec![
        vec!["Alice".to_string(), "30".to_string()],
        vec!["Bob".to_string(), "25".to_string()],
    ];
    let table = build_table(headers, &rows);

    assert!(table.contains("| Name"));
    assert!(table.contains("| Age"));
    assert!(table.contains("| Alice"));
    assert!(table.contains("| 30"));
    assert!(table.contains("| Bob"));
    assert!(table.contains("| 25"));
    assert!(table.contains("|---"));
}

#[test]
fn table_single_row() {
    let headers = &["Item"];
    let rows = vec![vec!["Test".to_string()]];
    let table = build_table(headers, &rows);

    assert!(table.contains("| Item"));
    assert!(table.contains("| Test"));
}

#[test]
fn table_empty_rows() {
    let headers = &["A", "B"];
    let rows: Vec<Vec<String>> = vec![];
    let table = build_table(headers, &rows);

    assert!(table.contains("| A"));
    assert!(table.contains("| B"));
    assert!(table.contains("|---"));
}

#[test]
fn table_empty_headers() {
    let headers: &[&str] = &[];
    let rows: Vec<Vec<String>> = vec![];
    let table = build_table(headers, &rows);
    assert_eq!(table, "");
}

#[test]
fn table_column_width_from_data() {
    let headers = &["N", "Name"];
    let rows = vec![vec!["1".to_string(), "Christopher".to_string()]];
    let table = build_table(headers, &rows);

    // Christopher is longer than "Name", so column should accommodate it
    assert!(table.contains("Christopher"));
}

#[test]
fn table_three_columns() {
    let headers = &["ID", "Name", "Score"];
    let rows = vec![
        vec!["1".to_string(), "Alice".to_string(), "95".to_string()],
        vec!["2".to_string(), "Bob".to_string(), "87".to_string()],
    ];
    let table = build_table(headers, &rows);

    assert!(table.contains("| ID"));
    assert!(table.contains("| Name"));
    assert!(table.contains("| Score"));
    assert!(table.contains("| 1"));
    assert!(table.contains("| Alice"));
    assert!(table.contains("| 95"));
}

#[test]
fn table_alignment_check() {
    let headers = &["A", "BB"];
    let rows = vec![
        vec!["x".to_string(), "yy".to_string()],
        vec!["xx".to_string(), "y".to_string()],
    ];
    let table = build_table(headers, &rows);

    // Table should have consistent formatting with pipes
    let lines: Vec<&str> = table.lines().collect();
    assert!(lines.len() >= 3); // Header, separator, at least one row
    for line in lines {
        assert!(line.starts_with('|'));
        assert!(line.ends_with('|'));
    }
}

// ============ concat_with_separator tests ============

#[test]
fn concat_basic() {
    assert_eq!(concat_with_separator(&["a", "b", "c"], ", "), "a, b, c");
}

#[test]
fn concat_single_item() {
    assert_eq!(concat_with_separator(&["hello"], "-"), "hello");
}

#[test]
fn concat_empty() {
    assert_eq!(concat_with_separator(&[], ", "), "");
}

#[test]
fn concat_two_items() {
    assert_eq!(concat_with_separator(&["foo", "bar"], " - "), "foo - bar");
}

#[test]
fn concat_empty_separator() {
    assert_eq!(concat_with_separator(&["a", "b", "c"], ""), "abc");
}

#[test]
fn concat_longer_separator() {
    assert_eq!(
        concat_with_separator(&["x", "y"], " <=> "),
        "x <=> y"
    );
}

#[test]
fn concat_with_newlines() {
    assert_eq!(
        concat_with_separator(&["line1", "line2", "line3"], "\n"),
        "line1\nline2\nline3"
    );
}

#[test]
fn concat_with_empty_strings() {
    assert_eq!(concat_with_separator(&["", "", ""], ","), ",,");
}

#[test]
fn concat_words() {
    assert_eq!(
        concat_with_separator(&["hello", "world", "rust"], " "),
        "hello world rust"
    );
}
