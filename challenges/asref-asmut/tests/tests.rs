use asref_asmut::*;
use std::path::PathBuf;

// ============================================================================
// string_length tests
// ============================================================================

#[test]
fn test_string_length_str_slice() {
    assert_eq!(string_length("hello"), 5);
}

#[test]
fn test_string_length_string() {
    assert_eq!(string_length(String::from("world")), 5);
}

#[test]
fn test_string_length_empty() {
    assert_eq!(string_length(""), 0);
    assert_eq!(string_length(String::new()), 0);
}

#[test]
fn test_string_length_unicode() {
    assert_eq!(string_length("héllo"), 6); // é is 2 bytes
    assert_eq!(string_length("日本語"), 9); // 3 chars, 3 bytes each
}

#[test]
fn test_string_length_with_reference() {
    let s = String::from("test");
    assert_eq!(string_length(&s), 4);
}

// ============================================================================
// slice_sum tests
// ============================================================================

#[test]
fn test_slice_sum_array() {
    assert_eq!(slice_sum(&[1, 2, 3, 4]), 10);
}

#[test]
fn test_slice_sum_vec() {
    assert_eq!(slice_sum(vec![10, 20, 30]), 60);
}

#[test]
fn test_slice_sum_empty() {
    assert_eq!(slice_sum(&[] as &[i32]), 0);
    assert_eq!(slice_sum(Vec::<i32>::new()), 0);
}

#[test]
fn test_slice_sum_single_element() {
    assert_eq!(slice_sum(&[42]), 42);
    assert_eq!(slice_sum(vec![100]), 100);
}

#[test]
fn test_slice_sum_negative_numbers() {
    assert_eq!(slice_sum(&[-1, -2, -3]), -6);
    assert_eq!(slice_sum(vec![-10, 20, -5]), 5);
}

#[test]
fn test_slice_sum_with_reference() {
    let v = vec![1, 2, 3];
    assert_eq!(slice_sum(&v), 6);
}

// ============================================================================
// contains_element tests
// ============================================================================

#[test]
fn test_contains_element_found_in_array() {
    assert!(contains_element(&[1, 2, 3], 2));
}

#[test]
fn test_contains_element_found_in_vec() {
    assert!(contains_element(vec![10, 20, 30], 20));
}

#[test]
fn test_contains_element_not_found() {
    assert!(!contains_element(&[1, 2, 3], 5));
    assert!(!contains_element(vec![1, 2, 3], 0));
}

#[test]
fn test_contains_element_empty() {
    assert!(!contains_element(&[] as &[i32], 1));
    assert!(!contains_element(Vec::<i32>::new(), 1));
}

#[test]
fn test_contains_element_first_last() {
    assert!(contains_element(&[1, 2, 3], 1));
    assert!(contains_element(&[1, 2, 3], 3));
}

#[test]
fn test_contains_element_negative() {
    assert!(contains_element(&[-1, 0, 1], -1));
    assert!(!contains_element(&[1, 2, 3], -1));
}

// ============================================================================
// double_all tests
// ============================================================================

#[test]
fn test_double_all_array() {
    let mut arr = [1, 2, 3];
    double_all(&mut arr);
    assert_eq!(arr, [2, 4, 6]);
}

#[test]
fn test_double_all_vec() {
    let mut vec = vec![5, 10, 15];
    double_all(&mut vec);
    assert_eq!(vec, vec![10, 20, 30]);
}

#[test]
fn test_double_all_empty() {
    let mut arr: [i32; 0] = [];
    double_all(&mut arr);
    assert_eq!(arr, []);

    let mut vec: Vec<i32> = vec![];
    double_all(&mut vec);
    assert!(vec.is_empty());
}

#[test]
fn test_double_all_single_element() {
    let mut arr = [7];
    double_all(&mut arr);
    assert_eq!(arr, [14]);
}

#[test]
fn test_double_all_negative_numbers() {
    let mut vec = vec![-3, -2, -1];
    double_all(&mut vec);
    assert_eq!(vec, vec![-6, -4, -2]);
}

#[test]
fn test_double_all_mixed() {
    let mut vec = vec![-5, 0, 5];
    double_all(&mut vec);
    assert_eq!(vec, vec![-10, 0, 10]);
}

