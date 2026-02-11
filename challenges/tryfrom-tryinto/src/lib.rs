/// A wrapper around `i32` that only accepts positive numbers (greater than 0).
///
/// # Examples
///
/// ```
/// use tryfrom_tryinto::PositiveNumber;
///
/// let pos = PositiveNumber::try_from(42);
/// assert!(pos.is_ok());
/// assert_eq!(pos.unwrap().0, 42);
///
/// let zero = PositiveNumber::try_from(0);
/// assert!(zero.is_err());
///
/// let neg = PositiveNumber::try_from(-5);
/// assert!(neg.is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositiveNumber(pub i32);

impl TryFrom<i32> for PositiveNumber {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(PositiveNumber(value))
        } else {
            Err("number must be positive")
        }
    }
}

/// A wrapper around `u8` that only accepts values from 0 to 100 (inclusive).
///
/// # Examples
///
/// ```
/// use tryfrom_tryinto::Percentage;
///
/// let valid = Percentage::try_from(75);
/// assert!(valid.is_ok());
/// assert_eq!(valid.unwrap().0, 75);
///
/// let zero = Percentage::try_from(0);
/// assert!(zero.is_ok());
///
/// let hundred = Percentage::try_from(100);
/// assert!(hundred.is_ok());
///
/// let over = Percentage::try_from(150);
/// assert!(over.is_err());
///
/// let negative = Percentage::try_from(-10);
/// assert!(negative.is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Percentage(pub u8);

impl TryFrom<i32> for Percentage {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if (0..=100).contains(&value) {
            Ok(Percentage(value as u8))
        } else {
            Err("percentage must be between 0 and 100")
        }
    }
}

/// A wrapper around `String` that ensures the string is not empty.
///
/// # Examples
///
/// ```
/// use tryfrom_tryinto::NonEmptyString;
///
/// let s = NonEmptyString::try_from("hello");
/// assert!(s.is_ok());
/// assert_eq!(s.unwrap().0, "hello");
///
/// let empty = NonEmptyString::try_from("");
/// assert!(empty.is_err());
///
/// let owned = NonEmptyString::try_from(String::from("world"));
/// assert!(owned.is_ok());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonEmptyString(pub String);

impl TryFrom<String> for NonEmptyString {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("string cannot be empty")
        } else {
            Ok(NonEmptyString(value))
        }
    }
}

impl TryFrom<&str> for NonEmptyString {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("string cannot be empty")
        } else {
            Ok(NonEmptyString(value.to_string()))
        }
    }
}

/// A wrapper around `i32` that only accepts even numbers.
///
/// # Examples
///
/// ```
/// use tryfrom_tryinto::EvenNumber;
///
/// let even = EvenNumber::try_from(4);
/// assert!(even.is_ok());
/// assert_eq!(even.unwrap().0, 4);
///
/// let zero = EvenNumber::try_from(0);
/// assert!(zero.is_ok());
///
/// let negative_even = EvenNumber::try_from(-6);
/// assert!(negative_even.is_ok());
///
/// let odd = EvenNumber::try_from(3);
/// assert!(odd.is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvenNumber(pub i32);

impl TryFrom<i32> for EvenNumber {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err("number must be even")
        }
    }
}

/// A wrapper around `char` that only accepts ASCII characters.
///
/// # Examples
///
/// ```
/// use tryfrom_tryinto::AsciiChar;
///
/// let ascii = AsciiChar::try_from('A');
/// assert!(ascii.is_ok());
/// assert_eq!(ascii.unwrap().0, 'A');
///
/// let from_byte = AsciiChar::try_from(65u8);
/// assert!(from_byte.is_ok());
/// assert_eq!(from_byte.unwrap().0, 'A');
///
/// let non_ascii = AsciiChar::try_from('ñ');
/// assert!(non_ascii.is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsciiChar(pub char);

impl TryFrom<char> for AsciiChar {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_ascii() {
            Ok(AsciiChar(value))
        } else {
            Err("character must be ASCII")
        }
    }
}

impl TryFrom<u8> for AsciiChar {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let c = char::from(value);
        if c.is_ascii() {
            Ok(AsciiChar(c))
        } else {
            Err("character must be ASCII")
        }
    }
}
