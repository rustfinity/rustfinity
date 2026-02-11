use env_variables::*;
use std::env;

// Helper to generate unique test variable names to avoid conflicts between tests
fn test_var(name: &str) -> String {
    format!("ENV_VAR_TEST_{}", name)
}

// ==================== get_env_var tests ====================

#[test]
fn test_get_env_var_exists() {
    let key = test_var("GET_EXISTS");
    env::set_var(&key, "test_value");
    assert_eq!(get_env_var(&key), Some("test_value".to_string()));
    env::remove_var(&key);
}

#[test]
fn test_get_env_var_not_exists() {
    let key = test_var("NONEXISTENT_12345");
    env::remove_var(&key); // Ensure it doesn't exist
    assert_eq!(get_env_var(&key), None);
}

#[test]
fn test_get_env_var_empty_value() {
    let key = test_var("EMPTY_VALUE");
    env::set_var(&key, "");
    assert_eq!(get_env_var(&key), Some("".to_string()));
    env::remove_var(&key);
}

#[test]
fn test_get_env_var_with_spaces() {
    let key = test_var("WITH_SPACES");
    env::set_var(&key, "  hello world  ");
    assert_eq!(get_env_var(&key), Some("  hello world  ".to_string()));
    env::remove_var(&key);
}

#[test]
fn test_get_env_var_with_special_chars() {
    let key = test_var("SPECIAL_CHARS");
    env::set_var(&key, "hello=world&foo=bar");
    assert_eq!(get_env_var(&key), Some("hello=world&foo=bar".to_string()));
    env::remove_var(&key);
}

#[test]
fn test_get_env_var_unicode() {
    let key = test_var("UNICODE");
    env::set_var(&key, "Hello, \u{4e16}\u{754c}"); // Hello, 世界
    assert_eq!(get_env_var(&key), Some("Hello, 世界".to_string()));
    env::remove_var(&key);
}

// ==================== get_env_var_or_default tests ====================

#[test]
fn test_get_env_var_or_default_exists() {
    let key = test_var("DEFAULT_EXISTS");
    env::set_var(&key, "actual_value");
    assert_eq!(get_env_var_or_default(&key, "default"), "actual_value".to_string());
    env::remove_var(&key);
}

#[test]
fn test_get_env_var_or_default_not_exists() {
    let key = test_var("DEFAULT_NOT_EXISTS");
    env::remove_var(&key);
    assert_eq!(get_env_var_or_default(&key, "default_value"), "default_value".to_string());
}

#[test]
fn test_get_env_var_or_default_empty_value() {
    let key = test_var("DEFAULT_EMPTY");
    env::set_var(&key, "");
    // Empty string is a valid value, not the default
    assert_eq!(get_env_var_or_default(&key, "default"), "".to_string());
    env::remove_var(&key);
}

#[test]
fn test_get_env_var_or_default_empty_default() {
    let key = test_var("EMPTY_DEFAULT");
    env::remove_var(&key);
    assert_eq!(get_env_var_or_default(&key, ""), "".to_string());
}

#[test]
fn test_get_env_var_or_default_complex_default() {
    let key = test_var("COMPLEX_DEFAULT");
    env::remove_var(&key);
    assert_eq!(
        get_env_var_or_default(&key, "postgresql://localhost:5432/db"),
        "postgresql://localhost:5432/db".to_string()
    );
}

// ==================== get_multiple_env_vars tests ====================

#[test]
fn test_get_multiple_env_vars_all_exist() {
    let key_a = test_var("MULTI_A");
    let key_b = test_var("MULTI_B");
    env::set_var(&key_a, "value_a");
    env::set_var(&key_b, "value_b");

    let vars = get_multiple_env_vars(&[&key_a, &key_b]);
    assert_eq!(vars.len(), 2);
    assert_eq!(vars[0], (key_a.clone(), Some("value_a".to_string())));
    assert_eq!(vars[1], (key_b.clone(), Some("value_b".to_string())));

    env::remove_var(&key_a);
    env::remove_var(&key_b);
}

