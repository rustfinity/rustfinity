In Rust, a **unit struct** is a type of struct that doesn't have any fields. It is essentially a "marker" or "tag" type and is represented by an empty pair of curly braces (`struct Name;`).

Unit structs are particularly useful in scenarios where you need to define types without associated data. Here's an example:

```rust
pub struct Database;
```

## Your Task

In this challenge, you'll implement a unit struct named `Logger`. The struct will serve as a type marker that enables certain logging functionality via a method `log_message`. You'll also implement a `log_message` function that prints a message to the console using the unit struct `Logger` as a marker. The goal is to familiarize yourself with unit structs and their usage as markers.

### Requirements

1. Define a unit struct named `Logger`.
2. Implement a `Logger::log_message` method that:
   - Takes a `&str` message as input.
   - Prints the message to the console in the format: `[LOG]: {message}`.

3. Use the unit struct `Logger` to call this method.

## Example Test

```rust
Logger::log_message("This is a test log.");
// Output: [LOG]: This is a test log.
```

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- Define the unit struct as `pub struct Logger;`.
- Implement an associated function for the unit struct using the `impl Logger` block.
- Use the `println!` macro to format and print the message.
- Remember, unit structs do not require any fields and are defined with empty braces.

</details>
