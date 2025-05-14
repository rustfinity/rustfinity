In Rust, **constants** are values that are bound to a name and are **not allowed to change**. Constants are always immutable and are declared using the `const` keyword. They must be annotated with their type, and they can be set only to a constant expression.

##An example of type annotation
const MAX_USERS: u32 = 80;

In this challenge, you will define and use a constant in Rust. This will help you understand how to declare constants and how they can be utilized in your programs.

## Your task

- Define a constant named `MAX_SIZE` with a value of `100`.
- Ensure that `MAX_SIZE` has the type `i32`.
- Return the value of `MAX_SIZE` from the `main` function.
- Make sure you make the constant **public** using the `pub` keyword (important for the tests to pass).
- Remember that constants can only be defined at the global scope. You can not define them inside a function.

## Hints

- Use the `const` keyword to define the constant.
- Ensure you **specify the type** of the constant explicitly.
- For this task, write the constant outside the main function to ensure the tests pass.