#[test]
fn test_get_multiple_env_vars_some_missing() {
    let key_a = test_var("MULTI_SOME_A");
    let key_b = test_var("MULTI_SOME_B");
    env::set_var(&key_a, "value_a");
    env::remove_var(&key_b);

    let vars = get_multiple_env_vars(&[&key_a, &key_b]);
    assert_eq!(vars.len(), 2);
    assert_eq!(vars[0], (key_a.clone(), Some("value_a".to_string())));
    assert_eq!(vars[1], (key_b.clone(), None));

    env::remove_var(&key_a);
}

#[test]
fn test_get_multiple_env_vars_all_missing() {
    let key_a = test_var("MULTI_MISSING_A");
    let key_b = test_var("MULTI_MISSING_B");
    env::remove_var(&key_a);
    env::remove_var(&key_b);

    let vars = get_multiple_env_vars(&[&key_a, &key_b]);
    assert_eq!(vars.len(), 2);
    assert_eq!(vars[0], (key_a.clone(), None));
    assert_eq!(vars[1], (key_b.clone(), None));
}

#[test]
fn test_get_multiple_env_vars_empty_slice() {
    let vars = get_multiple_env_vars(&[]);
    assert!(vars.is_empty());
}

#[test]
fn test_get_multiple_env_vars_single() {
    let key = test_var("MULTI_SINGLE");
    env::set_var(&key, "single_value");

    let vars = get_multiple_env_vars(&[&key]);
    assert_eq!(vars.len(), 1);
    assert_eq!(vars[0], (key.clone(), Some("single_value".to_string())));

    env::remove_var(&key);
}

#[test]
fn test_get_multiple_env_vars_preserves_order() {
    let key_c = test_var("MULTI_ORDER_C");
    let key_a = test_var("MULTI_ORDER_A");
    let key_b = test_var("MULTI_ORDER_B");
    env::set_var(&key_c, "c");
    env::set_var(&key_a, "a");
    env::set_var(&key_b, "b");

    // Order should match input order, not alphabetical
    let vars = get_multiple_env_vars(&[&key_c, &key_a, &key_b]);
    assert_eq!(vars[0].0, key_c);
    assert_eq!(vars[1].0, key_a);
    assert_eq!(vars[2].0, key_b);

    env::remove_var(&key_c);
    env::remove_var(&key_a);
    env::remove_var(&key_b);
}

// ==================== parse_env_var tests ====================

#[test]
fn test_parse_env_var_u16() {
    let key = test_var("PARSE_U16");
    env::set_var(&key, "8080");
    let port: Option<u16> = parse_env_var(&key);
    assert_eq!(port, Some(8080));
    env::remove_var(&key);
}

#[test]
fn test_parse_env_var_bool_true() {
    let key = test_var("PARSE_BOOL_TRUE");
    env::set_var(&key, "true");
    let val: Option<bool> = parse_env_var(&key);
    assert_eq!(val, Some(true));
    env::remove_var(&key);
}

#[test]
fn test_parse_env_var_bool_false() {
    let key = test_var("PARSE_BOOL_FALSE");
    env::set_var(&key, "false");
    let val: Option<bool> = parse_env_var(&key);
    assert_eq!(val, Some(false));
    env::remove_var(&key);
}

#[test]
fn test_parse_env_var_i32_negative() {
    let key = test_var("PARSE_I32_NEG");
    env::set_var(&key, "-42");
    let val: Option<i32> = parse_env_var(&key);
    assert_eq!(val, Some(-42));
    env::remove_var(&key);
}

#[test]
fn test_parse_env_var_f64() {
    let key = test_var("PARSE_F64");
    env::set_var(&key, "3.14159");
    let val: Option<f64> = parse_env_var(&key);
    assert!((val.unwrap() - 3.14159).abs() < 0.00001);
    env::remove_var(&key);
}

#[test]
fn test_parse_env_var_missing() {
    let key = test_var("PARSE_MISSING");
    env::remove_var(&key);
    let val: Option<i32> = parse_env_var(&key);
    assert_eq!(val, None);
}

#[test]
fn test_parse_env_var_invalid_format() {
    let key = test_var("PARSE_INVALID");
    env::set_var(&key, "not_a_number");
    let val: Option<i32> = parse_env_var(&key);
    assert_eq!(val, None);
    env::remove_var(&key);
}