// ============================================================================
// Text type tests
// ============================================================================

#[test]
fn test_text_new_from_str() {
    let text = Text::new("hello");
    assert_eq!(text.into_inner(), "hello");
}

#[test]
fn test_text_new_from_string() {
    let text = Text::new(String::from("world"));
    assert_eq!(text.into_inner(), "world");
}

#[test]
fn test_text_asref_str() {
    let text = Text::new("hello");
    let s: &str = text.as_ref();
    assert_eq!(s, "hello");
}

#[test]
fn test_text_asref_bytes() {
    let text = Text::new("hello");
    let bytes: &[u8] = text.as_ref();
    assert_eq!(bytes, b"hello");
}

#[test]
fn test_text_asmut_string() {
    let mut text = Text::new("hello");
    {
        let s: &mut String = text.as_mut();
        s.push_str(" world");
    }
    assert_eq!(text.into_inner(), "hello world");
}

#[test]
fn test_text_empty() {
    let text = Text::new("");
    let s: &str = text.as_ref();
    assert_eq!(s, "");

    let text2 = Text::new("");
    let bytes: &[u8] = text2.as_ref();
    assert!(bytes.is_empty());
}

#[test]
fn test_text_unicode() {
    let text = Text::new("日本語");
    let s: &str = text.as_ref();
    assert_eq!(s, "日本語");

    let text2 = Text::new("日本語");
    let bytes: &[u8] = text2.as_ref();
    assert_eq!(bytes.len(), 9); // 3 chars * 3 bytes each
}

#[test]
fn test_text_clone_and_eq() {
    let text1 = Text::new("test");
    let text2 = text1.clone();
    assert_eq!(text1, text2);
}

// ============================================================================
// print_as_uppercase tests
// ============================================================================

#[test]
fn test_print_as_uppercase_str() {
    assert_eq!(print_as_uppercase("hello"), "HELLO");
}

#[test]
fn test_print_as_uppercase_string() {
    assert_eq!(print_as_uppercase(String::from("world")), "WORLD");
}

#[test]
fn test_print_as_uppercase_mixed_case() {
    assert_eq!(print_as_uppercase("HeLLo WoRLd"), "HELLO WORLD");
}

#[test]
fn test_print_as_uppercase_already_upper() {
    assert_eq!(print_as_uppercase("ALREADY UPPER"), "ALREADY UPPER");
}

#[test]
fn test_print_as_uppercase_empty() {
    assert_eq!(print_as_uppercase(""), "");
}

#[test]
fn test_print_as_uppercase_numbers_and_special() {
    assert_eq!(print_as_uppercase("hello123!@#"), "HELLO123!@#");
}

#[test]
fn test_print_as_uppercase_unicode() {
    // Some unicode characters have uppercase forms
    assert_eq!(print_as_uppercase("café"), "CAFÉ");
}

// ============================================================================
// append_value tests
// ============================================================================

#[test]
fn test_append_value_basic() {
    let mut vec = vec![1, 2];
    append_value(&mut vec, 3);
    assert_eq!(vec, vec![1, 2, 3]);
}

#[test]
fn test_append_value_empty() {
    let mut vec: Vec<i32> = vec![];
    append_value(&mut vec, 42);
    assert_eq!(vec, vec![42]);
}

#[test]
fn test_append_value_multiple() {
    let mut vec = vec![1];
    append_value(&mut vec, 2);
    append_value(&mut vec, 3);
    append_value(&mut vec, 4);
    assert_eq!(vec, vec![1, 2, 3, 4]);
}

#[test]
fn test_append_value_negative() {
    let mut vec = vec![1, 2, 3];
    append_value(&mut vec, -1);
    assert_eq!(vec, vec![1, 2, 3, -1]);
}

// ============================================================================
// get_extension tests
// ============================================================================

#[test]
fn test_get_extension_str() {
    assert_eq!(get_extension("file.txt"), Some("txt".to_string()));
}

#[test]
fn test_get_extension_string() {
    assert_eq!(
        get_extension(String::from("document.pdf")),
        Some("pdf".to_string())
    );
}

