use std::io::Cursor;
use stdio_operations::*;

// ==================== read_line_from_reader tests ====================

#[test]
fn test_read_line_simple() {
    let input = Cursor::new("Hello, World!\n");
    let result = read_line_from_reader(input).unwrap();
    assert_eq!(result, "Hello, World!");
}

#[test]
fn test_read_line_no_newline() {
    let input = Cursor::new("No newline at end");
    let result = read_line_from_reader(input).unwrap();
    assert_eq!(result, "No newline at end");
}

#[test]
fn test_read_line_windows_newline() {
    let input = Cursor::new("Windows line\r\n");
    let result = read_line_from_reader(input).unwrap();
    assert_eq!(result, "Windows line");
}

#[test]
fn test_read_line_empty() {
    let input = Cursor::new("");
    let result = read_line_from_reader(input).unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_read_line_only_newline() {
    let input = Cursor::new("\n");
    let result = read_line_from_reader(input).unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_read_line_with_spaces() {
    let input = Cursor::new("  spaces around  \n");
    let result = read_line_from_reader(input).unwrap();
    assert_eq!(result, "  spaces around  ");
}

#[test]
fn test_read_line_unicode() {
    let input = Cursor::new("Hello, \u{4E16}\u{754C}!\n");
    let result = read_line_from_reader(input).unwrap();
    assert_eq!(result, "Hello, \u{4E16}\u{754C}!");
}

#[test]
fn test_read_line_multiline_only_first() {
    let input = Cursor::new("First line\nSecond line\n");
    let result = read_line_from_reader(input).unwrap();
    assert_eq!(result, "First line");
}

// ==================== read_all_lines_from_reader tests ====================

#[test]
fn test_read_all_lines_simple() {
    let input = Cursor::new("Line 1\nLine 2\nLine 3\n");
    let result = read_all_lines_from_reader(input).unwrap();
    assert_eq!(result, vec!["Line 1", "Line 2", "Line 3"]);
}

#[test]
fn test_read_all_lines_no_trailing_newline() {
    let input = Cursor::new("Line 1\nLine 2\nLine 3");
    let result = read_all_lines_from_reader(input).unwrap();
    assert_eq!(result, vec!["Line 1", "Line 2", "Line 3"]);
}

#[test]
fn test_read_all_lines_empty() {
    let input = Cursor::new("");
    let result = read_all_lines_from_reader(input).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_read_all_lines_single_line() {
    let input = Cursor::new("Single line\n");
    let result = read_all_lines_from_reader(input).unwrap();
    assert_eq!(result, vec!["Single line"]);
}

#[test]
fn test_read_all_lines_blank_lines() {
    let input = Cursor::new("Line 1\n\nLine 3\n");
    let result = read_all_lines_from_reader(input).unwrap();
    assert_eq!(result, vec!["Line 1", "", "Line 3"]);
}

#[test]
fn test_read_all_lines_windows_newlines() {
    let input = Cursor::new("Line 1\r\nLine 2\r\n");
    let result = read_all_lines_from_reader(input).unwrap();
    assert_eq!(result, vec!["Line 1", "Line 2"]);
}

#[test]
fn test_read_all_lines_unicode() {
    let input = Cursor::new("\u{4E2D}\u{6587}\n\u{65E5}\u{672C}\u{8A9E}\n\u{D55C}\u{AD6D}\u{C5B4}\n");
    let result = read_all_lines_from_reader(input).unwrap();
    assert_eq!(result, vec!["\u{4E2D}\u{6587}", "\u{65E5}\u{672C}\u{8A9E}", "\u{D55C}\u{AD6D}\u{C5B4}"]);
}

// ==================== write_to_writer tests ====================

#[test]
fn test_write_simple() {
    let mut output = Vec::new();
    write_to_writer(&mut output, "Hello").unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "Hello");
}

#[test]
fn test_write_empty() {
    let mut output = Vec::new();
    write_to_writer(&mut output, "").unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "");
}

#[test]
fn test_write_with_newline_in_message() {
    let mut output = Vec::new();
    write_to_writer(&mut output, "Line 1\nLine 2").unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "Line 1\nLine 2");
}

#[test]
fn test_write_unicode() {
    let mut output = Vec::new();
    write_to_writer(&mut output, "\u{1F600} Emoji!").unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "\u{1F600} Emoji!");
}

#[test]
fn test_write_multiple_times() {
    let mut output = Vec::new();
    write_to_writer(&mut output, "Hello, ").unwrap();
    write_to_writer(&mut output, "World!").unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "Hello, World!");
}

// ==================== writeln_to_writer tests ====================

#[test]
fn test_writeln_simple() {
    let mut output = Vec::new();
    writeln_to_writer(&mut output, "Hello").unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "Hello\n");
}

#[test]
fn test_writeln_empty() {
    let mut output = Vec::new();
    writeln_to_writer(&mut output, "").unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "\n");
}

