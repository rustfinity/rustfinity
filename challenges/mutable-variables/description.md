Declaring and manipulating variables in programming is a fundamental concept that allows you to store and modify data. Variables in Rust are **immutable by default**, but you can make them mutable using the `mut` keyword.

In this challenge, you will declare and use **mutable variables in Rust**. You will be given a function where you need to declare variables, modify their values, and perform specific operations.

## Your task

- Declare variable `text` with an initial value of type `String` Use `let mut` to make it mutable.
- Re assign the variable `text` to something else of your choice.
- Call the `mutates_value(&mut text)` function, which will change the value of your `text` variable.
- Return the final value of the variable.

## Hints

If you're stuck, feel free to check out the hints below

<details>
    <summary>Click here to see the hints</summary>

- Use the `let mut` keyword to declare a mutable variable.
- To give the value to as a mutable reference use the `&mut` keyword. e.g.

  ```rust
  let mut text = "hello";
  mutates_value(&mut text);
  ```

- To create a `String` from a `&str`, use `String::from(value)` or `value.to_string()`. e.g.

  ```rust
  let text = "hello";
  let text = text.to_string();
  ```

- To return a value from a function, either use the `return` statement or let the last expression be the return value. For example:

  ```rust
  // Using return statement
  fn example1() -> String {
      return String::from("hello");
  }

  // Using expression (no semicolon)
  fn example2() -> String {
      String::from("hello")
  }
  ```

</details>