#[test]
fn test_parse_env_var_overflow() {
    let key = test_var("PARSE_OVERFLOW");
    env::set_var(&key, "999999999999999999999");
    let val: Option<u32> = parse_env_var(&key);
    assert_eq!(val, None);
    env::remove_var(&key);
}

#[test]
fn test_parse_env_var_string() {
    let key = test_var("PARSE_STRING");
    env::set_var(&key, "hello");
    let val: Option<String> = parse_env_var(&key);
    assert_eq!(val, Some("hello".to_string()));
    env::remove_var(&key);
}

#[test]
fn test_parse_env_var_char() {
    let key = test_var("PARSE_CHAR");
    env::set_var(&key, "x");
    let val: Option<char> = parse_env_var(&key);
    assert_eq!(val, Some('x'));
    env::remove_var(&key);
}

#[test]
fn test_parse_env_var_char_multiple_fails() {
    let key = test_var("PARSE_CHAR_MULTI");
    env::set_var(&key, "xyz");
    let val: Option<char> = parse_env_var(&key);
    assert_eq!(val, None); // Can't parse multiple chars as single char
    env::remove_var(&key);
}

// ==================== get_args tests ====================

#[test]
fn test_get_args_not_empty() {
    let args = get_args();
    // There should always be at least the program name
    assert!(!args.is_empty());
}

#[test]
fn test_get_args_first_is_string() {
    let args = get_args();
    // First argument should be the executable path/name
    assert!(!args[0].is_empty());
}

// ==================== get_current_dir tests ====================

#[test]
fn test_get_current_dir_exists() {
    let cwd = get_current_dir();
    assert!(cwd.is_some());
}

#[test]
fn test_get_current_dir_not_empty() {
    let cwd = get_current_dir();
    assert!(!cwd.unwrap().is_empty());
}

#[test]
fn test_get_current_dir_is_valid_path() {
    let cwd = get_current_dir().unwrap();
    // Should be an absolute path (starts with / on Unix or drive letter on Windows)
    #[cfg(unix)]
    assert!(cwd.starts_with('/'));
    #[cfg(windows)]
    assert!(cwd.chars().nth(1) == Some(':'));
}

// ==================== get_current_exe_name tests ====================

#[test]
fn test_get_current_exe_name_exists() {
    let exe_name = get_current_exe_name();
    assert!(exe_name.is_some());
}

#[test]
fn test_get_current_exe_name_not_empty() {
    let exe_name = get_current_exe_name();
    assert!(!exe_name.unwrap().is_empty());
}

#[test]
fn test_get_current_exe_name_no_path_separator() {
    let exe_name = get_current_exe_name().unwrap();
    // Should be just the file name, no path separators
    assert!(!exe_name.contains('/'));
    #[cfg(windows)]
    assert!(!exe_name.contains('\\'));
}

// ==================== env_var_is_set tests ====================

#[test]
fn test_env_var_is_set_true() {
    let key = test_var("IS_SET_TRUE");
    env::set_var(&key, "some_value");
    assert!(env_var_is_set(&key));
    env::remove_var(&key);
}

#[test]
fn test_env_var_is_set_false() {
    let key = test_var("IS_SET_FALSE");
    env::remove_var(&key);
    assert!(!env_var_is_set(&key));
}

#[test]
fn test_env_var_is_set_empty_string() {
    let key = test_var("IS_SET_EMPTY");
    env::set_var(&key, "");
    // Empty string still counts as set
    assert!(env_var_is_set(&key));
    env::remove_var(&key);
}

#[test]
fn test_env_var_is_set_after_remove() {
    let key = test_var("IS_SET_REMOVE");
    env::set_var(&key, "temp");
    assert!(env_var_is_set(&key));
    env::remove_var(&key);
    assert!(!env_var_is_set(&key));
}

// ==================== Integration tests ====================

#[test]
fn test_integration_set_get_parse() {
    let key = test_var("INTEGRATION_1");
    env::set_var(&key, "42");

    // Should be set
    assert!(env_var_is_set(&key));

    // Should get the value
    assert_eq!(get_env_var(&key), Some("42".to_string()));

    // Should parse correctly
    let val: Option<i32> = parse_env_var(&key);
    assert_eq!(val, Some(42));

    // Default should return actual value
    assert_eq!(get_env_var_or_default(&key, "0"), "42".to_string());

    env::remove_var(&key);
}

