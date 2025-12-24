Generics in Rust allow you to write code that works with different types without duplicating code for each specific type. They are commonly used in functions, structs, enums, and methods to make your code more reusable and flexible.

For this challenge, you will implement a generic struct `ItemContainer` that can store any type of item and provide methods to retrieve and manipulate the item.

## Your Task

1. Define a generic struct `ItemContainer<T>` with a single field `item: T`.
2. Implement a method `get_item` that returns a reference to the item.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
  <summary>Click here to reveal hints</summary>

- Define the struct as `ItemContainer<T>`. e.g.

  ```rust
  struct ItemContainer<T> {
    item: T,
  }
  ```

- Implement the method in the `impl<T>` block. The method should take `&self` and return `&T`.

</details>
