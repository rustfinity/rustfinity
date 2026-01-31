use std::fmt::{self, Display, Formatter, Write};

/// Build a greeting message using format!
pub fn build_greeting(name: &str, age: u32) -> String {
    // TODO: Use format!() to create a greeting
    // Format: "Hello, {name}! You are {age} years old."
    unimplemented!()
}

/// Build a numbered list from items using write!
pub fn build_list(items: &[&str]) -> String {
    // TODO: Create a numbered list using std::fmt::Write
    // Format: "1. item1\n2. item2\n3. item3"
    unimplemented!()
}

/// A person with a name and age.
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

// TODO: Implement Display for Person
// Format: "Name (Age years old)"
impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO: Write the person's info in the format
        unimplemented!()
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
pub fn build_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    // TODO: Build a formatted table
    // Steps:
    // 1. Calculate column widths (max of header and cell values)
    // 2. Build header row with pipes: | Header1 | Header2 |
    // 3. Build separator row: |---------|---------|
    // 4. Build each data row: | Value1  | Value2  |
    unimplemented!()
}

/// Concatenate strings with a separator without using .join()
pub fn concat_with_separator(parts: &[&str], sep: &str) -> String {
    // TODO: Join the parts with the separator
    // Don't use the .join() method - implement it manually
    unimplemented!()
}

pub fn main() {
    // Demonstrate build_greeting
    println!("=== build_greeting ===");
    println!("{}", build_greeting("Alice", 30));

    // Demonstrate build_list
    println!("\n=== build_list ===");
    println!("{}", build_list(&["apple", "banana", "cherry"]));

    // Demonstrate Person Display
    println!("\n=== Person Display ===");
    let person = Person {
        name: "Bob".to_string(),
        age: 25,
    };
    println!("{}", person);

    // Demonstrate build_table
    println!("\n=== build_table ===");
    let headers = &["Name", "Age"];
    let rows = vec![
        vec!["Alice".to_string(), "30".to_string()],
        vec!["Bob".to_string(), "25".to_string()],
    ];
    println!("{}", build_table(headers, &rows));

    // Demonstrate concat_with_separator
    println!("\n=== concat_with_separator ===");
    println!("{}", concat_with_separator(&["a", "b", "c"], ", "));
}
