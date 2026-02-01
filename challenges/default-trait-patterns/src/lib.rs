/// Application configuration with sensible defaults.
///
/// This struct demonstrates manual `Default` implementation with
/// custom default values for each field.
///
/// # Examples
///
/// ```
/// use default_trait_patterns::AppConfig;
///
/// let config = AppConfig::default();
/// assert_eq!(config.theme, "light");
/// assert_eq!(config.dark_mode, false);
/// assert_eq!(config.font_size, 14);
/// assert_eq!(config.max_connections, 100);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct AppConfig {
    pub theme: String,
    pub dark_mode: bool,
    pub font_size: u32,
    pub max_connections: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            theme: String::from("light"),
            dark_mode: false,
            font_size: 14,
            max_connections: 100,
        }
    }
}

/// A simple counter starting at zero.
///
/// Demonstrates manual `Default` implementation for a simple struct.
///
/// # Examples
///
/// ```
/// use default_trait_patterns::Counter;
///
/// let counter = Counter::default();
/// assert_eq!(counter.count, 0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
    pub count: i32,
}

impl Default for Counter {
    fn default() -> Self {
        Counter { count: 0 }
    }
}

impl Counter {
    /// Creates a new counter with the specified starting value.
    pub fn new(count: i32) -> Self {
        Counter { count }
    }

    /// Increments the counter by 1.
    pub fn increment(&mut self) {
        self.count += 1;
    }

    /// Decrements the counter by 1.
    pub fn decrement(&mut self) {
        self.count -= 1;
    }
}

/// A value constrained between minimum and maximum bounds.
///
/// Demonstrates implementing `Default` for a generic type where
/// the type parameter must also implement `Default` and `Clone`.
///
/// # Examples
///
/// ```
/// use default_trait_patterns::BoundedValue;
///
/// let bounded: BoundedValue<i32> = BoundedValue::default();
/// assert_eq!(bounded.value, 0);
/// assert_eq!(bounded.min, 0);
/// assert_eq!(bounded.max, 0);
///
/// let bounded_f64: BoundedValue<f64> = BoundedValue::default();
/// assert_eq!(bounded_f64.value, 0.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BoundedValue<T> {
    pub value: T,
    pub min: T,
    pub max: T,
}

impl<T: Default + Clone> Default for BoundedValue<T> {
    fn default() -> Self {
        let default_val = T::default();
        BoundedValue {
            value: default_val.clone(),
            min: default_val.clone(),
            max: default_val,
        }
    }
}

impl<T> BoundedValue<T> {
    /// Creates a new BoundedValue with specified value, min, and max.
    pub fn new(value: T, min: T, max: T) -> Self {
        BoundedValue { value, min, max }
    }
}

impl<T: Ord + Clone> BoundedValue<T> {
    /// Clamps the value to be within the min and max bounds.
    pub fn clamp(&mut self) {
        if self.value < self.min {
            self.value = self.min.clone();
        } else if self.value > self.max {
            self.value = self.max.clone();
        }
    }
}

/// Status of an operation.
///
/// Demonstrates implementing `Default` for an enum, where `Pending`
/// is the default variant.
///
/// # Examples
///
/// ```
/// use default_trait_patterns::Status;
///
/// let status = Status::default();
/// assert!(matches!(status, Status::Pending));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Pending,
    Active,
    Completed,
    Failed,
}

impl Default for Status {
    fn default() -> Self {
        Status::Pending
    }
}

/// Partial configuration with optional fields.
///
/// Used with `merge_with_defaults` to create a complete `AppConfig`
/// by filling in missing values with defaults.
#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub theme: Option<String>,
    pub dark_mode: Option<bool>,
    pub font_size: Option<u32>,
    pub max_connections: Option<usize>,
}

/// Creates a default instance of any type implementing `Default`.
///
/// This is a simple wrapper demonstrating generic functions with
/// `Default` trait bounds.
///
/// # Examples
///
/// ```
/// use default_trait_patterns::create_with_defaults;
///
/// let s: String = create_with_defaults();
/// assert_eq!(s, "");
///
/// let n: i32 = create_with_defaults();
/// assert_eq!(n, 0);
///
/// let v: Vec<u8> = create_with_defaults();
/// assert!(v.is_empty());
/// ```
pub fn create_with_defaults<T: Default>() -> T {
    T::default()
}

/// Merges a partial configuration with defaults.
///
/// Any `None` values in `partial` are replaced with the corresponding
/// default values from `AppConfig::default()`.
///
/// # Examples
///
/// ```
/// use default_trait_patterns::{merge_with_defaults, PartialConfig};
///
/// let partial = PartialConfig {
///     theme: Some("dark".to_string()),
///     dark_mode: Some(true),
///     font_size: None,
///     max_connections: None,
/// };
///
/// let config = merge_with_defaults(partial);
/// assert_eq!(config.theme, "dark");
/// assert!(config.dark_mode);
/// assert_eq!(config.font_size, 14);  // Used default
/// assert_eq!(config.max_connections, 100);  // Used default
/// ```
pub fn merge_with_defaults(partial: PartialConfig) -> AppConfig {
    let defaults = AppConfig::default();
    AppConfig {
        theme: partial.theme.unwrap_or(defaults.theme),
        dark_mode: partial.dark_mode.unwrap_or(defaults.dark_mode),
        font_size: partial.font_size.unwrap_or(defaults.font_size),
        max_connections: partial.max_connections.unwrap_or(defaults.max_connections),
    }
}

/// Creates a vector with `count` elements, each initialized to `T::default()`.
///
/// # Examples
///
/// ```
/// use default_trait_patterns::default_vec;
///
/// let zeros: Vec<i32> = default_vec(5);
/// assert_eq!(zeros, vec![0, 0, 0, 0, 0]);
///
/// let empty_strings: Vec<String> = default_vec(3);
/// assert_eq!(empty_strings, vec!["", "", ""]);
///
/// let empty: Vec<bool> = default_vec(0);
/// assert!(empty.is_empty());
/// ```
pub fn default_vec<T: Default>(count: usize) -> Vec<T> {
    (0..count).map(|_| T::default()).collect()
}
