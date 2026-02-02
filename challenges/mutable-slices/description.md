In this challenge, you'll explore how to work with **mutable slices** in Rust. A mutable slice, `&mut [T]`, allows you to modify the elements of a collection directly and efficiently without creating a new collection.

Slices are a fundamental part of Rust, providing a view into contiguous sequences of elements such as arrays or vectors. When working with slices, you can iterate over and manipulate the elements dynamically.

## Your Task

Write a function `transform_even_odd(slice: &mut [i32])` that modifies the elements of a mutable slice of integers in the following way:

1. **Double each even number** in the slice.
2. **Subtract 1 from each odd number** in the slice.
3. Modify the elements directly without creating a new collection.

## Example

```rust
let mut numbers = [1, 2, 3, 4, 5];
transform_even_odd(&mut numbers);

// Odd numbers reduced by 1
// Even numbers doubled
assert_eq!(numbers, [0, 4, 2, 8, 4]);

let mut numbers = [10, 15, 20];
transform_even_odd(&mut numbers);

// 10 -> 20, 15 -> 14, 20 -> 40
assert_eq!(numbers, [20, 14, 40]);
```

## Hints

If you're stuck, here are some hints to help you:

<details>
    <summary>Additional Hints</summary>

- Use the `.iter_mut()` method to traverse the slice by mutable reference.
- Remember that you can check if a number is even using `num % 2 == 0`.
- Use dereferencing (`*`) to update the value pointed to by the mutable reference.
- You can use a `for` loop with mutable references for concise modification.
- The slice directly modifies the original collection it is derived from, so no need to return anything.

</details>
