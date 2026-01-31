When working with file paths and system operations in Rust, you'll encounter `OsString` and `OsStr` - types designed to handle platform-specific string representations. While Rust's `String` and `&str` are guaranteed to be valid UTF-8, operating systems don't make such guarantees. Windows uses UTF-16, Unix systems allow arbitrary byte sequences in paths, and different platforms have different conventions.

The `std::ffi` module provides `OsString` (owned) and `OsStr` (borrowed) to bridge this gap. These types can represent any valid string on the host platform, even if it's not valid UTF-8. This is crucial for correctly handling file paths that might contain unusual characters.

## Key Concepts

1. **`OsString`** - An owned, mutable platform-native string (like `String` for UTF-8)
2. **`OsStr`** - A borrowed reference to a platform-native string (like `&str` for UTF-8)
3. **Conversions** - Converting between `String`/`&str` and `OsString`/`OsStr`
4. **Path integration** - `Path` and `PathBuf` internally use `OsStr` and `OsString`

```rust
use std::ffi::{OsString, OsStr};
use std::path::Path;

// Creating OsString from &str
let os_string = OsString::from("hello.txt");

// OsStr from &str
let os_str: &OsStr = OsStr::new("hello.txt");

// Path uses OsStr internally
let path = Path::new("hello.txt");
let file_name: Option<&OsStr> = path.file_name();

// Converting OsStr to &str (may fail if not valid UTF-8)
let os_str: &OsStr = OsStr::new("hello.txt");
// Returns Some if valid UTF-8
let s: Option<&str> = os_str.to_str();

// Converting OsStr to String (lossy conversion)
// Replaces invalid UTF-8 with replacement character
let s: String = os_str
    .to_string_lossy()
    .into_owned();
```

## Your Task

Implement the following functions for working with `OsString` and `OsStr`:

1. `to_os_string(s: &str) -> OsString` - Convert a `&str` to an `OsString`
2. `os_str_to_str(os: &OsStr) -> Option<&str>` - Try to convert an `OsStr` to `&str` (returns `None` if not valid UTF-8)
3. `os_string_to_string_lossy(os: &OsStr) -> String` - Convert `OsStr` to `String`, replacing invalid UTF-8 with the replacement character
4. `get_file_extension(path: &Path) -> Option<String>` - Extract the file extension from a path as a `String` (returns `None` if no extension or if extension isn't valid UTF-8)
5. `join_path_components(components: &[&str]) -> OsString` - Join multiple path components into a single `OsString` path
6. `is_valid_utf8(os: &OsStr) -> bool` - Check if an `OsStr` contains valid UTF-8

## Examples

```rust
use std::ffi::{OsString, OsStr};
use std::path::Path;

// to_os_string
let os = to_os_string("hello.txt");
assert_eq!(os, OsString::from("hello.txt"));

// os_str_to_str
let os_str = OsStr::new("hello.txt");
assert_eq!(os_str_to_str(os_str), Some("hello.txt"));

// os_string_to_string_lossy
let os_str = OsStr::new("valid utf8");
assert_eq!(os_string_to_string_lossy(os_str), "valid utf8");

// get_file_extension
let path = Path::new("document.pdf");
assert_eq!(
    get_file_extension(path),
    Some("pdf".to_string())
);

let path = Path::new("no_extension");
assert_eq!(get_file_extension(path), None);

// join_path_components
let path = join_path_components(&["home", "user", "documents"]);
// On Unix: "home/user/documents"
// On Windows: "home\\user\\documents"

// is_valid_utf8
let os_str = OsStr::new("hello");
assert!(is_valid_utf8(os_str));
```

## Hints

<details>
  <summary>Click here for hints</summary>

- For `to_os_string`, use `OsString::from()` or `.into()`
- For `os_str_to_str`, use the `.to_str()` method on `OsStr`
- For `os_string_to_string_lossy`, use `.to_string_lossy()` which returns a `Cow<str>`, then convert to owned `String`
- For `get_file_extension`, use `Path::extension()` which returns `Option<&OsStr>`, then convert to `String` using `to_str()`
- For `join_path_components`, you can use `PathBuf` and its `.push()` method, then convert to `OsString` with `.into_os_string()`
- For `is_valid_utf8`, simply check if `.to_str()` returns `Some`

</details>