#[test]
fn test_get_extension_pathbuf() {
    assert_eq!(
        get_extension(PathBuf::from("image.png")),
        Some("png".to_string())
    );
}

#[test]
fn test_get_extension_no_extension() {
    assert_eq!(get_extension("no_extension"), None);
}

#[test]
fn test_get_extension_dotfile() {
    // .gitignore has no extension, the dot is part of the name
    assert_eq!(get_extension(".gitignore"), None);
}

#[test]
fn test_get_extension_multiple_dots() {
    assert_eq!(
        get_extension("archive.tar.gz"),
        Some("gz".to_string())
    );
}

#[test]
fn test_get_extension_empty_extension() {
    // A file ending with a dot has an empty extension
    assert_eq!(get_extension("file."), Some("".to_string()));
}

#[test]
fn test_get_extension_with_path() {
    assert_eq!(
        get_extension("/home/user/documents/file.txt"),
        Some("txt".to_string())
    );
    assert_eq!(
        get_extension("./relative/path/script.rs"),
        Some("rs".to_string())
    );
}

// ============================================================================
// Integration tests
// ============================================================================

#[test]
fn test_integration_text_with_string_length() {
    let text = Text::new("integration test");
    // Text implements AsRef<str>, so it should work with string_length
    let s: &str = text.as_ref();
    assert_eq!(string_length(s), 16);
}

#[test]
fn test_integration_chained_operations() {
    // Chain multiple operations using AsRef/AsMut
    let mut vec = vec![1, 2, 3, 4, 5];

    // Check sum using AsRef
    let initial_sum = slice_sum(&vec);
    assert_eq!(initial_sum, 15);

    // Double all using AsMut
    double_all(&mut vec);
    assert_eq!(vec, vec![2, 4, 6, 8, 10]);

    // Check new sum
    let new_sum = slice_sum(&vec);
    assert_eq!(new_sum, 30);
}

#[test]
fn test_integration_text_modification() {
    let mut text = Text::new("hello");

    // Check initial state via AsRef<str>
    let s: &str = text.as_ref();
    assert_eq!(s.len(), 5);

    // Modify via AsMut<String>
    text.as_mut().push_str(" world");

    // Check bytes via AsRef<[u8]>
    let bytes: &[u8] = text.as_ref();
    assert_eq!(bytes, b"hello world");
}

#[test]
fn test_integration_generic_function_flexibility() {
    // Demonstrate that our generic functions work with various types

    // string_length works with &str, String, and references
    let s1 = "hello";
    let s2 = String::from("hello");
    let s3 = &s2;

    assert_eq!(string_length(s1), string_length(s2.clone()));
    assert_eq!(string_length(s2.clone()), string_length(s3));

    // slice_sum works with arrays, vecs, and slices
    let arr = [1, 2, 3];
    let vec = vec![1, 2, 3];
    let slice: &[i32] = &arr;

    assert_eq!(slice_sum(&arr), slice_sum(&vec));
    assert_eq!(slice_sum(&vec), slice_sum(slice));
}

#[test]
fn test_integration_path_various_types() {
    use std::path::Path;

    // All these types implement AsRef<Path>
    let ext1 = get_extension("file.txt");
    let ext2 = get_extension(String::from("file.txt"));
    let ext3 = get_extension(PathBuf::from("file.txt"));
    let ext4 = get_extension(Path::new("file.txt"));

    assert_eq!(ext1, ext2);
    assert_eq!(ext2, ext3);
    assert_eq!(ext3, ext4);
}

#[test]
fn test_integration_append_and_sum() {
    let mut vec = vec![];

    for i in 1..=5 {
        append_value(&mut vec, i);
    }

    assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    assert_eq!(slice_sum(&vec), 15);
}

#[test]
fn test_integration_contains_after_double() {
    let mut vec = vec![1, 3, 5];

    // Check original contents
    assert!(contains_element(&vec, 3));
    assert!(!contains_element(&vec, 6));

    // Double all values: [1, 3, 5] -> [2, 6, 10]
    double_all(&mut vec);

    // Now 3 -> 6, so check new contents
    assert!(!contains_element(&vec, 3));
    assert!(contains_element(&vec, 6));
}
