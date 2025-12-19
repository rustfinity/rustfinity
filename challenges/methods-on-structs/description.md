In Rust, methods and associated functions are both defined using `impl` blocks. While they are similar in some ways, they serve different purposes:

1. **Associated Functions**:
   - Defined without a `self` parameter.
   - Typically used as constructors or utility functions.
   - Called using the struct's name, e.g., `StructName::function_name()`.

2. **Methods**:
   - Defined with a `self` parameter (`&self`, `&mut self`, or `self`).
   - Operate on an instance of the struct.
   - Called on a struct instance using the dot syntax, e.g., `instance.method_name()`.

In the previous challenge, the `Logger::log_message` was an **associated function** because it didn't take `self` as a parameter. In this challenge, you will learn how to create **methods** that operate on struct instances.

## Your Task

Define a struct `Counter` that represents a simple counter. Implement methods on this struct to increment, decrement, and get the current count. This will help you understand how to define and use methods that operate on a struct instance.

### Requirements

1. Define a struct `Counter` with a single field `count` of type `i32`.
2. Define a `new` associated function that acts as a constructor and initializes the `count` field to 0.
3. Implement the following methods for `Counter`:
   - `increment`: Increments the counter by 1.
   - `decrement`: Decrements the counter by 1.
   - `get_count`: Returns the current count.

4. Ensure these methods use the correct `self` parameter type (`&self` or `&mut self`) based on their behavior.
5. Make the `count` field private and provide a public constructor `Counter::new` that initializes the count to 0.

## Example Test

```rust
let mut counter = Counter::new();
counter.increment();
counter.increment();
assert_eq!(counter.get_count(), 2);

counter.decrement();
assert_eq!(counter.get_count(), 1);
```

## Hints

If you're stuck, you can check out these hints:

<details>
    <summary>Click here to reveal hints</summary>

- Fort he `increment` and `decrement` methods, use `&mut self` as the parameter type.
- Use `self.count += 1` to modify the counter in the `increment` method.
- Use `self.count -= 1` to modify the counter in the `decrement` method.
- For `get_count`, use `&self` and return the value of `self.count`.

</details>
