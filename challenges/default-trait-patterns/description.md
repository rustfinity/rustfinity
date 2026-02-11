The `Default` trait in Rust provides a way to create a default value for a type. This is incredibly useful when you need sensible starting values, especially in configurations, builders, or when using struct update syntax.

The `Default` trait is defined in the standard library as:

```rust
pub trait Default {
    fn default() -> Self;
}
```

Many types in the standard library implement `Default`: numbers default to zero, booleans to `false`, `String` to an empty string, `Vec` to an empty vector, and `Option` to `None`. You can derive `Default` for your own types if all fields implement `Default`, or implement it manually for custom behavior.

A common pattern is using `Default` with struct update syntax (`..Default::default()`) to specify only the fields you care about while letting the rest take their default values. This is particularly powerful in configuration structs and builder patterns.

## Your Task

Implement the following types and functions demonstrating the `Default` trait:

1. **`AppConfig`** - A configuration struct with `theme` (String), `dark_mode` (bool), `font_size` (u32), and `max_connections` (usize). Derive `Default` where theme is `"light"`, dark_mode is `false`, font_size is `14`, and max_connections is `100`.

2. **`Counter`** - A simple counter struct with a `count` field (i32). Implement `Default` manually to start at `0`.

3. **`BoundedValue<T>`** - A generic struct with `value: T`, `min: T`, and `max: T`. Implement `Default` where `T: Default + Clone`, setting all three fields to `T::default()`.

4. **`Status`** - An enum with variants `Pending`, `Active`, `Completed`, and `Failed`. Implement `Default` to return `Status::Pending`.

5. **`create_with_defaults<T: Default>()`** - A generic function that creates and returns `T::default()`.

6. **`merge_with_defaults(partial: PartialConfig) -> AppConfig`** - Takes a `PartialConfig` (with all `Option` fields) and returns an `AppConfig`, using defaults for any `None` values.

7. **`default_vec<T: Default>(count: usize) -> Vec<T>`** - Creates a `Vec` containing `count` elements, each initialized to `T::default()`.

## Example

```rust
use default_trait_patterns::*;

// Using derived Default
let config = AppConfig::default();
assert_eq!(config.theme, "light");
assert_eq!(config.dark_mode, false);
assert_eq!(config.font_size, 14);

// Struct update syntax with Default
let custom = AppConfig {
    dark_mode: true,
    ..Default::default()
};
assert!(custom.dark_mode);
assert_eq!(custom.theme, "light"); // Still default

// Manual Default implementation
let counter = Counter::default();
assert_eq!(counter.count, 0);

// Generic with Default bounds
let bounded: BoundedValue<i32> = BoundedValue::default();
assert_eq!(bounded.value, 0);

// Enum default
let status = Status::default();
assert!(matches!(status, Status::Pending));

// Generic function
let s: String = create_with_defaults();
assert_eq!(s, "");

// Merging with defaults
let partial = PartialConfig {
    theme: Some("dark".to_string()),
    dark_mode: None,
    font_size: Some(16),
    max_connections: None,
};
let merged = merge_with_defaults(partial);
assert_eq!(merged.theme, "dark");
assert!(!merged.dark_mode); // Used default
assert_eq!(merged.font_size, 16);
assert_eq!(merged.max_connections, 100); // Used default

// Creating default-initialized vectors
let zeros: Vec<i32> = default_vec(5);
assert_eq!(zeros, vec![0, 0, 0, 0, 0]);
```

## Hints

<details>
  <summary>Click here to reveal hints</summary>

- To derive `Default` with custom values, you'll need to implement the trait manually rather than using `#[derive(Default)]`
- For `AppConfig`, implement `Default` manually to set specific default values
- The `#[derive(Default)]` macro works when all fields implement `Default` and you want their default values
- Use `Option::unwrap_or_else(|| default_value)` for efficient default fallbacks in `merge_with_defaults`
- For `default_vec`, you can use `std::iter::repeat_with` or a simple loop
- Remember that `Clone` is needed if you want to use the same default value multiple times in `BoundedValue`

</details>