#[test]
fn test_integration_multiple_vars_workflow() {
    let host_key = test_var("INT_HOST");
    let port_key = test_var("INT_PORT");
    let debug_key = test_var("INT_DEBUG");

    env::set_var(&host_key, "localhost");
    env::set_var(&port_key, "3000");
    // debug not set

    let vars = get_multiple_env_vars(&[&host_key, &port_key, &debug_key]);

    let host = vars.iter().find(|(k, _)| k == &host_key).unwrap().1.clone();
    let port = vars.iter().find(|(k, _)| k == &port_key).unwrap().1.clone();
    let debug = vars.iter().find(|(k, _)| k == &debug_key).unwrap().1.clone();

    assert_eq!(host, Some("localhost".to_string()));
    assert_eq!(port, Some("3000".to_string()));
    assert_eq!(debug, None);

    env::remove_var(&host_key);
    env::remove_var(&port_key);
}

#[test]
fn test_integration_config_pattern() {
    // Simulate a typical configuration pattern
    let app_name = test_var("APP_NAME");
    let app_port = test_var("APP_PORT");
    let app_debug = test_var("APP_DEBUG");
    let app_timeout = test_var("APP_TIMEOUT");

    env::set_var(&app_name, "my_app");
    env::set_var(&app_port, "8080");
    env::set_var(&app_debug, "true");
    // timeout not set, will use default

    // Build config from env vars
    let name = get_env_var_or_default(&app_name, "unnamed");
    let port: u16 = parse_env_var(&app_port).unwrap_or(3000);
    let debug: bool = parse_env_var(&app_debug).unwrap_or(false);
    let timeout: u64 = parse_env_var(&app_timeout).unwrap_or(30);

    assert_eq!(name, "my_app");
    assert_eq!(port, 8080);
    assert!(debug);
    assert_eq!(timeout, 30); // default

    env::remove_var(&app_name);
    env::remove_var(&app_port);
    env::remove_var(&app_debug);
}

#[test]
fn test_integration_environment_info() {
    // Test getting process/environment info
    let cwd = get_current_dir();
    let exe = get_current_exe_name();
    let args = get_args();

    // All should succeed
    assert!(cwd.is_some());
    assert!(exe.is_some());
    assert!(!args.is_empty());

    // cwd should be a directory that exists (we're running in it)
    let cwd_path = std::path::Path::new(cwd.as_ref().unwrap());
    assert!(cwd_path.is_absolute());
}

#[test]
fn test_integration_var_lifecycle() {
    let key = test_var("LIFECYCLE");

    // Initially not set
    assert!(!env_var_is_set(&key));
    assert_eq!(get_env_var(&key), None);

    // Set it
    env::set_var(&key, "first");
    assert!(env_var_is_set(&key));
    assert_eq!(get_env_var(&key), Some("first".to_string()));

    // Update it
    env::set_var(&key, "second");
    assert!(env_var_is_set(&key));
    assert_eq!(get_env_var(&key), Some("second".to_string()));

    // Remove it
    env::remove_var(&key);
    assert!(!env_var_is_set(&key));
    assert_eq!(get_env_var(&key), None);
}

#[test]
fn test_integration_path_like_value() {
    let key = test_var("PATH_VALUE");
    let path_value = "/usr/local/bin:/usr/bin:/bin";
    env::set_var(&key, path_value);

    assert_eq!(get_env_var(&key), Some(path_value.to_string()));

    // Can still get it with multiple vars
    let vars = get_multiple_env_vars(&[&key]);
    assert_eq!(vars[0].1, Some(path_value.to_string()));

    env::remove_var(&key);
}

#[test]
fn test_integration_url_value() {
    let key = test_var("URL_VALUE");
    let url = "https://user:pass@example.com:8080/path?query=value#fragment";
    env::set_var(&key, url);

    assert_eq!(get_env_var(&key), Some(url.to_string()));
    assert_eq!(get_env_var_or_default(&key, ""), url.to_string());

    env::remove_var(&key);
}
