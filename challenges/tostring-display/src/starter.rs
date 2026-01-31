use std::fmt;

/// A point in 2D space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// TODO: Implement the Display trait for Point
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Use write!(f, ...) to format the point
        todo!()
    }
}

/// A color represented as either a named color or custom RGB values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

// TODO: Implement the Display trait for Color
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Match on self and format each variant appropriately
        todo!()
    }
}

/// A temperature in either Celsius or Fahrenheit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

// TODO: Implement the Display trait for Temperature
impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Match on self and format each variant
        todo!()
    }
}

/// Money represented in cents with a currency code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Money {
    pub amount: i64, // Amount in cents (can be negative)
    pub currency: String,
}

// TODO: Implement the Display trait for Money
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Convert cents to dollars/cents
        todo!()
    }
}

/// A person with a name and age.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

// TODO: Implement the Display trait for Person
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Format the person
        todo!()
    }
}

/// Converts a slice of displayable items to a comma-separated string in square brackets.
///
/// Examples:
/// - [1, 2, 3] -> "[1, 2, 3]"
/// - [] -> "[]"
/// - ["hello"] -> "[hello]"
pub fn list_to_string<T: fmt::Display>(items: &[T]) -> String {
    // TODO: Iterate over items, convert each to string, and join with ", "
    // Return the result wrapped in square brackets
    todo!()
}

/// Formats data as a simple text table with headers and rows.
///
/// Each row is formatted with cells separated by " | ", and the header row
/// is followed by a separator line of dashes.
///
/// Example output:
/// ```text
/// Name | Age
/// -----+----
/// Alice | 30
/// Bob | 25
/// ```
pub fn format_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    // TODO:
    // 1. Join headers with " | " and add to result
    // 2. Create a separator line with dashes (matching header lengths) joined by "-+-"
    // 3. Join each row with " | " and add to result
    // 4. Remove trailing newline and return
    todo!()
}

// Example usage
pub fn main() {
    // Point
    let point = Point { x: 3, y: -4 };
    println!("Point: {}", point);

    // Color
    let red = Color::Red;
    let custom = Color::Custom(255, 128, 0);
    println!("Colors: {}, {}", red, custom);

    // Temperature
    let celsius = Temperature::Celsius(25.5);
    let fahrenheit = Temperature::Fahrenheit(77.0);
    println!("Temperatures: {}, {}", celsius, fahrenheit);

    // Money
    let usd = Money {
        amount: 1234,
        currency: "USD".to_string(),
    };
    let eur = Money {
        amount: 5000,
        currency: "EUR".to_string(),
    };
    println!("Money: {}, {}", usd, eur);

    // Person
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    println!("Person: {}", person);

    // list_to_string
    let numbers = vec![1, 2, 3];
    println!("List: {}", list_to_string(&numbers));

    // format_table
    let headers = vec!["Name", "Age"];
    let rows = vec![
        vec!["Alice".to_string(), "30".to_string()],
        vec!["Bob".to_string(), "25".to_string()],
    ];
    println!("Table:\n{}", format_table(&headers, &rows));
}
