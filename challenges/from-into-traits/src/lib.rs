/// A temperature in Celsius.
///
/// # Examples
///
/// ```
/// use from_into_traits::{Celsius, Fahrenheit};
///
/// let c = Celsius(100.0);
/// let f: Fahrenheit = c.into();
/// assert!((f.0 - 212.0).abs() < 0.001);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Celsius(pub f64);

/// A temperature in Fahrenheit.
///
/// # Examples
///
/// ```
/// use from_into_traits::{Celsius, Fahrenheit};
///
/// let f = Fahrenheit(32.0);
/// let c: Celsius = f.into();
/// assert!((c.0 - 0.0).abs() < 0.001);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fahrenheit(pub f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

/// An RGB color with red, green, and blue components.
///
/// # Examples
///
/// ```
/// use from_into_traits::{Rgb, HexColor};
///
/// let rgb = Rgb { r: 255, g: 87, b: 51 };
/// let hex: HexColor = rgb.into();
/// assert_eq!(hex.0, "#FF5733");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// A hex color string representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexColor(pub String);

impl From<Rgb> for HexColor {
    fn from(rgb: Rgb) -> Self {
        HexColor(format!("#{:02X}{:02X}{:02X}", rgb.r, rgb.g, rgb.b))
    }
}

/// An email address wrapper.
///
/// # Examples
///
/// ```
/// use from_into_traits::Email;
///
/// let email: Email = "user@example.com".into();
/// assert_eq!(email.0, "user@example.com");
///
/// let owned = String::from("test@test.com");
/// let email: Email = owned.into();
/// assert_eq!(email.0, "test@test.com");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(pub String);

impl From<&str> for Email {
    fn from(s: &str) -> Self {
        Email(s.to_string())
    }
}

impl From<String> for Email {
    fn from(s: String) -> Self {
        Email(s)
    }
}

/// A point in 2D space.
///
/// # Examples
///
/// ```
/// use from_into_traits::{Point2D, Point3D};
///
/// let p2d = Point2D { x: 1.0, y: 2.0 };
/// let p3d: Point3D = p2d.into();
/// assert_eq!(p3d.x, 1.0);
/// assert_eq!(p3d.y, 2.0);
/// assert_eq!(p3d.z, 0.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

/// A point in 3D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<Point2D> for Point3D {
    fn from(p: Point2D) -> Self {
        Point3D {
            x: p.x,
            y: p.y,
            z: 0.0,
        }
    }
}

/// A generic wrapper that can hold any value.
///
/// # Examples
///
/// ```
/// use from_into_traits::Wrapper;
///
/// let wrapped: Wrapper<i32> = 42.into();
/// assert_eq!(wrapped.into_inner(), 42);
///
/// let wrapped: Wrapper<String> = String::from("hello").into();
/// assert_eq!(wrapped.into_inner(), "hello");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wrapper<T>(pub T);

impl<T> Wrapper<T> {
    /// Unwraps the value, consuming the wrapper.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for Wrapper<T> {
    fn from(value: T) -> Self {
        Wrapper(value)
    }
}
