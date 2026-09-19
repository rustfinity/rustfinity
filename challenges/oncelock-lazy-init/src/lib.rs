use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, OnceLock};

/// Settings built once, on first use, and shared for the rest of the process.
#[derive(Debug, PartialEq, Eq)]
pub struct Settings {
    pub name: String,
    pub retries: u32,
}

/// How many times the `Settings` initializer has actually run.
static BUILD_COUNT: AtomicUsize = AtomicUsize::new(0);

/// The storage slot. Empty until the first `settings()` call fills it.
static SETTINGS: OnceLock<Settings> = OnceLock::new();

/// Builds the settings. Deliberately counted so the tests can prove this
/// runs exactly once, no matter how many threads race for it.
fn build_settings() -> Settings {
    BUILD_COUNT.fetch_add(1, Ordering::Relaxed);

    Settings {
        name: "rustfinity".to_string(),
        retries: 3,
    }
}

/// Returns the process-wide settings, building them on first access.
///
/// Every caller gets a reference to the *same* value. If several threads
/// arrive at once, one of them runs the initializer and the rest block
/// until it is done, then receive that result.
///
/// # Examples
///
/// ```
/// use oncelock_lazy_init::settings;
///
/// assert_eq!(settings().retries, 3);
/// assert_eq!(settings().name, "rustfinity");
/// ```
pub fn settings() -> &'static Settings {
    SETTINGS.get_or_init(build_settings)
}

/// How many times the settings initializer ran. Always 0 or 1.
///
/// # Examples
///
/// ```
/// use oncelock_lazy_init::{build_count, settings};
///
/// settings();
/// settings();
/// assert_eq!(build_count(), 1);
/// ```
pub fn build_count() -> usize {
    BUILD_COUNT.load(Ordering::Relaxed)
}

/// The first 32 triangular numbers, computed on first use.
///
/// `LazyLock` is `OnceLock` with the initializer baked in, which lets it
/// be used like a plain `static` while still doing no work at startup.
///
/// # Examples
///
/// ```
/// use oncelock_lazy_init::TRIANGULAR;
///
/// assert_eq!(TRIANGULAR[0], 0);
/// assert_eq!(TRIANGULAR[3], 6);
/// ```
pub static TRIANGULAR: LazyLock<Vec<u64>> =
    LazyLock::new(|| (0..32).map(|n: u64| n * (n + 1) / 2).collect());
