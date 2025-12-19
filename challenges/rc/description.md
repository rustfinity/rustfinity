Reference-counted pointers, `Rc<T>`, are a type of smart pointer in Rust that allow multiple ownership of the same data. This means you can have multiple references to a value, and the value will only be dropped when the last reference is out of scope. This is particularly useful in scenarios where you want to share data between multiple parts of your program without needing to copy it.

In this challenge, you'll use `Rc<T>` to share data between functions.

## Your Task

Implement the functions `use_shared_data` and `share_data_to_other_functions` to work with `Rc<T>`.

- `use_shared_data`:
  - Take an `Rc<Vec<T>>` as argument.
  - Loop over each item in the vector and print each element using `println!`.

- `share_data_to_other_functions`:
  - Share the input as a reference-counted pointer 3 times with the given closure.
  - Do cheap clones only and avoid deep copying the data.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
    <summary>Click here to reveal hints</summary>

- Use `Rc::new` to wrap your vector in an `Rc` smart pointer.
- Use `Rc::clone` to create new references to the shared data.
- For `use_shared_data`, make sure to use `T: Display` to allow printing elements with `{}` formatting.

</details>
