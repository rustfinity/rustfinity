The `From` and `Into` traits are Rust's standard way to express type conversions. They provide a consistent interface for transforming one type into another, making your code more flexible and easier to read. When you implement `From`, you automatically get `Into` for free thanks to a blanket implementation in the standard library.

The `From` trait defines a `from()` method that takes ownership of a value and returns the new type. It's typically used when the conversion is infallible (cannot fail). For fallible conversions, Rust provides `TryFrom` and `TryInto` (covered in a separate challenge).

## The From and Into Traits

```rust
pub trait From<T> {
    fn from(value: T) -> Self;
}

pub trait Into<T> {
    fn into(self) -> T;
}
```

When you implement `From<T> for U`, you automatically get `Into<U> for T`. This means you should always implement `From` rather than `Into` directly.

## Standard Library Examples

```rust
// String implements From<&str>
let s: String = String::from("hello");
let s: String = "hello".into();

// Vec<u8> implements From<&str>
let bytes: Vec<u8> = Vec::from("hello");

// i64 implements From<i32>
let big: i64 = i64::from(42i32);
let big: i64 = 42i32.into();
```

## Your Task

Implement the `From` trait for the following type conversions:

### 1. Celsius to Fahrenheit

Create a `Celsius` struct and a `Fahrenheit` struct, both wrapping an `f64`. Implement conversions in both directions:

- `Fahrenheit::from(Celsius)` - Formula: F = C × 9/5 + 32
- `Celsius::from(Fahrenheit)` - Formula: C = (F - 32) × 5/9

### 2. RGB to Hex Color

Create an `Rgb` struct with `r`, `g`, `b` fields (all `u8`) and a `HexColor` struct wrapping a `String`. Implement:

- `HexColor::from(Rgb)` - Convert RGB to hex string like "#FF5733"

### 3. Email Address Validation

Create an `Email` struct wrapping a `String`. Implement:

- `Email::from(&str)` - Create an email from a string slice
- `Email::from(String)` - Create an email from an owned string

### 4. Point2D to Point3D

Create `Point2D` with `x` and `y` fields (`f64`) and `Point3D` with `x`, `y`, `z` fields (`f64`). Implement:

- `Point3D::from(Point2D)` - Convert 2D point to 3D with z=0.0

### 5. Generic Wrapper

Create a generic `Wrapper<T>` struct that wraps a value. Implement:

- `Wrapper<T>::from(T)` - Wrap any value
- Implement a method `into_inner(self) -> T` to unwrap the value

## Examples

```rust
// Temperature conversion
let celsius = Celsius(100.0);
let fahrenheit: Fahrenheit = celsius.into();
assert!((fahrenheit.0 - 212.0).abs() < 0.001);

let fahrenheit = Fahrenheit(32.0);
let celsius: Celsius = fahrenheit.into();
assert!((celsius.0 - 0.0).abs() < 0.001);

// RGB to Hex
let rgb = Rgb { r: 255, g: 87, b: 51 };
let hex: HexColor = rgb.into();
assert_eq!(hex.0, "#FF5733");

// Email
let email: Email = "user@example.com".into();
assert_eq!(email.0, "user@example.com");

// Point conversion
let p2d = Point2D { x: 1.0, y: 2.0 };
let p3d: Point3D = p2d.into();
assert_eq!(p3d.z, 0.0);

// Generic wrapper
let wrapped: Wrapper<i32> = 42.into();
assert_eq!(wrapped.into_inner(), 42);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Implement `From` rather than `Into` - you get `Into` automatically
- Use `format!("{:02X}{:02X}{:02X}", r, g, b)` for hex conversion (formats RGB values as 2-digit uppercase hex)
- For the generic `Wrapper<T>`, the `From` implementation should be generic over `T`
- Remember that `From` takes ownership of the input value
- The `.into()` method can require type annotations if Rust can't infer the target type
- You can use turbofish syntax `value.into::<TargetType>()` or let binding `let x: TargetType = value.into()`

</details>
