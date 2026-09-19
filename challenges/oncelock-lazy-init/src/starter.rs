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

// TODO: declare the OnceLock<Settings> slot here

/// Builds the settings. Leave the counter bump in place: the tests use it
/// to prove the initializer runs exactly once.
fn build_settings() -> Settings {
    BUILD_COUNT.fetch_add(1, Ordering::Relaxed);

    Settings {
        name: "rustfinity".to_string(),
        retries: 3,
    }
}

/// Returns the process-wide settings, building them on first access.
pub fn settings() -> &'static Settings {
    // TODO: initialize the slot on first call and return a reference to it
    unimplemented!()
}

/// How many times the settings initializer ran. Always 0 or 1.
pub fn build_count() -> usize {
    // TODO
    unimplemented!()
}

/// The first 32 triangular numbers, computed on first use.
pub static TRIANGULAR: LazyLock<Vec<u64>> = LazyLock::new(|| todo!());

// Example usage
pub fn main() {
    println!("{:?}", settings());
    println!("{} {:?}", build_count(), &TRIANGULAR[..4]);
}
