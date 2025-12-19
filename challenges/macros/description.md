Macros in Rust are one of its most powerful features, enabling meta-programming and reducing repetitive code. A macro allows you to generate and include code fragments during compilation, making it an excellent tool for tasks like boilerplate elimination.

In this challenge, you'll create a macro that formats mathematical operations as human-readable strings. The macro should perform the calculation and present it in a clear, formatted way.

## Your Task

In this challenge, you will implement a macro named `math_operations!` that:

1. Takes three arguments: two integers and an operator
2. Performs the mathematical operation
3. Returns a formatted string showing the operation and its result

### Requirements

1. The macro should format the output as: `"{number} {operator} {number} = {result}"`

2. Input Types:
   - The macro expects integer operands only (no floating-point numbers)
   - Both signed and unsigned integers are supported
   - Results will also be **integers** (division rounds towards zero)

3. Supported Operations:
   - Addition (`+`)
   - Subtraction (`-`)
   - Multiplication (`*`)
   - Division (`/`)

4. Error Handling:
   - Division by zero should panic with message: `"Division by zero"`
   - Invalid operators should panic with message: `"Unsupported operator: {operator}"`

## Hints

If you're stuck, check out the hints below.

<details>
  <summary>Click here to reveal hints</summary>

- Use pattern matching to handle different operators
- Remember to check for division by zero before performing division
- The `format!` macro is useful for creating the output string
- Use `expr` matchers in your macro for maximum flexibility, e.g.

  ```rust
  macro_rules! math_operations {
    ($a:expr, $op:expr, $b:expr) => {{
        // Your code here
    }};
  }
  ```

</details>
