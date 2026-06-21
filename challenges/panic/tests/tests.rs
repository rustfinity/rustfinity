use panic::*;
use std::sync::Mutex;

// All tests mutate the DATABASE_URL env var, which is global process state.
// Rust runs tests in parallel by default, causing race conditions.
// This mutex serializes access to prevent flakiness.
// We use lock_env() to handle mutex poisoning from #[should_panic] tests.
static ENV_MUTEX: Mutex<()> = Mutex::new(());

fn lock_env() -> std::sync::MutexGuard<'static, ()> {
    ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}

#[test]
fn test_get_database_url_valid() {
    let _lock = lock_env();
    std::env::set_var("DATABASE_URL", "postgresql://localhost");
    let db_url = get_database_url();
    assert_eq!(db_url, "postgresql://localhost");
}

#[test]
#[should_panic(expected = "DATABASE_URL environment variable is not set.")]
fn test_get_database_url_missing() {
    let _lock = lock_env();
    std::env::remove_var("DATABASE_URL");
    get_database_url();
}

#[test]
#[should_panic(expected = "DATABASE_URL must start with 'postgresql://'")]
fn test_get_database_url_invalid_prefix() {
    let _lock = lock_env();
    std::env::set_var("DATABASE_URL", "mysql://localhost");
    get_database_url();
}

#[test]
fn test_get_database_url_empty_value() {
    let _lock = lock_env();
    std::env::set_var("DATABASE_URL", "postgresql://");
    let db_url = get_database_url();
    assert_eq!(db_url, "postgresql://");
}

#[test]
fn test_get_database_url_with_long_valid_url() {
    let _lock = lock_env();
    let long_url = "postgresql://user:password@host:5432/database";
    std::env::set_var("DATABASE_URL", long_url);
    let db_url = get_database_url();
    assert_eq!(db_url, long_url);
}

#[test]
#[should_panic(expected = "DATABASE_URL must start with 'postgresql://'")]
fn test_get_database_url_prefix_case_sensitive() {
    let _lock = lock_env();
    std::env::set_var("DATABASE_URL", "PostgreSQL://localhost");
    get_database_url();
}

#[test]
fn test_get_database_url_with_special_characters() {
    let _lock = lock_env();
    let special_url = "postgresql://user:pa$$word@host:5432/dbname";
    std::env::set_var("DATABASE_URL", special_url);
    let db_url = get_database_url();
    assert_eq!(db_url, special_url);
}

#[test]
#[should_panic(expected = "DATABASE_URL must start with 'postgresql://'")]
fn test_get_database_url_numeric_prefix() {
    let _lock = lock_env();
    std::env::set_var("DATABASE_URL", "123postgresql://localhost");
    get_database_url();
}

#[test]
fn test_get_database_url_edge_case_minimal_valid() {
    let _lock = lock_env();
    std::env::set_var("DATABASE_URL", "postgresql://h");
    let db_url = get_database_url();
    assert_eq!(db_url, "postgresql://h");
}

#[test]
fn test_get_database_url_with_port_only() {
    let _lock = lock_env();
    let port_only_url = "postgresql://:5432";
    std::env::set_var("DATABASE_URL", port_only_url);
    let db_url = get_database_url();
    assert_eq!(db_url, port_only_url);
}
