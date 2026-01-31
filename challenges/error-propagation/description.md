Error propagation is a core concept in Rust that allows you to handle errors in a clean and structured way. Instead of having to handle each error manually on every step, you can easily use the `?` operator to propagate errors to a higher level so that they can be handled in a single place.

In this challenge, you’ll use `io::Error` to represent potential issues when working with file I/O. This approach leverages Rust’s standard library for concise and idiomatic error handling.

## Your Task

Your task is to implement a function that reads integers from a file, computes their sum, and gracefully propagates any errors using the `?` operator.

Implement the function `sum_integers_from_file`:

- Takes the file path as a parameter.
- Reads the file line by line, assuming each line contains
  a single integer or invalid data.
- Computes and returns the sum of all integers as a `Result<i32, io::Error>`.
- Handles the following:
  - If the file cannot be opened, propagate the `io::Error`.
  - If a line cannot be parsed as an integer, propagate
    a custom `io::Error` with a meaningful message.

### Requirements

1. Handle errors cleanly and propagate them using the `?` operator.
2. For invalid lines, create an `io::Error` with `io::ErrorKind::InvalidData`.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
<summary>Click here to reveal hints</summary>

- Use `std::fs::File::open` to open a file.
- Use `io::BufReader::new` to read lines from the file.
- Convert strings to integers with the `str::parse` method.
- The `io::Error::new` function can create custom errors.
- To create the error type we want for invalid numbers, you can do the following:
  ```rust
  let error = io::Error::new(io::ErrorKind::InvalidData, "Invalid number");
  ```
- To propagate an error, use the `?` operator. e.g.
  ```rust
  let file = File::open(file_path)?;
  ```
- You can transform an error type to another error type by using the `map_err` method. e.g.
  ```rust
  let num = num_str.parse::<i32>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid number"))?;
  ```

</details>
