Pattern matching in Rust is a powerful feature that allows you to destructure and inspect complex data types in a concise and readable manner. By using match statements, you can handle different variants of enums, tuples, and other data structures, applying specific logic based on the shape and content of the data.

This challenge will help you practice advanced pattern matching techniques in a real-world scenario.

## Your Task

You need to implement a method `check_validity(&self)` for the `BookItem` enum that returns a `bool` indicating whether the item is valid based on the following rules:

1. **Book**:
   - The number of pages must be greater than 0.
   - If there is a discount:
     - Must be non-negative (≥ 0)
     - Must not exceed 50% (≤ 50)

2. **EBook**:
   - The title (a `String`) must be non-empty.
   - The second field in the tuple must be greater than 0.

3. **Collection**:
   - Must not be empty
   - **ALL** items in the collection must be valid
   - This means a collection containing even one invalid item is considered invalid

4. **OutOfPrint**:
   - This variant is always considered invalid.

### Requirements

- Implement `check_validity(&self)` using match guards to reflect these rules.
- Return a boolean (`true` or `false`) based on the type of `BookItem`.
- Ensure that the function does not panic for any valid input.

## Hints

<details>
  <summary>Click here to reveal hints</summary>

- Use `if` guards to combine multiple conditions: `if *pages > 0 && *d >= 0`
- Pattern match on `Option<i32>` using `Some(d)` and `None`
- For Collections:
  - Check `!items.is_empty()` first
  - Use `iter().all(|item| item.check_validity())` to ensure all items are valid
- Remember to dereference numbers when using them in guards: `*pages`, `*second`
- The catch-all `_ => false` arm handles any remaining invalid cases

</details>
