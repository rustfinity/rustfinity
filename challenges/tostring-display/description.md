The `Display` trait is Rust's standard way to format types for user-facing output. When you implement `Display`, you automatically get the `ToString` trait for free, allowing you to convert your type to a `String`. While the `Debug` trait is for developer-oriented output (often derived automatically), `Display` is meant for clean, human-readable formatting.

The `ToString` trait provides a single method `to_string()` that converts a value to a `String`. In practice, you almost never implement `ToString` directly—instead, you implement `Display`, and the standard library provides a blanket implementation of `ToString` for any type that implements `Display`.

## The Display and ToString Traits

```rust
use std::fmt;

pub trait Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

pub trait ToString {
    fn to_string(&self) -> String;
}
```

The `Display::fmt` method writes formatted output to a `Formatter`. You use the `write!` macro inside this method to write your formatted content.

## Standard Library Examples

```rust
use std::fmt;

// Built-in types implement Display
let n = 42;
println!("{}", n);  // Uses Display
let s = n.to_string();  // "42"

// Strings implement Display trivially
let greeting = "Hello";
println!("{}", greeting);  // "Hello"
```

## Your Task

Implement the `Display` trait and related formatting for the following types:

### 1. Point

Create a `Point` struct with `x` and `y` fields (both `i32`). Implement `Display` to format it as `(x, y)`.

### 2. Color

Create a `Color` enum with variants `Red`, `Green`, `Blue`, and `Custom(u8, u8, u8)`. Implement `Display` to show:

- Named colors as their name (e.g., "Red", "Green", "Blue")
- Custom colors as "RGB(r, g, b)"

### 3. Temperature

Create a `Temperature` enum with variants `Celsius(f64)` and `Fahrenheit(f64)`. Implement `Display` to format as:

- Celsius: "X°C" (e.g., "25.5°C")
- Fahrenheit: "X°F" (e.g., "77.0°F")

### 4. Money

Create a `Money` struct with `amount` (i64, representing
cents) and `currency` (String) fields. Implement `Display`
to format as "$X.XX" for USD, "€X.XX" for EUR, or
"X.XX CURRENCY" for others. The amount should always show
exactly 2 decimal places.

### 5. Person

Create a `Person` struct with `name` (String) and `age` (u32) fields. Implement `Display` to format as "Name (age X)".

### 6. list_to_string Function

Write a generic function
`list_to_string<T: Display>(items: &[T]) -> String`
that formats a slice of displayable items as a
comma-separated list in square brackets,
e.g., "[1, 2, 3]".

### 7. format_table Function

Write a function
`format_table(headers: &[&str], rows: &[Vec<String>])
-> String` that formats data as a simple text table with
headers and rows, each cell separated by " | ".

## Examples

```rust
// Point
let point = Point { x: 3, y: -4 };
assert_eq!(point.to_string(), "(3, -4)");
assert_eq!(format!("{}", point), "(3, -4)");

// Color
let red = Color::Red;
assert_eq!(red.to_string(), "Red");
let custom = Color::Custom(255, 128, 0);
assert_eq!(custom.to_string(), "RGB(255, 128, 0)");

// Temperature
let celsius = Temperature::Celsius(25.5);
assert_eq!(celsius.to_string(), "25.5°C");
let fahrenheit = Temperature::Fahrenheit(77.0);
assert_eq!(fahrenheit.to_string(), "77°F");

// Money
let usd = Money {
    amount: 1234,
    currency: "USD".to_string()
};
assert_eq!(usd.to_string(), "$12.34");
let eur = Money {
    amount: 5000,
    currency: "EUR".to_string()
};
assert_eq!(eur.to_string(), "€50.00");
let other = Money {
    amount: 999,
    currency: "GBP".to_string()
};
assert_eq!(other.to_string(), "9.99 GBP");

// Person
let person = Person {
    name: "Alice".to_string(),
    age: 30
};
assert_eq!(person.to_string(), "Alice (age 30)");

// list_to_string
let numbers = vec![1, 2, 3];
assert_eq!(list_to_string(&numbers), "[1, 2, 3]");
let empty: Vec<i32> = vec![];
assert_eq!(list_to_string(&empty), "[]");

// format_table
let headers = vec!["Name", "Age"];
let rows = vec![
    vec!["Alice".to_string(), "30".to_string()],
    vec!["Bob".to_string(), "25".to_string()],
];
let table = format_table(&headers, &rows);
assert!(table.contains("Name | Age"));
assert!(table.contains("Alice | 30"));
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `write!(f, "...")` inside the `fmt` method to write
  formatted output
- The `Formatter` passed to `fmt` implements
  `std::fmt::Write`, so `write!` works with it
- For the degree symbol, you can use the Unicode
  character `°` (or `\u{00B0}`)
- To format floats with specific precision, use `{:.1}`
  for 1 decimal place
- Remember that `to_string()` is automatically available
  once you implement `Display`
- For Money, convert cents to dollars by dividing:
  `amount / 100` for whole part, `amount % 100` for cents
- Use `format!` to build intermediate strings when needed
- The `join` method on iterators is useful for creating
  comma-separated lists

</details>
