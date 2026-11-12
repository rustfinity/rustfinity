use std::process::Command;
use the_unit_type::*;

const CHILD_ENV: &str = "THE_UNIT_TYPE_PRINT_CHILD";
const BEGIN: &str = "<<print_message begin>>";
const END: &str = "<<print_message end>>";

#[test]
fn test_returns_unit() {
    let unit = print_message();

    assert_eq!(unit, (), "The function should return the unit type `()`");
}

/// Re-runs this test binary with only this test selected and output capture
/// off, then checks what `print_message` wrote to stdout between two markers.
#[test]
fn test_prints_hello_rust() {
    if std::env::var_os(CHILD_ENV).is_some() {
        print!("{BEGIN}");
        print_message();
        print!("{END}");
        return;
    }

    let output = Command::new(std::env::current_exe().unwrap())
        .args([
            "test_prints_hello_rust",
            "--exact",
            "--nocapture",
            "--test-threads=1",
        ])
        .env(CHILD_ENV, "1")
        .output()
        .expect("failed to re-run the test binary");
    let stdout = String::from_utf8_lossy(&output.stdout);

    let start = stdout.find(BEGIN).expect("print_message did not run") + BEGIN.len();
    let end = start
        + stdout[start..]
            .find(END)
            .expect("print_message did not return");

    assert_eq!(
        &stdout[start..end],
        "Hello, Rust!\n",
        "The function should print exactly `Hello, Rust!` followed by a newline"
    );
}
