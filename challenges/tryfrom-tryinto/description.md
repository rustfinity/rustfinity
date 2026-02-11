# TryFrom and TryInto: Fallible Conversions

In the previous challenge, you learned about `From` and `Into` traits for infallible conversions. But what happens when a conversion might fail? For example, converting a large `i64` to a `u8` could overflow, or parsing a string into a number might fail if the string contains invalid characters.

Rust provides `TryFrom` and `TryInto` traits in the standard library for **fallible conversions** - conversions that might not succeed. These traits return a `Result` type, allowing you to handle conversion failures gracefully instead of panicking.

```rust
pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```

Just like `From` and `Into`, implementing `TryFrom` automatically gives you `TryInto` for free (due to a blanket implementation in the standard library).

## Your Task

Implement the following types and their `TryFrom` implementations:

**1. `PositiveNumber`**

A wrapper around `i32` that only accepts positive numbers (greater than 0).

- Implement `TryFrom<i32>` for `PositiveNumber`
- Return `Err("number must be positive")` if the value is 0 or negative
- The inner value should be accessible via the `.0` field

**2. `Percentage`**

A wrapper around `u8` that only accepts values from 0 to 100 (inclusive).

- Implement `TryFrom<i32>` for `Percentage`
- Return `Err("percentage must be between 0 and 100")` if out of range
- The inner value should be accessible via the `.0` field

**3. `NonEmptyString`**

A wrapper around `String` that ensures the string is not empty.

- Implement `TryFrom<String>` for `NonEmptyString`
- Implement `TryFrom<&str>` for `NonEmptyString`
- Return `Err("string cannot be empty")` if the string is empty
- The inner value should be accessible via the `.0` field

**4. `EvenNumber`**

A wrapper around `i32` that only accepts even numbers.

- Implement `TryFrom<i32>` for `EvenNumber`
- Return `Err("number must be even")` if the number is odd
- The inner value should be accessible via the `.0` field

**5. `AsciiChar`**

A wrapper around `char` that only accepts ASCII characters.

- Implement `TryFrom<char>` for `AsciiChar`
- Implement `TryFrom<u8>` for `AsciiChar` (convert byte to char, then validate)
- Return `Err("character must be ASCII")` if the character is not ASCII
- The inner value should be accessible via the `.0` field

## Examples

```rust
use tryfrom_tryinto::*;

// PositiveNumber
let pos: Result<PositiveNumber, _> = 42.try_into();
assert!(pos.is_ok());
assert_eq!(pos.unwrap().0, 42);

let neg: Result<PositiveNumber, _> = (-5).try_into();
assert!(neg.is_err());

// Percentage
let valid = Percentage::try_from(75);
assert!(valid.is_ok());

let invalid = Percentage::try_from(150);
assert!(invalid.is_err());

// NonEmptyString
let s = NonEmptyString::try_from("hello");
assert!(s.is_ok());

let empty = NonEmptyString::try_from("");
assert!(empty.is_err());

// EvenNumber
let even = EvenNumber::try_from(4);
assert!(even.is_ok());

let odd = EvenNumber::try_from(3);
assert!(odd.is_err());

// AsciiChar
let ascii = AsciiChar::try_from('A');
assert!(ascii.is_ok());

let non_ascii = AsciiChar::try_from('');
assert!(non_ascii.is_err());
```

## Hints

<details>
  <summary>Click here to reveal hints</summary>

- The `TryFrom` trait requires an associated `Error` type. For simple cases, `&'static str` works well.
- Remember that implementing `TryFrom<T>` for your type automatically provides `TryInto<YourType>` for `T`.
- Use the `?` operator or pattern matching to handle results elegantly.
- For `AsciiChar::try_from(u8)`, you can convert a `u8` to a `char` using `char::from(byte)` or `byte as char`.
- The `char::is_ascii()` method checks if a character is ASCII.
- Zero is neither positive nor negative, so `PositiveNumber::try_from(0)` should fail.
- Even numbers include negative even numbers and zero: -4, -2, 0, 2, 4, ...

</details>
