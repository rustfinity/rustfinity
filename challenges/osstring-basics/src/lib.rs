use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

/// Convert a `&str` to an `OsString`.
///
/// # Examples
///
/// ```
/// use osstring_basics::to_os_string;
/// use std::ffi::OsString;
///
/// let os = to_os_string("hello.txt");
/// assert_eq!(os, OsString::from("hello.txt"));
/// ```
pub fn to_os_string(s: &str) -> OsString {
    OsString::from(s)
}

/// Try to convert an `OsStr` to `&str`.
/// Returns `None` if the `OsStr` is not valid UTF-8.
///
/// # Examples
///
/// ```
/// use osstring_basics::os_str_to_str;
/// use std::ffi::OsStr;
///
/// let os_str = OsStr::new("hello.txt");
/// assert_eq!(os_str_to_str(os_str), Some("hello.txt"));
/// ```
pub fn os_str_to_str(os: &OsStr) -> Option<&str> {
    os.to_str()
}

/// Convert `OsStr` to `String`, replacing invalid UTF-8 with the replacement character.
///
/// # Examples
///
/// ```
/// use osstring_basics::os_string_to_string_lossy;
/// use std::ffi::OsStr;
///
/// let os_str = OsStr::new("valid utf8");
/// assert_eq!(os_string_to_string_lossy(os_str), "valid utf8");
/// ```
pub fn os_string_to_string_lossy(os: &OsStr) -> String {
    os.to_string_lossy().into_owned()
}

/// Extract the file extension from a path as a `String`.
/// Returns `None` if there is no extension or if the extension isn't valid UTF-8.
///
/// # Examples
///
/// ```
/// use osstring_basics::get_file_extension;
/// use std::path::Path;
///
/// let path = Path::new("document.pdf");
/// assert_eq!(get_file_extension(path), Some("pdf".to_string()));
///
/// let path = Path::new("no_extension");
/// assert_eq!(get_file_extension(path), None);
/// ```
pub fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()?.to_str().map(|s| s.to_string())
}

/// Join multiple path components into a single `OsString` path.
/// Uses the platform-appropriate path separator.
///
/// # Examples
///
/// ```
/// use osstring_basics::join_path_components;
/// use std::path::Path;
///
/// let path = join_path_components(&["home", "user", "documents"]);
/// // This creates a proper path for the current platform
/// let expected = Path::new("home").join("user").join("documents");
/// assert_eq!(path, expected.into_os_string());
/// ```
pub fn join_path_components(components: &[&str]) -> OsString {
    let mut path_buf = PathBuf::new();
    for component in components {
        path_buf.push(component);
    }
    path_buf.into_os_string()
}

/// Check if an `OsStr` contains valid UTF-8.
///
/// # Examples
///
/// ```
/// use osstring_basics::is_valid_utf8;
/// use std::ffi::OsStr;
///
/// let os_str = OsStr::new("hello");
/// assert!(is_valid_utf8(os_str));
/// ```
pub fn is_valid_utf8(os: &OsStr) -> bool {
    os.to_str().is_some()
}

pub fn main() {
    println!("=== OsString and OsStr Basics ===\n");

    // Demonstrate to_os_string
    let os = to_os_string("hello.txt");
    println!("to_os_string(\"hello.txt\"): {:?}", os);

    // Demonstrate os_str_to_str
    let os_str = OsStr::new("valid_string.rs");
    println!(
        "os_str_to_str(OsStr::new(\"valid_string.rs\")): {:?}",
        os_str_to_str(os_str)
    );

    // Demonstrate os_string_to_string_lossy
    let os_str = OsStr::new("hello world");
    println!(
        "os_string_to_string_lossy(OsStr::new(\"hello world\")): {}",
        os_string_to_string_lossy(os_str)
    );

    // Demonstrate get_file_extension
    println!("\n=== File Extensions ===");
    let paths = ["document.pdf", "archive.tar.gz", "README", ".gitignore"];
    for path_str in paths {
        let path = Path::new(path_str);
        println!(
            "get_file_extension(Path::new(\"{}\")): {:?}",
            path_str,
            get_file_extension(path)
        );
    }

    // Demonstrate join_path_components
    println!("\n=== Path Joining ===");
    let components = &["home", "user", "documents", "file.txt"];
    let joined = join_path_components(components);
    println!(
        "join_path_components(&{:?}): {:?}",
        components, joined
    );

    // Demonstrate is_valid_utf8
    println!("\n=== UTF-8 Validation ===");
    let os_str = OsStr::new("valid utf8 string");
    println!(
        "is_valid_utf8(OsStr::new(\"valid utf8 string\")): {}",
        is_valid_utf8(os_str)
    );
}
