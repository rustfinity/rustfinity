Now that we know a bit about traits and generics, let's dive into **trait bounds**. Trait bounds allow you to specify that a generic type must implement a particular trait. This ensures that only types with certain capabilities can be used with your generic code.

For example:

- A generic function can be constrained to types that implement the `std::fmt::Display` trait for printing.
- A generic struct can be limited to types that implement traits like `PartialOrd` for comparison.

Trait bounds make your code flexible while maintaining strong type safety.

## Syntax

You can specify trait bounds in two ways:

1. **Inline**: Specify the trait bounds after the `impl` or `fn` keyword.

   ```rust
   fn my_function<T: Trait1 + Trait2>(value: T) {
       // code here
   }
   ```

2. **Where clause**: Use a `where` clause to specify trait bounds.

   ```rust
   fn my_function<T>(value: T)
   where
        T: Trait1 + Trait2,
   {
        // code here
   }
   ```

This means that the generic type `T` must implement both `Trait1` and `Trait2`.

## Your Task

1. Define a generic function `compare_and_display` that:
   - Takes two parameters of the same type.
   - Prints both parameters using the `Display` trait.
   - Returns the greater of the two using the `PartialOrd` trait.

2. Use trait bounds to ensure the function works only with types that implement both `Display` and `PartialOrd`.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
  <summary>Click here to reveal hints</summary>

- Use `std::cmp::PartialOrd` to compare values.
- Use `std::fmt::Display` to print values with the `println!` macro.
- You can write trait bounds inline with `T: Trait1 + Trait2` or using a `where` clause. e.g.
  ```rust
  fn my_function<T: Trait1 + Trait2>(value: T) {
      // code here
  }
  ```

</details>
