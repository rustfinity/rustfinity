use std::ffi::{OsStr, OsString};
use std::path::Path;

/// Convert a `&str` to an `OsString`.
pub fn to_os_string(s: &str) -> OsString {
    // TODO: Use OsString::from() to convert the string
    unimplemented!()
}

/// Try to convert an `OsStr` to `&str`.
/// Returns `None` if the `OsStr` is not valid UTF-8.
pub fn os_str_to_str(os: &OsStr) -> Option<&str> {
    // TODO: Use the .to_str() method on OsStr
    unimplemented!()
}

/// Convert `OsStr` to `String`, replacing invalid UTF-8 with the replacement character.
pub fn os_string_to_string_lossy(os: &OsStr) -> String {
    // TODO: Use .to_string_lossy() and convert to owned String
    unimplemented!()
}

/// Extract the file extension from a path as a `String`.
/// Returns `None` if there is no extension or if the extension isn't valid UTF-8.
pub fn get_file_extension(path: &Path) -> Option<String> {
    // TODO: Use path.extension() to get the extension as OsStr
    // Then convert it to a String using .to_str()
    unimplemented!()
}

/// Join multiple path components into a single `OsString` path.
/// Uses the platform-appropriate path separator.
pub fn join_path_components(components: &[&str]) -> OsString {
    // TODO: Use PathBuf to build the path, then convert to OsString
    // Hint: PathBuf::new() creates an empty path, .push() adds components
    unimplemented!()
}

/// Check if an `OsStr` contains valid UTF-8.
pub fn is_valid_utf8(os: &OsStr) -> bool {
    // TODO: Check if .to_str() returns Some
    unimplemented!()
}

pub fn main() {
    println!("=== OsString and OsStr Basics ===\n");

    // Test to_os_string
    let os = to_os_string("hello.txt");
    println!("to_os_string(\"hello.txt\"): {:?}", os);

    // Test os_str_to_str
    let os_str = OsStr::new("valid_string.rs");
    println!(
        "os_str_to_str(OsStr::new(\"valid_string.rs\")): {:?}",
        os_str_to_str(os_str)
    );

    // Test os_string_to_string_lossy
    let os_str = OsStr::new("hello world");
    println!(
        "os_string_to_string_lossy: {}",
        os_string_to_string_lossy(os_str)
    );

    // Test get_file_extension
    let path = Path::new("document.pdf");
    println!(
        "get_file_extension(Path::new(\"document.pdf\")): {:?}",
        get_file_extension(path)
    );

    // Test join_path_components
    let components = &["home", "user", "documents"];
    let joined = join_path_components(components);
    println!(
        "join_path_components(&{:?}): {:?}",
        components, joined
    );

    // Test is_valid_utf8
    let os_str = OsStr::new("valid utf8 string");
    println!(
        "is_valid_utf8(OsStr::new(\"valid utf8 string\")): {}",
        is_valid_utf8(os_str)
    );
}
