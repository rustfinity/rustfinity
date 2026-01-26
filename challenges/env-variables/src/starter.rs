use std::env;
use std::str::FromStr;

/// Gets an environment variable by key.
///
/// Returns `None` if the variable doesn't exist or contains invalid Unicode.
pub fn get_env_var(key: &str) -> Option<String> {
    // TODO: Use env::var() and convert the Result to Option
    unimplemented!()
}

/// Gets an environment variable, returning a default value if it doesn't exist.
pub fn get_env_var_or_default(key: &str, default: &str) -> String {
    // TODO: Use env::var() with unwrap_or_else to provide a default
    unimplemented!()
}

/// Gets multiple environment variables at once.
///
/// Returns a vector of (key, value) pairs where value is `None` if the variable doesn't exist.
pub fn get_multiple_env_vars(keys: &[&str]) -> Vec<(String, Option<String>)> {
    // TODO: Iterate over keys and collect (key, value) pairs
    // Use get_env_var or env::var().ok() for each key
    unimplemented!()
}

/// Gets an environment variable and parses it to type T.
///
/// Returns `None` if the variable doesn't exist or can't be parsed.
pub fn parse_env_var<T: FromStr>(key: &str) -> Option<T> {
    // TODO: Get the env var, then parse it
    // Chain .ok()?.parse().ok() to handle both missing and parse errors
    unimplemented!()
}

/// Gets all command-line arguments as a vector of strings.
pub fn get_args() -> Vec<String> {
    // TODO: Use env::args() and collect into a Vec
    unimplemented!()
}

/// Gets the current working directory as a string.
///
/// Returns `None` if the current directory can't be determined or converted to UTF-8.
pub fn get_current_dir() -> Option<String> {
    // TODO: Use env::current_dir() and convert PathBuf to String
    // Remember to handle the Result and Option conversions
    unimplemented!()
}

/// Gets the name of the current executable (just the file name, not the full path).
///
/// Returns `None` if the executable path can't be determined.
pub fn get_current_exe_name() -> Option<String> {
    // TODO: Use env::current_exe() to get the path
    // Then use .file_name() to extract just the name
    // Convert OsStr to String
    unimplemented!()
}

/// Checks if an environment variable is set.
///
/// Returns `true` if the variable is set, even if it's an empty string.
pub fn env_var_is_set(key: &str) -> bool {
    // TODO: Use env::var_os() which returns Option<OsString>
    // This can detect if a variable is set regardless of content
    unimplemented!()
}

// Example usage
pub fn main() {
    // Set some test environment variables
    env::set_var("MY_APP_NAME", "test-app");
    env::set_var("MY_APP_PORT", "8080");
    env::set_var("MY_APP_DEBUG", "true");

    // Test basic retrieval
    println!("App name: {:?}", get_env_var("MY_APP_NAME"));
    println!("Missing var: {:?}", get_env_var("NONEXISTENT"));

    // Test with default
    println!("With default: {}", get_env_var_or_default("MY_APP_NAME", "default"));
    println!("Missing with default: {}", get_env_var_or_default("NONEXISTENT", "default"));

    // Test multiple variables
    let vars = get_multiple_env_vars(&["MY_APP_NAME", "MY_APP_PORT", "NONEXISTENT"]);
    println!("Multiple vars: {:?}", vars);

    // Test parsing
    let port: Option<u16> = parse_env_var("MY_APP_PORT");
    println!("Parsed port: {:?}", port);

    let debug: Option<bool> = parse_env_var("MY_APP_DEBUG");
    println!("Parsed debug: {:?}", debug);

    // Test arguments
    let args = get_args();
    println!("Args: {:?}", args);

    // Test current directory
    println!("Current dir: {:?}", get_current_dir());

    // Test executable name
    println!("Exe name: {:?}", get_current_exe_name());

    // Test is_set
    println!("MY_APP_NAME is set: {}", env_var_is_set("MY_APP_NAME"));
    println!("NONEXISTENT is set: {}", env_var_is_set("NONEXISTENT"));

    // Clean up
    env::remove_var("MY_APP_NAME");
    env::remove_var("MY_APP_PORT");
    env::remove_var("MY_APP_DEBUG");
}
