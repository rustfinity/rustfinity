Procedural macros in Rust allow developers to extend the language and automate code generation. They are often used to create custom `derive` attributes or other annotations that modify or add to Rust's functionality.

In this challenge, you'll implement a procedural macro to derive a custom trait called `Describe`. The `Describe` trait should provide a method, `describe`, that returns a string containing the name of the struct and its fields with their values. This macro should generate an implementation of the trait automatically when applied to a struct.

## Your Task

Your task is to create a procedural macro named `derive_describe` that can be used to generate the `Describe` trait implementation for any struct. The generated `describe` method should return a string representation of the struct, including its name and all its fields with their respective values.

For example, applying the macro to a struct like:

```rust
#[derive(Describe)]
struct Point {
    x: i32,
    y: i32,
}
```

Should generate a `Describe` implementation such that:

```rust
let p = Point { x: 1, y: 2 };
assert_eq!(p.describe(), "Point { x: 1, y: 2 }");
```

### Requirements

1. The `derive_describe` macro should:
   - Generate the `Describe` trait implementation for the struct it is applied to.
   - Support structs with named fields. (Tuple structs or unit structs are not required for this challenge.)

2. The `describe` method should return a properly formatted string, including the struct name and fields with their values.

## Good to know

Procedural macros must be defined in a separate crate from the one where they are used. This is because procedural macros are compiled to a shared library that is loaded by the Rust compiler at compile time. The shared library must be compiled before the crate that uses the macro.

In your `Cargo.toml`, you should define a new crate for the procedural macro:

```toml
[lib]
proc-macro = true
```

This is already done for this challenge, but it's important to understand the setup.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- Use the `syn` crate to parse the input `TokenStream` into an Abstract Syntax Tree (AST).
- Use the `quote` crate to generate Rust code as a `TokenStream`.

</details>
