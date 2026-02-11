/// A temperature in Celsius.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Celsius(pub f64);

/// A temperature in Fahrenheit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fahrenheit(pub f64);

// TODO: Implement From<Celsius> for Fahrenheit

// TODO: Implement From<Fahrenheit> for Celsius

/// An RGB color with red, green, and blue components.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// A hex color string representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexColor(pub String);

// TODO: Implement From<Rgb> for HexColor

/// An email address wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(pub String);

// TODO: Implement From<&str> for Email

// TODO: Implement From<String> for Email

/// A point in 2D space.
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

// TODO: Implement From<Point2D> for Point3D

/// A generic wrapper that can hold any value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wrapper<T>(pub T);

impl<T> Wrapper<T> {
    /// Unwraps the value, consuming the wrapper.
    pub fn into_inner(self) -> T {
        // TODO: Return the inner value
        unimplemented!()
    }
}

// TODO: Implement From<T> for Wrapper<T>

pub fn main() {
    // Temperature conversion example
    let celsius = Celsius(100.0);
    let fahrenheit: Fahrenheit = celsius.into();
    println!("100°C = {}°F", fahrenheit.0);

    let fahrenheit = Fahrenheit(32.0);
    let celsius: Celsius = fahrenheit.into();
    println!("32°F = {}°C", celsius.0);

    // RGB to Hex example
    let rgb = Rgb { r: 255, g: 87, b: 51 };
    let hex: HexColor = rgb.into();
    println!("RGB({}, {}, {}) = {}", rgb.r, rgb.g, rgb.b, hex.0);

    // Email example
    let email: Email = "user@example.com".into();
    println!("Email: {}", email.0);

    // Point conversion example
    let p2d = Point2D { x: 3.0, y: 4.0 };
    let p3d: Point3D = p2d.into();
    println!("2D({}, {}) -> 3D({}, {}, {})", p2d.x, p2d.y, p3d.x, p3d.y, p3d.z);

    // Wrapper example
    let wrapped: Wrapper<i32> = 42.into();
    println!("Wrapped value: {}", wrapped.into_inner());
}
