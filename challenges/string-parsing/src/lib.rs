use std::str::FromStr;

/// Parse a string into an i32, returning a descriptive error message on failure.
///
/// # Examples
///
/// ```
/// use string_parsing::parse_int;
///
/// assert_eq!(parse_int("42"), Ok(42));
/// assert_eq!(parse_int("-17"), Ok(-17));
/// assert!(parse_int("not a number").is_err());
/// ```
pub fn parse_int(s: &str) -> Result<i32, String> {
    s.trim()
        .parse::<i32>()
        .map_err(|_| format!("'{}' is not a valid integer", s))
}

/// Parse common boolean representations (case-insensitive).
///
/// Accepts: "true", "false", "1", "0", "yes", "no"
///
/// # Examples
///
/// ```
/// use string_parsing::parse_bool;
///
/// assert_eq!(parse_bool("true"), Ok(true));
/// assert_eq!(parse_bool("YES"), Ok(true));
/// assert_eq!(parse_bool("0"), Ok(false));
/// assert!(parse_bool("maybe").is_err());
/// ```
pub fn parse_bool(s: &str) -> Result<bool, String> {
    match s.trim().to_lowercase().as_str() {
        "true" | "1" | "yes" => Ok(true),
        "false" | "0" | "no" => Ok(false),
        _ => Err(format!("'{}' is not a valid boolean", s)),
    }
}

/// Parse a "key=value" string into a tuple.
///
/// # Examples
///
/// ```
/// use string_parsing::parse_key_value;
///
/// assert_eq!(
///     parse_key_value("name=Alice"),
///     Ok(("name".to_string(), "Alice".to_string()))
/// );
/// ```
pub fn parse_key_value(s: &str) -> Result<(String, String), String> {
    let mut parts = s.splitn(2, '=');
    let key = parts.next().ok_or("Empty input")?;
    let value = parts.next().ok_or_else(|| format!("No '=' found in '{}'", s))?;
    Ok((key.trim().to_string(), value.trim().to_string()))
}

/// A color represented by red, green, and blue components.
#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FromStr for Color {
    type Err = String;

    /// Parse a color from "r,g,b" format.
    ///
    /// # Examples
    ///
    /// ```
    /// use string_parsing::Color;
    ///
    /// let color: Color = "255,128,0".parse().unwrap();
    /// assert_eq!(color.r, 255);
    /// assert_eq!(color.g, 128);
    /// assert_eq!(color.b, 0);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err(format!("Expected 'r,g,b' format, got '{}'", s));
        }
        let r = parts[0]
            .trim()
            .parse::<u8>()
            .map_err(|_| format!("Invalid red value: '{}'", parts[0].trim()))?;
        let g = parts[1]
            .trim()
            .parse::<u8>()
            .map_err(|_| format!("Invalid green value: '{}'", parts[1].trim()))?;
        let b = parts[2]
            .trim()
            .parse::<u8>()
            .map_err(|_| format!("Invalid blue value: '{}'", parts[2].trim()))?;
        Ok(Color { r, g, b })
    }
}

/// Parse a delimited list of values into a Vec.
///
/// # Examples
///
/// ```
/// use string_parsing::parse_list;
///
/// assert_eq!(parse_list::<i32>("1,2,3", ','), Ok(vec![1, 2, 3]));
/// assert_eq!(parse_list::<f64>("1.5;2.5;3.5", ';'), Ok(vec![1.5, 2.5, 3.5]));
/// ```
pub fn parse_list<T: FromStr>(s: &str, delimiter: char) -> Result<Vec<T>, String> {
    s.split(delimiter)
        .map(|part| {
            part.trim()
                .parse::<T>()
                .map_err(|_| format!("Failed to parse '{}'", part.trim()))
        })
        .collect()
}

pub fn main() {
    // Demonstrate parse_int
    println!("Parsing integers:");
    println!("  '42' -> {:?}", parse_int("42"));
    println!("  '-17' -> {:?}", parse_int("-17"));
    println!("  'abc' -> {:?}", parse_int("abc"));

    // Demonstrate parse_bool
    println!("\nParsing booleans:");
    println!("  'true' -> {:?}", parse_bool("true"));
    println!("  'YES' -> {:?}", parse_bool("YES"));
    println!("  '0' -> {:?}", parse_bool("0"));
    println!("  'maybe' -> {:?}", parse_bool("maybe"));

    // Demonstrate parse_key_value
    println!("\nParsing key=value pairs:");
    println!("  'name=Alice' -> {:?}", parse_key_value("name=Alice"));
    println!("  'count=42' -> {:?}", parse_key_value("count=42"));
    println!("  'invalid' -> {:?}", parse_key_value("invalid"));

    // Demonstrate Color parsing
    println!("\nParsing colors:");
    let color: Result<Color, _> = "255,128,0".parse();
    println!("  '255,128,0' -> {:?}", color);
    let invalid: Result<Color, _> = "256,0,0".parse();
    println!("  '256,0,0' -> {:?}", invalid);

    // Demonstrate parse_list
    println!("\nParsing lists:");
    println!("  '1,2,3' as i32 -> {:?}", parse_list::<i32>("1,2,3", ','));
    println!(
        "  '1.5;2.5;3.5' as f64 -> {:?}",
        parse_list::<f64>("1.5;2.5;3.5", ';')
    );
}
