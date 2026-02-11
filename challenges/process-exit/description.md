# Process and Exit

In Rust, the `std::process` module provides functionality for working with the current process and spawning new processes. This challenge focuses on working with the current process - getting its ID, handling exit codes, and managing process information.

## Key Concepts

### Process ID

Every running process has a unique identifier (PID) assigned by the operating system. You can get the current process ID using `std::process::id()`.

### Exit Codes

When a program terminates, it returns an exit code to the operating system:

- **0** typically indicates success
- **Non-zero values** indicate various types of failures

The `std::process::ExitCode` type provides a type-safe way to represent exit codes. You can also use `std::process::exit()` to terminate a program immediately with a specific exit code.

### Abort

In extreme cases, you can use `std::process::abort()` to terminate the process abnormally. This is typically used when the program is in an unrecoverable state.

## Your Task

Implement the following functions to work with process information and exit codes:

1. **`get_process_id`** - Returns the current process ID as a `u32`

2. **`exit_code_success`** - Returns an `ExitCode` representing success (0)

3. **`exit_code_failure`** - Returns an `ExitCode` representing failure (1)

4. **`exit_code_from_u8`** - Converts a `u8` value to an `ExitCode`

5. **`is_success_code`** - Checks if a given `u8` exit code represents success (0)

6. **`is_failure_code`** - Checks if a given `u8` exit code represents failure (non-zero)

7. **`describe_exit_code`** - Returns a description string for common exit codes:
   - 0 -> "success"
   - 1 -> "general error"
   - 2 -> "misuse of command"
   - 126 -> "command not executable"
   - 127 -> "command not found"
   - 128 -> "invalid exit argument"
   - 130 -> "script terminated by ctrl-c"
   - Any other value -> "unknown exit code: N" (where N is the code)

8. **`validate_exit_code`** - Takes an `Option<u8>` and returns:
   - `Ok(code)` if Some and code is 0
   - `Err("process failed with code N")` if Some and code is non-zero
   - `Err("process did not return an exit code")` if None

## Examples

```rust
use process_exit::*;

// Get current process ID
let pid = get_process_id();
assert!(pid > 0); // PID is always positive

// Create exit codes
let success = exit_code_success();
let failure = exit_code_failure();
let custom = exit_code_from_u8(42);

// Check exit codes
assert!(is_success_code(0));
assert!(!is_success_code(1));
assert!(is_failure_code(1));
assert!(!is_failure_code(0));

// Describe exit codes
assert_eq!(describe_exit_code(0), "success");
assert_eq!(describe_exit_code(127), "command not found");
assert_eq!(describe_exit_code(42), "unknown exit code: 42");

// Validate exit codes
assert_eq!(validate_exit_code(Some(0)), Ok(0));
assert_eq!(
    validate_exit_code(Some(1)),
    Err("process failed with code 1".to_string())
);
assert_eq!(
    validate_exit_code(None),
    Err("process did not return an exit code".to_string())
);
```

## Hints

<details>
  <summary>Click here to reveal hints</summary>

- Use `std::process::id()` to get the current process ID
- `ExitCode::SUCCESS` and `ExitCode::FAILURE` are predefined constants
- Use `ExitCode::from(u8)` to create an exit code from a number
- For `describe_exit_code`, use a `match` expression with the common Unix exit codes
- Remember that exit code 0 is the only "success" code; all others indicate failure

</details>
