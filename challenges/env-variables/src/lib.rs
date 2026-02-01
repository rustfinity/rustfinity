use std::env;
use std::str::FromStr;

/// Gets an environment variable by key.
///
/// Returns `None` if the variable doesn't exist or contains invalid Unicode.
///
/// # Arguments
///
/// * `key` - The name of the environment variable
///
/// # Examples
///
/// ```
/// use env_variables::get_env_var;
/// use std::env;
///
/// env::set_var("TEST_VAR_1", "hello");
/// assert_eq!(get_env_var("TEST_VAR_1"), Some("hello".to_string()));
/// assert_eq!(get_env_var("NONEXISTENT_VAR_123"), None);
/// env::remove_var("TEST_VAR_1");
/// ```
pub fn get_env_var(key: &str) -> Option<String> {
    env::var(key).ok()
}

/// Gets an environment variable, returning a default value if it doesn't exist.
///
/// # Arguments
///
/// * `key` - The name of the environment variable
/// * `default` - The default value to return if the variable is not set
///
/// # Examples
///
/// ```
/// use env_variables::get_env_var_or_default;
/// use std::env;
///
/// env::set_var("TEST_VAR_2", "value");
/// assert_eq!(get_env_var_or_default("TEST_VAR_2", "default"), "value".to_string());
/// assert_eq!(get_env_var_or_default("NONEXISTENT_VAR_456", "default"), "default".to_string());
/// env::remove_var("TEST_VAR_2");
/// ```
pub fn get_env_var_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Gets multiple environment variables at once.
///
/// Returns a vector of (key, value) pairs where value is `None` if the variable doesn't exist.
///
/// # Arguments
///
/// * `keys` - A slice of environment variable names to retrieve
///
/// # Examples
///
/// ```
/// use env_variables::get_multiple_env_vars;
/// use std::env;
///
/// env::set_var("MULTI_A", "a_value");
/// env::set_var("MULTI_B", "b_value");
///
/// let vars = get_multiple_env_vars(&["MULTI_A", "MULTI_B", "MULTI_C"]);
/// assert_eq!(vars[0], ("MULTI_A".to_string(), Some("a_value".to_string())));
/// assert_eq!(vars[1], ("MULTI_B".to_string(), Some("b_value".to_string())));
/// assert_eq!(vars[2], ("MULTI_C".to_string(), None));
///
/// env::remove_var("MULTI_A");
/// env::remove_var("MULTI_B");
/// ```
pub fn get_multiple_env_vars(keys: &[&str]) -> Vec<(String, Option<String>)> {
    keys.iter()
        .map(|&key| (key.to_string(), env::var(key).ok()))
        .collect()
}

/// Gets an environment variable and parses it to type T.
///
/// Returns `None` if the variable doesn't exist or can't be parsed.
///
/// # Arguments
///
/// * `key` - The name of the environment variable
///
/// # Type Parameters
///
/// * `T` - The type to parse the environment variable into. Must implement `FromStr`.
///
/// # Examples
///
/// ```
/// use env_variables::parse_env_var;
/// use std::env;
///
/// env::set_var("PARSE_PORT", "8080");
/// env::set_var("PARSE_DEBUG", "true");
///
/// let port: Option<u16> = parse_env_var("PARSE_PORT");
/// assert_eq!(port, Some(8080));
///
/// let debug: Option<bool> = parse_env_var("PARSE_DEBUG");
/// assert_eq!(debug, Some(true));
///
/// let missing: Option<i32> = parse_env_var("NONEXISTENT_789");
/// assert_eq!(missing, None);
///
/// env::remove_var("PARSE_PORT");
/// env::remove_var("PARSE_DEBUG");
/// ```
pub fn parse_env_var<T: FromStr>(key: &str) -> Option<T> {
    env::var(key).ok()?.parse().ok()
}

/// Gets all command-line arguments as a vector of strings.
///
/// The first argument is typically the name or path of the executable.
///
/// # Examples
///
/// ```
/// use env_variables::get_args;
///
/// let args = get_args();
/// // At minimum, the program name should be present
/// assert!(!args.is_empty());
/// ```
pub fn get_args() -> Vec<String> {
    env::args().collect()
}

/// Gets the current working directory as a string.
///
/// Returns `None` if the current directory can't be determined or converted to UTF-8.
///
/// # Examples
///
/// ```
/// use env_variables::get_current_dir;
///
/// let cwd = get_current_dir();
/// assert!(cwd.is_some());
/// // The directory path should not be empty
/// assert!(!cwd.unwrap().is_empty());
/// ```
pub fn get_current_dir() -> Option<String> {
    env::current_dir()
        .ok()?
        .to_str()
        .map(|s| s.to_string())
}

/// Gets the name of the current executable (just the file name, not the full path).
///
/// Returns `None` if the executable path can't be determined or the file name
/// can't be converted to UTF-8.
///
/// # Examples
///
/// ```
/// use env_variables::get_current_exe_name;
///
/// let exe_name = get_current_exe_name();
/// // During tests, this will be the test runner executable
/// assert!(exe_name.is_some());
/// ```
pub fn get_current_exe_name() -> Option<String> {
    env::current_exe()
        .ok()?
        .file_name()?
        .to_str()
        .map(|s| s.to_string())
}

/// Checks if an environment variable is set.
///
/// Returns `true` if the variable is set, even if it's an empty string.
/// Returns `false` only if the variable is not present in the environment.
///
/// # Arguments
///
/// * `key` - The name of the environment variable to check
///
/// # Examples
///
/// ```
/// use env_variables::env_var_is_set;
/// use std::env;
///
/// env::set_var("IS_SET_TEST", "value");
/// env::set_var("EMPTY_VAR", "");
///
/// assert!(env_var_is_set("IS_SET_TEST"));
/// assert!(env_var_is_set("EMPTY_VAR"));  // Empty string still counts as set
/// assert!(!env_var_is_set("DEFINITELY_NOT_SET_ABC123"));
///
/// env::remove_var("IS_SET_TEST");
/// env::remove_var("EMPTY_VAR");
/// ```
pub fn env_var_is_set(key: &str) -> bool {
    env::var_os(key).is_some()
}
