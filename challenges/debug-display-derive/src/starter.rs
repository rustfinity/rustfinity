use std::fmt;

/// A 2D coordinate with x and y values.
///
/// TODO: Derive Debug trait
/// TODO: Implement Display to format as "(x, y)"
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

// TODO: Implement Display for Coordinate
// The output should be "(x, y)" format, e.g., "(3.5, -2.0)"


/// A color representation supporting named colors, RGB, and hex values.
///
/// TODO: Derive Debug trait
/// TODO: Implement Display for each variant
pub enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
    Hex(String),
}

// TODO: Implement Display for Color
// - Red, Green, Blue -> "Red", "Green", "Blue"
// - Rgb(r, g, b) -> "rgb(r, g, b)"
// - Hex(s) -> the hex string itself


/// Temperature units.
///
/// TODO: Derive Debug trait
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

/// A temperature value with its unit.
///
/// TODO: Derive Debug trait
/// TODO: Implement Display to format as "value°C", "value°F", or "valueK"
pub struct Temperature {
    pub value: f64,
    pub unit: TemperatureUnit,
}

// TODO: Implement Display for Temperature
// - Celsius: "25.5°C"
// - Fahrenheit: "98.6°F"
// - Kelvin: "300K" (no degree symbol for Kelvin)


/// Log severity levels.
///
/// TODO: Derive Debug trait
/// TODO: Implement Display to show as uppercase: "ERROR", "WARNING", "INFO", "DEBUG"
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

// TODO: Implement Display for LogLevel
// Output should be uppercase: "ERROR", "WARNING", "INFO", "DEBUG"


/// A log message with a severity level and message text.
///
/// TODO: Derive Debug trait
/// TODO: Implement Display to format as "[LEVEL] message"
pub struct LogMessage {
    pub level: LogLevel,
    pub message: String,
}

// TODO: Implement Display for LogMessage
// Output should be "[LEVEL] message", e.g., "[ERROR] Connection failed"


/// Returns the debug representation of any Debug type.
///
/// Use format!("{:?}", value) to get the debug string.
pub fn debug_string<T: fmt::Debug>(value: &T) -> String {
    // TODO
    todo!()
}

/// Returns the display representation of any Display type.
///
/// Use format!("{}", value) to get the display string.
pub fn display_string<T: fmt::Display>(value: &T) -> String {
    // TODO
    todo!()
}

/// Returns the pretty-printed debug representation of any Debug type.
///
/// Use format!("{:#?}", value) to get the pretty-printed debug string.
pub fn pretty_debug<T: fmt::Debug>(value: &T) -> String {
    // TODO
    todo!()
}

// Example usage
pub fn main() {
    // Coordinate examples
    let coord = Coordinate { x: 3.5, y: -2.0 };
    println!("Coordinate Display: {}", coord);
    println!("Coordinate Debug: {:?}", coord);

    // Color examples
    let red = Color::Red;
    let custom = Color::Rgb(255, 128, 0);
    let hex = Color::Hex(String::from("#FF5733"));
    println!("Red: {}", red);
    println!("Custom RGB: {}", custom);
    println!("Hex color: {}", hex);

    // Temperature examples
    let temp = Temperature { value: 25.5, unit: TemperatureUnit::Celsius };
    println!("Temperature: {}", temp);

    // LogMessage examples
    let log = LogMessage {
        level: LogLevel::Error,
        message: String::from("Connection failed"),
    };
    println!("Log: {}", log);

    // Utility function examples
    println!("Debug string: {}", debug_string(&coord));
    println!("Display string: {}", display_string(&coord));
    println!("Pretty debug: {}", pretty_debug(&coord));
}
