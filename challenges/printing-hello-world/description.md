It's a convention to start learning a new programming language by writing a program that prints `"Hello, World!"` to the console.

In Rust, we can do this using the `println!` macro. The `!` indicates that this is a **macro** rather than a **function**.

## What is a macro?

In Rust, a **macro** is something that generates Rust code for you. Macros run **at compile time**, which lets them do things functions can't, like accept a variable number of arguments or adjust how they expand based on their arguments.

You can recognize macros because they end with `!`, for example: `println!`, `vec!`, or `format!`.

> **Note:** Don't worry about how macros work internally yet — just know they exist and how to recognize them.

## println!

The `println!` macro is used to print text to the console. It is similar to the `println` function in other programming languages, but it is a **macro** in Rust.

Here's how to print `Hello` to the console using the `println!` macro:

```rust
fn main() {
    println!("Hello");
}
```

## Your task

Write a program that prints `"Hello, World!"` to the console using the `println!` macro.

## Expected Output

```rust
Hello, World!
```
