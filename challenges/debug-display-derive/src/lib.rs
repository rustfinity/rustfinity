use std::fmt;

/// A 2D coordinate with x and y values.
///
/// # Examples
///
/// ```
/// use debug_display_derive::Coordinate;
///
/// let coord = Coordinate { x: 3.5, y: -2.5 };
/// assert_eq!(format!("{}", coord), "(3.5, -2.5)");
/// assert_eq!(format!("{:?}", coord), "Coordinate { x: 3.5, y: -2.5 }");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// A color representation supporting named colors, RGB, and hex values.
///
/// # Examples
///
/// ```
/// use debug_display_derive::Color;
///
/// let red = Color::Red;
/// assert_eq!(format!("{}", red), "Red");
///
/// let custom = Color::Rgb(255, 128, 0);
/// assert_eq!(format!("{}", custom), "rgb(255, 128, 0)");
///
/// let hex = Color::Hex(String::from("#FF5733"));
/// assert_eq!(format!("{}", hex), "#FF5733");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
    Hex(String),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Blue => write!(f, "Blue"),
            Color::Rgb(r, g, b) => write!(f, "rgb({}, {}, {})", r, g, b),
            Color::Hex(hex) => write!(f, "{}", hex),
        }
    }
}

/// Temperature units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

/// A temperature value with its unit.
///
/// # Examples
///
/// ```
/// use debug_display_derive::{Temperature, TemperatureUnit};
///
/// let temp = Temperature { value: 25.5, unit: TemperatureUnit::Celsius };
/// assert_eq!(format!("{}", temp), "25.5°C");
///
/// let temp_f = Temperature { value: 98.6, unit: TemperatureUnit::Fahrenheit };
/// assert_eq!(format!("{}", temp_f), "98.6°F");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Temperature {
    pub value: f64,
    pub unit: TemperatureUnit,
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.unit {
            TemperatureUnit::Celsius => write!(f, "{}°C", self.value),
            TemperatureUnit::Fahrenheit => write!(f, "{}°F", self.value),
            TemperatureUnit::Kelvin => write!(f, "{}K", self.value),
        }
    }
}

/// Log severity levels.
///
/// # Examples
///
/// ```
/// use debug_display_derive::LogLevel;
///
/// let level = LogLevel::Error;
/// assert_eq!(format!("{}", level), "ERROR");
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Debug => write!(f, "DEBUG"),
        }
    }
}

/// A log message with a severity level and message text.
///
/// # Examples
///
/// ```
/// use debug_display_derive::{LogMessage, LogLevel};
///
/// let log = LogMessage {
///     level: LogLevel::Error,
///     message: String::from("Connection failed"),
/// };
/// assert_eq!(format!("{}", log), "[ERROR] Connection failed");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct LogMessage {
    pub level: LogLevel,
    pub message: String,
}

impl fmt::Display for LogMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.level, self.message)
    }
}

/// Returns the debug representation of any Debug type.
///
/// # Examples
///
/// ```
/// use debug_display_derive::{debug_string, Coordinate};
///
/// let coord = Coordinate { x: 1.0, y: 2.0 };
/// assert_eq!(debug_string(&coord), "Coordinate { x: 1.0, y: 2.0 }");
/// ```
pub fn debug_string<T: fmt::Debug>(value: &T) -> String {
    format!("{:?}", value)
}

/// Returns the display representation of any Display type.
///
/// # Examples
///
/// ```
/// use debug_display_derive::{display_string, Coordinate};
///
/// let coord = Coordinate { x: 1.5, y: 2.5 };
/// assert_eq!(display_string(&coord), "(1.5, 2.5)");
/// ```
pub fn display_string<T: fmt::Display>(value: &T) -> String {
    format!("{}", value)
}

/// Returns the pretty-printed debug representation of any Debug type.
///
/// # Examples
///
/// ```
/// use debug_display_derive::{pretty_debug, Coordinate};
///
/// let coord = Coordinate { x: 1.0, y: 2.0 };
/// let pretty = pretty_debug(&coord);
/// assert!(pretty.contains("Coordinate"));
/// assert!(pretty.contains("x:"));
/// ```
pub fn pretty_debug<T: fmt::Debug>(value: &T) -> String {
    format!("{:#?}", value)
}
