In Rust, traits define shared behavior that types can implement. A trait acts as a contract, requiring implementing types to provide certain methods. This makes traits one of the most powerful features for achieving polymorphism and code reusability in Rust.

For example, the `Display` trait allows custom formatting for types using the `{}` formatter. Similarly, you can define your own traits to describe behaviors relevant to your program.

## Your Task

In this challenge, you'll implement a trait `Describable` and use it to define a common interface for objects that can provide a description. Your task is to:

1. Define a trait `Describable` with a single method `describe` that returns a `String`.
2. Implement this trait for the struct `Person`.
3. Implement this trait for the struct `Book`.

### Requirements

- The `Person` struct should have fields `name: String` and `age: u8`.
- The `Book` struct should have fields `title: String` and `author: String`.
- The `describe` method for `Person` should return a string
  like `"Person: Alice, Age: 30"`.
- The `describe` method for `Book` should return a string
  like `"Book: Rust Programming, Author: Jane Doe"`.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
  <summary>Click here to reveal hints</summary>

- Implement a trait for a struct using
  `impl TraitName for StructName`. e.g.
  ```rust
  impl Describable for Person {
      fn describe(&self) -> String {
          format!(
              "Person: {}, Age: {}",
              self.name,
              self.age
          )
      }
  }
  ```
- Don't forget to use the `&self` as the parameter for
  the `describe` method in the trait.

</details>
