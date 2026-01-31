/// A wrapper around `i32` that only accepts positive numbers (greater than 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositiveNumber(pub i32);

// TODO: Implement TryFrom<i32> for PositiveNumber

/// A wrapper around `u8` that only accepts values from 0 to 100 (inclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Percentage(pub u8);

// TODO: Implement TryFrom<i32> for Percentage

/// A wrapper around `String` that ensures the string is not empty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonEmptyString(pub String);

// TODO: Implement TryFrom<String> for NonEmptyString

// TODO: Implement TryFrom<&str> for NonEmptyString

/// A wrapper around `i32` that only accepts even numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvenNumber(pub i32);

// TODO: Implement TryFrom<i32> for EvenNumber

/// A wrapper around `char` that only accepts ASCII characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsciiChar(pub char);

// TODO: Implement TryFrom<char> for AsciiChar

// TODO: Implement TryFrom<u8> for AsciiChar

// Example usage
pub fn main() {
    // PositiveNumber
    let pos: Result<PositiveNumber, _> = 42.try_into();
    println!("PositiveNumber from 42: {:?}", pos);

    let neg: Result<PositiveNumber, _> = (-5).try_into();
    println!("PositiveNumber from -5: {:?}", neg);

    // Percentage
    let valid = Percentage::try_from(75);
    println!("Percentage from 75: {:?}", valid);

    let invalid = Percentage::try_from(150);
    println!("Percentage from 150: {:?}", invalid);

    // NonEmptyString
    let s = NonEmptyString::try_from("hello");
    println!("NonEmptyString from 'hello': {:?}", s);

    let empty = NonEmptyString::try_from("");
    println!("NonEmptyString from '': {:?}", empty);

    // EvenNumber
    let even = EvenNumber::try_from(4);
    println!("EvenNumber from 4: {:?}", even);

    let odd = EvenNumber::try_from(3);
    println!("EvenNumber from 3: {:?}", odd);

    // AsciiChar
    let ascii = AsciiChar::try_from('A');
    println!("AsciiChar from 'A': {:?}", ascii);

    let non_ascii = AsciiChar::try_from('ñ');
    println!("AsciiChar from 'ñ': {:?}", non_ascii);

    let from_byte = AsciiChar::try_from(65u8);
    println!("AsciiChar from 65u8: {:?}", from_byte);
}
