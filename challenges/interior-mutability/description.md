Rust provides powerful abstractions for managing memory safely and efficiently. One such abstraction is `Rc` (Reference Counting), which allows multiple ownership of data. When combined with `RefCell`, it unlocks the ability to mutate data through shared ownership safely at runtime.

In this challenge, you'll use `Rc` and `RefCell` together to explore the concept of **Interior Mutability** in Rust.

## Your Task

Implement the following functions to demonstrate shared ownership and interior mutability:

1. `push`:
   - Accept an `Rc<RefCell<Vec<T>>>` as input.
   - Append an element to the shared vector inside the `RefCell`.

2. `iterate_and_print_shared_data`:
   - Take an `Rc<RefCell<Vec<T>>>` as input.
   - Iterate through the vector and print each element.

3. `plus_one`:
   - Accept an `Rc<RefCell<i32>` as input.
   - Increment the value inside the `RefCell` by one.

### Requirements

- Use `Rc` to share ownership of the vector.
- Use `RefCell` to allow interior mutability.
- Avoid using unsafe code.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
  <summary>Click here to reveal hints</summary>

- Use `Rc::clone` to share ownership of the vector.
- Use `RefCell`'s `.borrow_mut()` for mutable access and `.borrow()` for immutable access.
- `T: Display` is needed to format the output with `println!`.

</details>
