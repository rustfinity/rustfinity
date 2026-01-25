Building strings efficiently is a common task in Rust. Whether you're generating reports, creating log messages, or formatting output for display, Rust provides several powerful tools for constructing strings.

## String Building Techniques

### The `format!` Macro

The `format!` macro is the most straightforward way to build formatted strings. It works like `println!` but returns a `String` instead of printing:

```rust
let name = "Alice";
let age = 30;
let greeting = format!("Hello, {}! You are {} years old.", name, age);
assert_eq!(greeting, "Hello, Alice! You are 30 years old.");
```

### The `write!` Macro

For building strings incrementally, `write!` is more efficient than repeated concatenation. It writes directly into a `String` (or any type implementing `std::fmt::Write`):

```rust
use std::fmt::Write;

let mut output = String::new();
write!(output, "Name: {}", "Bob").unwrap();
write!(output, ", Age: {}", 25).unwrap();
assert_eq!(output, "Name: Bob, Age: 25");
```

### The `Display` Trait

Implementing `Display` for your types lets them work with `format!`, `write!`, and `println!`:

```rust
use std::fmt::{Display, Formatter, Result};

struct Point { x: i32, y: i32 }

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

let p = Point { x: 3, y: 4 };
assert_eq!(format!("{}", p), "(3, 4)");
```

### String Concatenation

For simple cases, you can use `+` operator or `push_str`:

```rust
let mut s = String::from("Hello");
s.push_str(", World!");
assert_eq!(s, "Hello, World!");

// Or using +
let s1 = String::from("Hello, ");
let s2 = "World!";
let s3 = s1 + s2;  // Note: s1 is moved here
assert_eq!(s3, "Hello, World!");
```

## Your Task

Implement the following:

1. `build_greeting(name: &str, age: u32) -> String` - Build a greeting message using `format!`
2. `build_list(items: &[&str]) -> String` - Build a numbered list (1. first\n2. second\n...) using `write!`
3. `Person` struct with `name: String` and `age: u32` - Implement `Display` to format as "Name (Age years old)"
4. `build_table(headers: &[&str], rows: &[Vec<String>]) -> String` - Build a simple text table with headers and data rows
5. `concat_with_separator(parts: &[&str], sep: &str) -> String` - Join strings with a separator (implement without using `.join()`)

## Examples

```rust
// build_greeting
assert_eq!(
    build_greeting("Alice", 30),
    "Hello, Alice! You are 30 years old."
);

// build_list
assert_eq!(
    build_list(&["apple", "banana", "cherry"]),
    "1. apple\n2. banana\n3. cherry"
);

// Person Display
let person = Person { name: "Bob".to_string(), age: 25 };
assert_eq!(format!("{}", person), "Bob (25 years old)");

// build_table
let headers = vec!["Name", "Age"];
let rows = vec![
    vec!["Alice".to_string(), "30".to_string()],
    vec!["Bob".to_string(), "25".to_string()],
];
let table = build_table(&headers, &rows);
// Returns:
// | Name  | Age |
// |-------|-----|
// | Alice | 30  |
// | Bob   | 25  |

// concat_with_separator
assert_eq!(
    concat_with_separator(&["a", "b", "c"], ", "),
    "a, b, c"
);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- For `build_greeting`, use `format!("{} ... {}", name, age)`
- For `build_list`, use `std::fmt::Write` and iterate with `.enumerate()`
- For `Person`'s `Display`, implement `fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`
- For `build_table`, calculate column widths first, then use `write!` with padding specifiers like `{:<width$}`
- For `concat_with_separator`, iterate through items and conditionally add the separator

</details>
