In Rust, the `Debug` and `Display` traits control how types are formatted as strings. `Debug` is intended for programmer-facing output (useful for debugging and logging), while `Display` is for user-facing output. Understanding when and how to use each is essential for writing clear, professional Rust code.

The `Debug` trait can almost always be derived automatically using `#[derive(Debug)]`, which generates a representation showing the type name and all field values. This is incredibly useful during development - you can print any `Debug` type using `{:?}` (compact) or `{:#?}` (pretty-printed with indentation).

The `Display` trait, however, must be implemented manually because there's no single "correct" way to display a type to users. You implement `Display` by writing a `fmt` method that uses format macros to build the output string. Once you implement `Display`, you automatically get a `to_string()` method for free.

## The Debug and Display Traits

```rust
use std::fmt;

// Debug - can be derived
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Display - must be implemented manually
impl fmt::Display for Point {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>
    ) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

let p = Point { x: 3, y: 4 };
println!("{:?}", p);   // Debug: Point { x: 3, y: 4 }
println!("{}", p);     // Display: (3, 4)
```

## Format Specifiers

- `{}` - Uses `Display` trait
- `{:?}` - Uses `Debug` trait (compact)
- `{:#?}` - Uses `Debug` trait (pretty-printed)

## Your Task

Implement the following types demonstrating `Debug` and `Display`:

### 1. Coordinate Struct

Create a `Coordinate` struct with `x` and `y` fields (both `f64`).

- Derive `Debug`
- Implement `Display` to format as `"(x, y)"` (e.g., `"(3.5, -2.0)"`)

### 2. Color Enum

Create a `Color` enum with variants:

- `Red`, `Green`, `Blue` (unit variants)
- `Rgb(u8, u8, u8)` (RGB values)
- `Hex(String)` (hex string like "#FF5733")

- Derive `Debug`
- Implement `Display`:
  - Unit variants display as their name: `"Red"`, `"Green"`, `"Blue"`
  - `Rgb` displays as `"rgb(r, g, b)"`
    (e.g., `"rgb(255, 128, 0)"`)
  - `Hex` displays as the hex string (e.g., `"#FF5733"`)

### 3. Temperature Struct

Create a `Temperature` struct that holds a value (`f64`) and a unit (`TemperatureUnit` enum).

The `TemperatureUnit` enum should have variants: `Celsius`, `Fahrenheit`, `Kelvin`.

- Derive `Debug` for both
- Implement `Display` for `Temperature` to format
  as `"value°C"`, `"value°F"`, or `"valueK"`
  (e.g., `"25.5°C"`, `"98.6°F"`, `"300K"`)

### 4. LogLevel Enum

Create a `LogLevel` enum with variants: `Error`, `Warning`, `Info`, `Debug`.

- Derive `Debug`
- Implement `Display` to show as uppercase strings:
  `"ERROR"`, `"WARNING"`, `"INFO"`, `"DEBUG"`

### 5. LogMessage Struct

Create a `LogMessage` struct with:

- `level`: `LogLevel`
- `message`: `String`

- Derive `Debug`
- Implement `Display` to format as
  `"[LEVEL] message"` (e.g.,
  `"[ERROR] Connection failed"`)

### 6. Utility Functions

Implement these functions to work with Debug and Display:

- `debug_string<T: std::fmt::Debug>(
    value: &T
) -> String`
  - Returns the debug representation of any `Debug` type
- `display_string<T: std::fmt::Display>(
    value: &T
) -> String`
  - Returns the display representation of any
    `Display` type
- `pretty_debug<T: std::fmt::Debug>(
    value: &T
) -> String`
  - Returns the pretty-printed debug
    representation (with `{:#?}`)

## Examples

```rust
// Coordinate
let coord = Coordinate { x: 3.5, y: -2.0 };
assert_eq!(format!("{}", coord), "(3.5, -2.0)");
assert_eq!(
    format!("{:?}", coord),
    "Coordinate { x: 3.5, y: -2.0 }"
);

// Color
let red = Color::Red;
let custom = Color::Rgb(255, 128, 0);
let hex = Color::Hex(String::from("#FF5733"));
assert_eq!(format!("{}", red), "Red");
assert_eq!(format!("{}", custom), "rgb(255, 128, 0)");
assert_eq!(format!("{}", hex), "#FF5733");

// Temperature
let temp = Temperature {
    value: 25.5,
    unit: TemperatureUnit::Celsius
};
assert_eq!(format!("{}", temp), "25.5°C");

// LogMessage
let log = LogMessage {
    level: LogLevel::Error,
    message: String::from("Connection failed"),
};
assert_eq!(
    format!("{}", log),
    "[ERROR] Connection failed"
);

// Utility functions
assert_eq!(
    debug_string(&coord),
    "Coordinate { x: 3.5, y: -2.0 }"
);
assert_eq!(display_string(&coord), "(3.5, -2.0)");
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `#[derive(Debug)]` on structs and enums for
  automatic Debug implementation
- Implement `Display` using
  `impl std::fmt::Display for TypeName {
    fn fmt(...)
}`
- Use `write!(f, "format string", args)` inside
  the `fmt` method
- For enums, use `match self { ... }` to handle
  each variant
- The degree symbol `°` is a valid UTF-8
  character you can include directly in strings
- `format!("{:?}", value)` returns the Debug
  string
- `format!("{:#?}", value)` returns the pretty-
  printed Debug string
- `format!("{}", value)` returns the Display
  string
- You can also use `value.to_string()` for
  Display types

</details>
