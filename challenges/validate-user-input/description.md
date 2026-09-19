In many programs, validating user input is a common and crucial task. Ensuring that input data adheres to expected formats and constraints can prevent bugs, **enhance security**, and improve **user experience**. In this challenge, you will create a function that validates a user's age and email, returning early if any conditions are not met.

## Your task

Your task is to implement a function `validate_user(age: i32, email: &str) -> Result<(), String>` that validates the user's age and email. The function should follow these rules:

1. If the age is **less than `0` or greater than `120`**, return an error with the message `"Invalid age"`.
2. If the email does not contain an `'@'` symbol, return an error with the message `"Invalid email"`.
3. If both the age and email are valid, return `Ok(())`.

Here's an example of how to use the **early return** technique:

```rust {3,7}
fn validate(name: &str, age: i32) -> Result<(), String> {
    if name.is_empty() {
        return Err("Name is empty".to_string());
    }

    if age < 0 {
        return Err("Invalid age".to_string());
    }

    Ok(())
}
```

## Example

```rust
let result = validate_user(25, "user@example.com");
assert_eq!(result, Ok(()));

let result = validate_user(-1, "user@example.com");
assert_eq!(result, Err("Invalid age".to_string()));

let result = validate_user(25, "userexample.com");
assert_eq!(result, Err("Invalid email".to_string()));
```

## Hints

- Use the `return` keyword to exit the function early when an invalid condition is encountered.
- Consider using the `contains` method to check if the email contains an '@' symbol.
