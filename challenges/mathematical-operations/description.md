This challenge is about basic mathematical operations. You will be given 2 numbers `a` and `b`. You need to perform the following operations:

1. Sum of `a` and `b`
2. Difference of `a` and `b`
3. Multiplication of `a` and `b`
4. Division of `a` and `b` (Note: `b` must not be zero. The function assumes valid input.)

You need to return a tuple containing the results of the above operations in the same order. `(sum, difference, multiply, divide)`

> Note that every value in the tuple must be of type `i32`. The function assumes `b` is not zero.

### Example

```rust
let a = 10;
let b = 5;
let result = math_operations(a, b);
assert_eq!(result, (15, 5, 50, 2));
```

## Hints

In Rust you can use the following operators for the above operations:

1. Sum: `+`
2. Difference: `-`
3. Multiplication: `*`
4. Division: `/`

**Good luck!**
