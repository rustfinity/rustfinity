Rust makes it easy to parse strings into other types using the `.parse()` method and the `FromStr` trait. This is a fundamental skill for handling user input, reading configuration files, or processing text data.

## The `parse()` Method

The `.parse()` method attempts to convert a string slice into another type. It returns a `Result` because parsing can fail if the string doesn't represent a valid value of the target type:

```rust
let num: i32 = "42".parse().unwrap();
assert_eq!(num, 42);

// Or with turbofish syntax:
let num = "42".parse::<i32>().unwrap();
```

## The `FromStr` Trait

When you call `.parse()`, Rust uses the `FromStr` trait behind the scenes. You can implement this trait for your own types to make them parseable from strings:

```rust
use std::str::FromStr;

struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse "x,y" format
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err("Expected format: x,y".to_string());
        }
        let x = parts[0]
            .trim()
            .parse()
            .map_err(|_| "Invalid x")?;
        let y = parts[1]
            .trim()
            .parse()
            .map_err(|_| "Invalid y")?;
        Ok(Point { x, y })
    }
}
```

## Your Task

Implement the following:

1. `parse_int(s: &str) -> Result<i32, String>` - Parse a string into an i32, returning a descriptive error message on failure
2. `parse_bool(s: &str) -> Result<bool, String>` - Parse common boolean representations: "true", "false", "1", "0", "yes", "no" (case-insensitive)
3. `parse_key_value(s: &str) -> Result<(String, String), String>` - Parse a "key=value" string into a tuple
4. Implement `FromStr` for a `Color` struct that parses "r,g,b" format (e.g., "255,128,0")
5. `parse_list<T: FromStr>(s: &str, delimiter: char) -> Result<Vec<T>, String>` - Parse a delimited list of values into a Vec

## Examples

```rust
// parse_int: string to integer
assert_eq!(parse_int("42"), Ok(42));
assert_eq!(parse_int("-17"), Ok(-17));
assert!(parse_int("not a number").is_err());

// parse_bool: flexible boolean parsing
assert_eq!(parse_bool("true"), Ok(true));
assert_eq!(parse_bool("YES"), Ok(true));
assert_eq!(parse_bool("0"), Ok(false));
assert!(parse_bool("maybe").is_err());

// parse_key_value: extract key and value
assert_eq!(
    parse_key_value("name=Alice"),
    Ok(("name".to_string(), "Alice".to_string()))
);
assert_eq!(
    parse_key_value("count=42"),
    Ok(("count".to_string(), "42".to_string()))
);
assert!(parse_key_value("no_equals_sign").is_err());

// Color with FromStr
let color: Color = "255,128,0".parse().unwrap();
assert_eq!(color.r, 255);
assert_eq!(color.g, 128);
assert_eq!(color.b, 0);

// parse_list: generic list parsing
assert_eq!(
    parse_list::<i32>("1,2,3", ','),
    Ok(vec![1, 2, 3])
);
assert_eq!(
    parse_list::<f64>("1.5;2.5;3.5", ';'),
    Ok(vec![1.5, 2.5, 3.5])
);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- For `parse_int`, use `.parse::<i32>()` and `.map_err()` to convert the error
- For `parse_bool`, match on `.to_lowercase().as_str()` for case-insensitive comparison
- For `parse_key_value`, use `.splitn(2, '=')` to split at only the first `=`
- For the `Color` struct, implement `FromStr` with `type Err = String`
- For `parse_list`, use `.split()`, then `.map()` with `.parse()`, and handle the Results with `.collect::<Result<Vec<T>, _>>()`
- The `?` operator is your friend for propagating errors

</details>
