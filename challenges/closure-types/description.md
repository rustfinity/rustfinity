Closures in Rust are categorized based on how they interact with the environment and the variables they capture. The three main types are `Fn`, `FnMut`, and `FnOnce`. Understanding and using these types effectively is key to mastering closures in Rust.

## Closure Type Definitions

- **Fn**: A closure that borrows its environment immutably. It is useful for read-only operations.
- **FnMut**: A closure that mutably borrows its environment, allowing it to modify variables.
- **FnOnce**: A closure that takes ownership of its captured variables, consuming them in the process.

## Your Task

Implement the following closures and their respective behaviors:

- `calculate_total`: An `Fn` closure that calculates the total price of an item, including tax (`price + price * tax_rate`).
- `apply_discount`: An `FnMut` closure that mutates the cart total by subtracting a given discount.
- `checkout_cart`: An `Fn` closure that takes ownership of the cart's details (a `String`), prepends a confirmation message, and returns the result.

## Hints

<details>
<summary>Click here to reveal hints</summary>

- Use `impl Fn`, `impl FnMut`, or `impl FnOnce` to define the types of the closures. e.g.
  ```rust
    pub fn return_closure() -> impl Fn(f64, f64) -> f64 {
        //
    }
  ```

</details>
