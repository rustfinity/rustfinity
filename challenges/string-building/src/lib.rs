use std::fmt::{self, Display, Formatter, Write};

/// Build a greeting message using format!
///
/// # Examples
///
/// ```
/// use string_building::build_greeting;
///
/// assert_eq!(
///     build_greeting("Alice", 30),
///     "Hello, Alice! You are 30 years old."
/// );
/// ```
pub fn build_greeting(name: &str, age: u32) -> String {
    format!("Hello, {}! You are {} years old.", name, age)
}

/// Build a numbered list from items using write!
///
/// # Examples
///
/// ```
/// use string_building::build_list;
///
/// assert_eq!(
///     build_list(&["apple", "banana", "cherry"]),
///     "1. apple\n2. banana\n3. cherry"
/// );
/// ```
pub fn build_list(items: &[&str]) -> String {
    let mut output = String::new();
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            output.push('\n');
        }
        write!(output, "{}. {}", i + 1, item).unwrap();
    }
    output
}

/// A person with a name and age.
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Display for Person {
    /// Format the person as "Name (Age years old)"
    ///
    /// # Examples
    ///
    /// ```
    /// use string_building::Person;
    ///
    /// let person = Person { name: "Bob".to_string(), age: 25 };
    /// assert_eq!(format!("{}", person), "Bob (25 years old)");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} years old)", self.name, self.age)
    }
}

/// Build a simple text table with headers and data rows.
///
/// The table format uses pipes and dashes:
/// ```text
/// | Name  | Age |
/// |-------|-----|
/// | Alice | 30  |
/// | Bob   | 25  |
/// ```
///
/// # Examples
///
/// ```
/// use string_building::build_table;
///
/// let headers = &["Name", "Age"];
/// let rows = vec![
///     vec!["Alice".to_string(), "30".to_string()],
///     vec!["Bob".to_string(), "25".to_string()],
/// ];
/// let table = build_table(headers, &rows);
/// assert!(table.contains("| Name"));
/// assert!(table.contains("| Alice"));
/// ```
pub fn build_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    if headers.is_empty() {
        return String::new();
    }

    // Calculate column widths (max of header and all row values)
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i < widths.len() && cell.len() > widths[i] {
                widths[i] = cell.len();
            }
        }
    }

    let mut output = String::new();

    // Build header row
    output.push('|');
    for (i, header) in headers.iter().enumerate() {
        write!(output, " {:<width$} |", header, width = widths[i]).unwrap();
    }
    output.push('\n');

    // Build separator row
    output.push('|');
    for width in &widths {
        write!(output, "-{}-|", "-".repeat(*width)).unwrap();
    }
    output.push('\n');

    // Build data rows
    for row in rows {
        output.push('|');
        for (i, cell) in row.iter().enumerate() {
            let width = widths.get(i).copied().unwrap_or(0);
            write!(output, " {:<width$} |", cell, width = width).unwrap();
        }
        output.push('\n');
    }

    // Remove trailing newline for cleaner output
    if output.ends_with('\n') {
        output.pop();
    }

    output
}

/// Concatenate strings with a separator without using .join()
///
/// # Examples
///
/// ```
/// use string_building::concat_with_separator;
///
/// assert_eq!(
///     concat_with_separator(&["a", "b", "c"], ", "),
///     "a, b, c"
/// );
/// assert_eq!(concat_with_separator(&["hello"], "-"), "hello");
/// assert_eq!(concat_with_separator(&[], ", "), "");
/// ```
pub fn concat_with_separator(parts: &[&str], sep: &str) -> String {
    let mut output = String::new();
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            output.push_str(sep);
        }
        output.push_str(part);
    }
    output
}

pub fn main() {
    // Demonstrate build_greeting
    println!("=== build_greeting ===");
    println!("{}", build_greeting("Alice", 30));
    println!("{}", build_greeting("Bob", 25));

    // Demonstrate build_list
    println!("\n=== build_list ===");
    println!("{}", build_list(&["apple", "banana", "cherry"]));

    // Demonstrate Person Display
    println!("\n=== Person Display ===");
    let person = Person {
        name: "Charlie".to_string(),
        age: 35,
    };
    println!("{}", person);

    // Demonstrate build_table
    println!("\n=== build_table ===");
    let headers = &["Name", "Age", "City"];
    let rows = vec![
        vec![
            "Alice".to_string(),
            "30".to_string(),
            "New York".to_string(),
        ],
        vec![
            "Bob".to_string(),
            "25".to_string(),
            "San Francisco".to_string(),
        ],
        vec![
            "Charlie".to_string(),
            "35".to_string(),
            "Chicago".to_string(),
        ],
    ];
    println!("{}", build_table(headers, &rows));

    // Demonstrate concat_with_separator
    println!("\n=== concat_with_separator ===");
    println!("{}", concat_with_separator(&["a", "b", "c"], ", "));
    println!("{}", concat_with_separator(&["hello", "world"], " - "));
}