#[test]
fn test_writeln_multiple_times() {
    let mut output = Vec::new();
    writeln_to_writer(&mut output, "Line 1").unwrap();
    writeln_to_writer(&mut output, "Line 2").unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "Line 1\nLine 2\n");
}

#[test]
fn test_writeln_unicode() {
    let mut output = Vec::new();
    writeln_to_writer(&mut output, "\u{4E16}\u{754C}").unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "\u{4E16}\u{754C}\n");
}

// ==================== write_and_flush tests ====================

#[test]
fn test_write_and_flush_simple() {
    let mut output = Vec::new();
    write_and_flush(&mut output, "Prompt: ").unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "Prompt: ");
}

#[test]
fn test_write_and_flush_empty() {
    let mut output = Vec::new();
    write_and_flush(&mut output, "").unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "");
}

#[test]
fn test_write_and_flush_multiple() {
    let mut output = Vec::new();
    write_and_flush(&mut output, "Enter name: ").unwrap();
    write_and_flush(&mut output, ">> ").unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "Enter name: >> ");
}

// ==================== write_error_to_writer tests ====================

#[test]
fn test_write_error_simple() {
    let mut output = Vec::new();
    write_error_to_writer(&mut output, "File not found").unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "[ERROR] File not found\n");
}

#[test]
fn test_write_error_empty() {
    let mut output = Vec::new();
    write_error_to_writer(&mut output, "").unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "[ERROR] \n");
}

#[test]
fn test_write_error_with_details() {
    let mut output = Vec::new();
    write_error_to_writer(&mut output, "Connection refused: timeout after 30s").unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "[ERROR] Connection refused: timeout after 30s\n");
}

#[test]
fn test_write_error_unicode() {
    let mut output = Vec::new();
    write_error_to_writer(&mut output, "\u{9519}\u{8BEF}").unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "[ERROR] \u{9519}\u{8BEF}\n");
}

#[test]
fn test_write_error_multiple() {
    let mut output = Vec::new();
    write_error_to_writer(&mut output, "Error 1").unwrap();
    write_error_to_writer(&mut output, "Error 2").unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "[ERROR] Error 1\n[ERROR] Error 2\n");
}

// ==================== Integration tests ====================

#[test]
fn test_integration_read_then_echo() {
    // Simulate reading input and echoing it back
    let input = Cursor::new("User input here\n");
    let line = read_line_from_reader(input).unwrap();

    let mut output = Vec::new();
    writeln_to_writer(&mut output, &format!("You entered: {}", line)).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "You entered: User input here\n");
}

#[test]
fn test_integration_prompt_pattern() {
    // Simulate a prompt-and-input pattern
    let mut output = Vec::new();

    // Write prompt without newline and flush
    write_and_flush(&mut output, "Enter your name: ").unwrap();
    assert_eq!(String::from_utf8(output.clone()).unwrap(), "Enter your name: ");

    // Simulate user input
    let input = Cursor::new("Alice\n");
    let name = read_line_from_reader(input).unwrap();

    // Write response
    writeln_to_writer(&mut output, &format!("Hello, {}!", name)).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "Enter your name: Hello, Alice!\n");
}

#[test]
fn test_integration_read_process_write() {
    // Read multiple lines, process them, and write results
    let input = Cursor::new("apple\nbanana\ncherry\n");
    let lines = read_all_lines_from_reader(input).unwrap();

    let mut output = Vec::new();
    for line in lines {
        writeln_to_writer(&mut output, &line.to_uppercase()).unwrap();
    }

    assert_eq!(String::from_utf8(output).unwrap(), "APPLE\nBANANA\nCHERRY\n");
}

#[test]
fn test_integration_error_handling_pattern() {
    // Simulate a program that writes normal output and errors
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    writeln_to_writer(&mut stdout, "Processing file...").unwrap();
    write_error_to_writer(&mut stderr, "Warning: file is empty").unwrap();
    writeln_to_writer(&mut stdout, "Done.").unwrap();

    assert_eq!(String::from_utf8(stdout).unwrap(), "Processing file...\nDone.\n");
    assert_eq!(String::from_utf8(stderr).unwrap(), "[ERROR] Warning: file is empty\n");
}

#[test]
fn test_integration_mixed_write_and_writeln() {
    let mut output = Vec::new();

    write_to_writer(&mut output, "Name: ").unwrap();
    writeln_to_writer(&mut output, "Alice").unwrap();
    write_to_writer(&mut output, "Age: ").unwrap();
    writeln_to_writer(&mut output, "30").unwrap();

    assert_eq!(String::from_utf8(output).unwrap(), "Name: Alice\nAge: 30\n");
}

#[test]
fn test_integration_large_input() {
    // Create a large input with many lines
    let lines: Vec<String> = (1..=100).map(|i| format!("Line {}", i)).collect();
    let content = lines.join("\n") + "\n";
    let input = Cursor::new(content);

    let result = read_all_lines_from_reader(input).unwrap();
    assert_eq!(result.len(), 100);
    assert_eq!(result[0], "Line 1");
    assert_eq!(result[99], "Line 100");
}
