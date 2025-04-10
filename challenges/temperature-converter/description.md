In many real-world applications, converting temperatures between different units is a common task. For example, meteorologists, scientists, and engineers often need to convert temperatures from Celsius to Fahrenheit or Kelvin. In this challenge, you'll implement a function that **converts temperatures between these units using logical operators and conditional statements**.

## Your task

Your task is to write a function `convert_temperature` that takes three parameters: a **float** representing the temperature value, a **string** representing the **unit** of the input temperature, and another **string** representing the desired unit for the output temperature. The function should return a `Result` type with either the converted temperature as a float or an error message if the input is invalid.

## Requirements

- The function should correctly convert temperatures between **Celsius**, **Fahrenheit**, and **Kelvin**.
- If the input unit or the desired output unit is not one of **"C", "F", or "K"**, the function should return an error message: `"Invalid unit"`.
- If the conversion is successful, the function should return the converted temperature as a **float** in the `Ok` variant of the `Result`.

## Did you know?

The Celsius and Fahrenheit scales are named after the scientists **Anders Celsius** and **Daniel Gabriel Fahrenheit**, respectively. The **Kelvin scale**, on the other hand, is named after **Lord Kelvin**, a British physicist. Unlike the **Celsius** and **Fahrenheit** scales, which are based on the freezing and boiling points of water, the **Kelvin** scale is an absolute temperature scale based on the concept of absolute zero, the lowest possible temperature where all molecular motion ceases.

## Example

```rust
let result = convert_temperature(100.0, "C", "F");
assert_eq!(result, Ok(212.0));

let result = convert_temperature(32.0, "F", "C");
assert_eq!(result, Ok(0.0));

let result = convert_temperature(0.0, "C", "K");
assert_eq!(result, Ok(273.15));

let result = convert_temperature(300.0, "K", "C");
assert_eq!(result, Ok(26.85));

let result = convert_temperature(100.0, "C", "X");
assert_eq!(result, Err("Invalid unit".to_string()));

let result = convert_temperature(37.5, "C", "C");
assert_eq!(result, Ok(37.5));
```

## Hints

- To convert Celsius to Fahrenheit: `F = C * (9/5) + 32`
- To convert Fahrenheit to Celsius: `C = (F - 32) * (5/9)`
- To convert Celsius to Kelvin: `K = C + 273.15`
- To convert Kelvin to Celsius: `C = K - 273.15`
- Remember to handle invalid units with proper error messages.
