Associated types in Rust allow you to define traits with a type placeholder that implementors of the trait can specify. This enables more flexible and type-safe abstractions.

For example, instead of passing types as generic parameters every time, you can use associated types to simplify the interface. Associated types are particularly useful when working with collections, iterators, or data processing pipelines.

In this challenge, you'll define a trait with an associated type and implement it for a struct. The goal is to create a structure for managing a simple Key-Value store where keys and values can have different types specified by the trait implementation.

You can solve the problem using either generics or associated types, but the challenge is made to make you familiar with associated types, so try to solve the challenge by using them instead of generics.

## Your Task

Define a trait `KeyValueStore` with an associated type `Key` and `Value`. Implement this trait for the struct `InMemoryStore`. The implementation should allow storing and retrieving key-value pairs of the specified types.

### Requirements

1. Define a trait `KeyValueStore` with:

   - An associated type `Key`.
   - An associated type `Value`.
   - Methods:
     - `set` to add a key-value pair.
     - `get` takes a reference of `&Key` and returns `Option<&Value>`.

2. Create a struct `InMemoryStore` that uses a `HashMap` to store key-value pairs. Implement the `KeyValueStore` trait for this struct.
3. Make sure all relevant values are public so that they can be accessed from outside the module (essential to pass the tests).

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
    <summary>Click here to reveal hints</summary>

- Define associated types in the `KeyValueStore` trait like this:

  ```rust
  pub trait KeyValueStore {
      type Key;
      type Value;

      fn set(&mut self, key: Self::Key, value: Self::Value);
      fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
  }
  ```

</details>
