use std::process::ExitCode;

/// Returns the current process ID.
///
/// Every running process has a unique identifier (PID) assigned by the
/// operating system. This function returns that ID.
///
/// # Examples
///
/// ```
/// use process_exit::get_process_id;
///
/// let pid = get_process_id();
/// assert!(pid > 0); // PIDs are always positive
/// ```
pub fn get_process_id() -> u32 {
    std::process::id()
}

/// Returns an ExitCode representing success (0).
///
/// By convention, exit code 0 indicates that a program completed successfully.
///
/// # Examples
///
/// ```
/// use process_exit::exit_code_success;
/// use std::process::ExitCode;
///
/// let code = exit_code_success();
/// // ExitCode::SUCCESS represents exit code 0
/// ```
pub fn exit_code_success() -> ExitCode {
    ExitCode::SUCCESS
}

/// Returns an ExitCode representing failure (1).
///
/// By convention, exit code 1 indicates a general error condition.
///
/// # Examples
///
/// ```
/// use process_exit::exit_code_failure;
/// use std::process::ExitCode;
///
/// let code = exit_code_failure();
/// // ExitCode::FAILURE represents exit code 1
/// ```
pub fn exit_code_failure() -> ExitCode {
    ExitCode::FAILURE
}

/// Converts a u8 value to an ExitCode.
///
/// This allows creating an exit code with any value from 0 to 255.
///
/// # Arguments
///
/// * `code` - The exit code value (0-255)
///
/// # Examples
///
/// ```
/// use process_exit::exit_code_from_u8;
///
/// let code = exit_code_from_u8(42);
/// // Creates an ExitCode with value 42
/// ```
pub fn exit_code_from_u8(code: u8) -> ExitCode {
    ExitCode::from(code)
}

/// Checks if a given exit code represents success.
///
/// In Unix conventions, exit code 0 indicates success.
///
/// # Arguments
///
/// * `code` - The exit code to check
///
/// # Examples
///
/// ```
/// use process_exit::is_success_code;
///
/// assert!(is_success_code(0));
/// assert!(!is_success_code(1));
/// assert!(!is_success_code(255));
/// ```
pub fn is_success_code(code: u8) -> bool {
    code == 0
}

/// Checks if a given exit code represents failure.
///
/// In Unix conventions, any non-zero exit code indicates failure.
///
/// # Arguments
///
/// * `code` - The exit code to check
///
/// # Examples
///
/// ```
/// use process_exit::is_failure_code;
///
/// assert!(!is_failure_code(0));
/// assert!(is_failure_code(1));
/// assert!(is_failure_code(127));
/// ```
pub fn is_failure_code(code: u8) -> bool {
    code != 0
}

/// Returns a description string for common exit codes.
///
/// Common Unix exit codes have specific meanings:
/// - 0: success
/// - 1: general error
/// - 2: misuse of command
/// - 126: command not executable
/// - 127: command not found
/// - 128: invalid exit argument
/// - 130: script terminated by ctrl-c
///
/// # Arguments
///
/// * `code` - The exit code to describe
///
/// # Examples
///
/// ```
/// use process_exit::describe_exit_code;
///
/// assert_eq!(describe_exit_code(0), "success");
/// assert_eq!(describe_exit_code(1), "general error");
/// assert_eq!(describe_exit_code(127), "command not found");
/// assert_eq!(describe_exit_code(42), "unknown exit code: 42");
/// ```
pub fn describe_exit_code(code: u8) -> String {
    match code {
        0 => "success".to_string(),
        1 => "general error".to_string(),
        2 => "misuse of command".to_string(),
        126 => "command not executable".to_string(),
        127 => "command not found".to_string(),
        128 => "invalid exit argument".to_string(),
        130 => "script terminated by ctrl-c".to_string(),
        _ => format!("unknown exit code: {}", code),
    }
}

/// Validates an optional exit code.
///
/// Returns Ok with the code if it's Some(0), indicating success.
/// Returns Err with an appropriate message otherwise.
///
/// # Arguments
///
/// * `code` - An optional exit code to validate
///
/// # Examples
///
/// ```
/// use process_exit::validate_exit_code;
///
/// assert_eq!(validate_exit_code(Some(0)), Ok(0));
/// assert_eq!(validate_exit_code(Some(1)), Err("process failed with code 1".to_string()));
/// assert_eq!(validate_exit_code(None), Err("process did not return an exit code".to_string()));
/// ```
pub fn validate_exit_code(code: Option<u8>) -> Result<u8, String> {
    match code {
        Some(0) => Ok(0),
        Some(c) => Err(format!("process failed with code {}", c)),
        None => Err("process did not return an exit code".to_string()),
    }
}
