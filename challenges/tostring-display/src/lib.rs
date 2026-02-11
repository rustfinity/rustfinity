use std::fmt;

/// A point in 2D space.
///
/// # Examples
///
/// ```
/// use tostring_display::Point;
///
/// let p = Point { x: 3, y: -4 };
/// assert_eq!(p.to_string(), "(3, -4)");
/// assert_eq!(format!("{}", p), "(3, -4)");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// A color represented as either a named color or custom RGB values.
///
/// # Examples
///
/// ```
/// use tostring_display::Color;
///
/// let red = Color::Red;
/// assert_eq!(red.to_string(), "Red");
///
/// let custom = Color::Custom(255, 128, 0);
/// assert_eq!(custom.to_string(), "RGB(255, 128, 0)");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Blue => write!(f, "Blue"),
            Color::Custom(r, g, b) => write!(f, "RGB({}, {}, {})", r, g, b),
        }
    }
}

/// A temperature in either Celsius or Fahrenheit.
///
/// # Examples
///
/// ```
/// use tostring_display::Temperature;
///
/// let celsius = Temperature::Celsius(25.5);
/// assert_eq!(celsius.to_string(), "25.5°C");
///
/// let fahrenheit = Temperature::Fahrenheit(77.0);
/// assert_eq!(fahrenheit.to_string(), "77°F");
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Temperature::Celsius(temp) => {
                // Format without trailing zeros
                let formatted = format_float(*temp);
                write!(f, "{}°C", formatted)
            }
            Temperature::Fahrenheit(temp) => {
                let formatted = format_float(*temp);
                write!(f, "{}°F", formatted)
            }
        }
    }
}

/// Helper function to format floats, removing unnecessary trailing zeros.
fn format_float(value: f64) -> String {
    // Format with enough precision, then trim trailing zeros
    let s = format!("{:.10}", value);
    let s = s.trim_end_matches('0');
    let s = s.trim_end_matches('.');
    s.to_string()
}

/// Money represented in cents with a currency code.
///
/// # Examples
///
/// ```
/// use tostring_display::Money;
///
/// let usd = Money { amount: 1234, currency: "USD".to_string() };
/// assert_eq!(usd.to_string(), "$12.34");
///
/// let eur = Money { amount: 5000, currency: "EUR".to_string() };
/// assert_eq!(eur.to_string(), "€50.00");
///
/// let gbp = Money { amount: 999, currency: "GBP".to_string() };
/// assert_eq!(gbp.to_string(), "9.99 GBP");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Money {
    pub amount: i64,
    pub currency: String,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dollars = self.amount.abs() / 100;
        let cents = self.amount.abs() % 100;
        let sign = if self.amount < 0 { "-" } else { "" };

        match self.currency.as_str() {
            "USD" => write!(f, "{}${}.{:02}", sign, dollars, cents),
            "EUR" => write!(f, "{}€{}.{:02}", sign, dollars, cents),
            other => write!(f, "{}{}.{:02} {}", sign, dollars, cents, other),
        }
    }
}

/// A person with a name and age.
///
/// # Examples
///
/// ```
/// use tostring_display::Person;
///
/// let person = Person { name: "Alice".to_string(), age: 30 };
/// assert_eq!(person.to_string(), "Alice (age 30)");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (age {})", self.name, self.age)
    }
}

/// Converts a slice of displayable items to a comma-separated string in square brackets.
///
/// # Examples
///
/// ```
/// use tostring_display::list_to_string;
///
/// let numbers = vec![1, 2, 3];
/// assert_eq!(list_to_string(&numbers), "[1, 2, 3]");
///
/// let empty: Vec<i32> = vec![];
/// assert_eq!(list_to_string(&empty), "[]");
///
/// let single = vec!["hello"];
/// assert_eq!(list_to_string(&single), "[hello]");
/// ```
pub fn list_to_string<T: fmt::Display>(items: &[T]) -> String {
    let items_str: Vec<String> = items.iter().map(|item| item.to_string()).collect();
    format!("[{}]", items_str.join(", "))
}

/// Formats data as a simple text table with headers and rows.
///
/// Each row is formatted with cells separated by " | ", and the header row
/// is followed by a separator line of dashes.
///
/// # Examples
///
/// ```
/// use tostring_display::format_table;
///
/// let headers = vec!["Name", "Age"];
/// let rows = vec![
///     vec!["Alice".to_string(), "30".to_string()],
///     vec!["Bob".to_string(), "25".to_string()],
/// ];
/// let table = format_table(&headers, &rows);
/// assert!(table.contains("Name | Age"));
/// assert!(table.contains("Alice | 30"));
/// assert!(table.contains("Bob | 25"));
/// ```
pub fn format_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let mut result = String::new();

    // Format header row
    let header_row = headers.join(" | ");
    result.push_str(&header_row);
    result.push('\n');

    // Add separator line
    let separator: Vec<String> = headers.iter().map(|h| "-".repeat(h.len())).collect();
    result.push_str(&separator.join("-+-"));
    result.push('\n');

    // Format data rows
    for row in rows {
        let row_str = row.join(" | ");
        result.push_str(&row_str);
        result.push('\n');
    }

    // Remove trailing newline
    result.trim_end().to_string()
}
