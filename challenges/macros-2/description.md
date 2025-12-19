Now that we have an overview of how macros work, let's dive into another macro challenge. When implementing traits for multiple types, you often end up writing very similar code over and over.

In this challenge, you'll create a macro that generates implementations of a trait for multiple types. The goal is to reduce the amount of boilerplate code you need to write.

## Your Task

1. Write a macro named `config_default_impl` that:
   - Accepts a type and a default value as arguments.
   - Generates the implementation of `ConfigDefault` for that type.

2. Use the macro to implement the `ConfigDefault` trait for the following types:
   - `ConnectionTimeout` with a value of `30`.
   - `MaxConnections` with a value of `100`.
   - `RetryAttempts` with a value of `3`.
   - `PostgresPort` with a value of `5432`.
   - `MySQLPort` with a value of `3306`.
   - `MongoPort` with a value of `27017`.
   - `RedisPort` with a value of `6379`.

## Hints

If you're stuck, feel free to check out the hints below:

<details>
    <summary>Click here to reveal hints</summary>

- The syntax for a macro that accepts a type and a value is as follows:

  ```rust
  macro_rules! config_default_impl {
    ($type:ty, $value:expr) => {
        // Implementation here
    };
  }
  ```

</details>
