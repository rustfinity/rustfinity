The `std::env` module provides functions for inspecting and manipulating the process's environment. This includes reading and setting environment variables, accessing command-line arguments, and querying information about the current execution context like the current directory and executable path.

Environment variables are key-value pairs that configure how programs behave. They're commonly used for configuration that varies between environments (development vs production), storing sensitive information (API keys, database URLs), and communicating between parent and child processes. Rust provides safe, cross-platform APIs for working with these variables.

## Key Functions

```rust
use std::env;

// Reading environment variables
let path = env::var("PATH");           // Result<String, VarError>
let home = env::var("HOME").ok();      // Option<String>

// Check if a variable exists (returns OsString)
let exists = env::var_os("MY_VAR");    // Option<OsString>

// Setting environment variables (affects only current process)
env::set_var("MY_VAR", "my_value");
env::remove_var("MY_VAR");

// Command-line arguments
let args: Vec<String> = env::args().collect();
let program_name = &args[0];

// Current directory
let cwd = env::current_dir();          // Result<PathBuf, Error>

// Executable path
let exe = env::current_exe();          // Result<PathBuf, Error>
```

## Error Handling

`env::var()` returns a `Result` because the variable might not exist or might contain invalid Unicode:

```rust
use std::env;

match env::var("DATABASE_URL") {
    Ok(url) => println!("Database URL: {}", url),
    Err(env::VarError::NotPresent) => println!("DATABASE_URL not set"),
    Err(env::VarError::NotUnicode(_)) => println!("DATABASE_URL contains invalid UTF-8"),
}

// For optional variables, use .ok() or .unwrap_or()
let debug = env::var("DEBUG").unwrap_or_else(|_| String::from("false"));
```

## Your Task

Implement the following functions to work with environment variables and process information:

**1. `get_env_var(key: &str) -> Option<String>`**

Get an environment variable by key. Return `None` if it
doesn't exist or contains invalid Unicode.

**2. `get_env_var_or_default(key: &str, default: &str) -> String`**

Get an environment variable, returning a default value
if it doesn't exist.

**3. `get_multiple_env_vars(keys: &[&str]) -> Vec<(String, Option<String>)>`**

Get multiple environment variables at once. Return a
vector of (key, value) pairs where value is `None` if
the variable doesn't exist.

**4. `parse_env_var<T: std::str::FromStr>(key: &str) -> Option<T>`**

Get an environment variable and parse it to type T.
Return `None` if the variable doesn't exist or can't
be parsed.

**5. `get_args() -> Vec<String>`**

Get all command-line arguments as a vector of strings.

**6. `get_current_dir() -> Option<String>`**

Get the current working directory as a string. Return
`None` if it can't be determined or converted to UTF-8.

**7. `get_current_exe_name() -> Option<String>`**

Get the name of the current executable (just the file
name, not the full path). Return `None` if it can't
be determined.

**8. `env_var_is_set(key: &str) -> bool`**

Check if an environment variable is set (even if it's
an empty string).

## Examples

```rust
use std::env;

// Set up some test variables
env::set_var("MY_APP_PORT", "8080");
env::set_var("MY_APP_DEBUG", "true");
env::set_var("MY_APP_NAME", "test-app");

// Basic retrieval
assert_eq!(get_env_var("MY_APP_NAME"), Some("test-app".to_string()));
assert_eq!(get_env_var("NONEXISTENT"), None);

// With default
assert_eq!(
    get_env_var_or_default("MY_APP_NAME", "default"),
    "test-app".to_string()
);
assert_eq!(
    get_env_var_or_default("NONEXISTENT", "default"),
    "default".to_string()
);

// Multiple variables
let vars = get_multiple_env_vars(&[
    "MY_APP_PORT",
    "MY_APP_DEBUG",
    "NONEXISTENT",
]);
assert_eq!(vars.len(), 3);
assert_eq!(
    vars[0],
    ("MY_APP_PORT".to_string(), Some("8080".to_string()))
);
assert_eq!(vars[2], ("NONEXISTENT".to_string(), None));

// Parsing
let port: Option<u16> = parse_env_var("MY_APP_PORT");
assert_eq!(port, Some(8080));

let debug: Option<bool> = parse_env_var("MY_APP_DEBUG");
assert_eq!(debug, Some(true));

let missing: Option<i32> = parse_env_var("NONEXISTENT");
assert_eq!(missing, None);

// Arguments (first is typically the program name)
let args = get_args();
assert!(!args.is_empty());

// Current directory
let cwd = get_current_dir();
assert!(cwd.is_some());

// Check if set
assert!(env_var_is_set("MY_APP_PORT"));
assert!(!env_var_is_set("DEFINITELY_NOT_SET_12345"));

// Clean up
env::remove_var("MY_APP_PORT");
env::remove_var("MY_APP_DEBUG");
env::remove_var("MY_APP_NAME");
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `std::env::var()` which returns `Result<String, VarError>`
- Convert `Result` to `Option` using `.ok()`
- For parsing, chain `.ok()?.parse().ok()` to handle both missing and parse errors
- `std::env::args()` returns an iterator, use `.collect()` to get a `Vec<String>`
- `std::env::current_dir()` returns `Result<PathBuf, Error>`
- Use `.to_str()` or `.to_string_lossy()` to convert `PathBuf` to string
- `std::env::current_exe()` gives the full path; use `.file_name()` to get just the name
- `std::env::var_os()` returns `Option<OsString>` and can
  detect if a variable is set regardless of content

</details>
