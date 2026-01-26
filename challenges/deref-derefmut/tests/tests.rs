use deref_derefmut::*;

// ==================== MyBox Tests ====================

#[test]
fn mybox_deref_integer() {
    let b = MyBox::new(42);
    assert_eq!(*b, 42);
}

#[test]
fn mybox_deref_string() {
    let b = MyBox::new(String::from("hello"));
    assert_eq!(&*b, "hello");
}

#[test]
fn mybox_derefmut_integer() {
    let mut b = MyBox::new(42);
    *b = 100;
    assert_eq!(*b, 100);
}

#[test]
fn mybox_derefmut_string() {
    let mut b = MyBox::new(String::from("hello"));
    b.push_str(" world");
    assert_eq!(&*b, "hello world");
}

#[test]
fn mybox_deref_coercion_to_str() {
    let b = MyBox::new(String::from("hello"));
    // Deref coercion: &MyBox<String> -> &String -> &str
    fn takes_str(s: &str) -> usize {
        s.len()
    }
    assert_eq!(takes_str(&*b), 5);
}

#[test]
fn mybox_nested() {
    let b = MyBox::new(MyBox::new(42));
    // Double deref
    assert_eq!(**b, 42);
}

#[test]
fn mybox_with_option() {
    let b = MyBox::new(Some(42));
    assert_eq!(b.unwrap(), 42);
}

#[test]
fn mybox_clone() {
    let b1 = MyBox::new(42);
    let b2 = b1.clone();
    assert_eq!(*b1, *b2);
}

// ==================== CachedValue Tests ====================

#[test]
fn cached_value_initial_count() {
    let cached = CachedValue::new("hello");
    assert_eq!(cached.access_count(), 0);
}

#[test]
fn cached_value_single_access() {
    let cached = CachedValue::new("hello");
    let _ = cached.len();
    assert_eq!(cached.access_count(), 1);
}

#[test]
fn cached_value_multiple_accesses() {
    let cached = CachedValue::new("hello");
    let _ = cached.len();
    let _ = cached.is_empty();
    let _ = cached.starts_with("he");
    assert_eq!(cached.access_count(), 3);
}

#[test]
fn cached_value_with_integer() {
    let cached = CachedValue::new(42i32);
    assert_eq!(*cached, 42);
    assert_eq!(cached.access_count(), 1);
}

#[test]
fn cached_value_preserves_value() {
    let cached = CachedValue::new("hello");
    assert_eq!(*cached, "hello");
    assert_eq!(*cached, "hello");
    assert_eq!(cached.access_count(), 2);
}

#[test]
fn cached_value_with_vec() {
    let cached = CachedValue::new(vec![1, 2, 3]);
    assert_eq!(cached.len(), 3);
    assert_eq!(cached[0], 1);
    assert_eq!(cached.access_count(), 2);
}

#[test]
fn cached_value_many_accesses() {
    let cached = CachedValue::new(100);
    for _ in 0..100 {
        let _ = *cached;
    }
    assert_eq!(cached.access_count(), 100);
}

// ==================== NonEmptyVec Tests ====================

#[test]
fn nonemptyvec_new() {
    let nev = NonEmptyVec::new(1);
    assert_eq!(nev.len(), 1);
}

#[test]
fn nonemptyvec_push() {
    let mut nev = NonEmptyVec::new(1);
    nev.push(2);
    nev.push(3);
    assert_eq!(nev.len(), 3);
}

#[test]
fn nonemptyvec_first_guaranteed() {
    let nev = NonEmptyVec::new(42);
    assert_eq!(*nev.first_guaranteed(), 42);
}

#[test]
fn nonemptyvec_first_guaranteed_mut() {
    let mut nev = NonEmptyVec::new(42);
    *nev.first_guaranteed_mut() = 100;
    assert_eq!(*nev.first_guaranteed(), 100);
}

#[test]
fn nonemptyvec_deref_to_slice() {
    let mut nev = NonEmptyVec::new(1);
    nev.push(2);
    nev.push(3);
    // Using slice methods via Deref
    assert_eq!(nev.first(), Some(&1));
    assert_eq!(nev.last(), Some(&3));
    assert!(!nev.is_empty());
}

#[test]
fn nonemptyvec_derefmut_index() {
    let mut nev = NonEmptyVec::new(1);
    nev.push(2);
    nev[0] = 10;
    nev[1] = 20;
    assert_eq!(nev[0], 10);
    assert_eq!(nev[1], 20);
}

#[test]
fn nonemptyvec_iter() {
    let mut nev = NonEmptyVec::new(1);
    nev.push(2);
    nev.push(3);
    let sum: i32 = nev.iter().sum();
    assert_eq!(sum, 6);
}

#[test]
fn nonemptyvec_contains() {
    let mut nev = NonEmptyVec::new("apple");
    nev.push("banana");
    nev.push("cherry");
    assert!(nev.contains(&"banana"));
    assert!(!nev.contains(&"durian"));
}

#[test]
fn nonemptyvec_sort_via_derefmut() {
    let mut nev = NonEmptyVec::new(3);
    nev.push(1);
    nev.push(2);
    nev.sort();
    assert_eq!(&*nev, &[1, 2, 3]);
}

#[test]
fn nonemptyvec_binary_search() {
    let mut nev = NonEmptyVec::new(1);
    nev.push(3);
    nev.push(5);
    nev.push(7);
    assert_eq!(nev.binary_search(&3), Ok(1));
    assert_eq!(nev.binary_search(&4), Err(2));
}

#[test]
fn nonemptyvec_with_strings() {
    let mut nev = NonEmptyVec::new(String::from("hello"));
    nev.push(String::from("world"));
    assert_eq!(nev.len(), 2);
    assert_eq!(&nev[0], "hello");
}

// ==================== UppercaseString Tests ====================

#[test]
fn uppercase_string_basic() {
    let upper = UppercaseString::new("hello");
    assert_eq!(&*upper, "HELLO");
}

#[test]
fn uppercase_string_mixed_case() {
    let upper = UppercaseString::new("HeLLo WoRLD");
    assert_eq!(&*upper, "HELLO WORLD");
}

#[test]
fn uppercase_string_already_upper() {
    let upper = UppercaseString::new("HELLO");
    assert_eq!(&*upper, "HELLO");
}

#[test]
fn uppercase_string_empty() {
    let upper = UppercaseString::new("");
    assert_eq!(&*upper, "");
}

#[test]
fn uppercase_string_with_numbers() {
    let upper = UppercaseString::new("Hello123World");
    assert_eq!(&*upper, "HELLO123WORLD");
}

#[test]
fn uppercase_string_with_special_chars() {
    let upper = UppercaseString::new("hello!@#$%world");
    assert_eq!(&*upper, "HELLO!@#$%WORLD");
}

#[test]
fn uppercase_string_unicode() {
    let upper = UppercaseString::new("caff");
    // e with acute should uppercase to E with acute
    assert!(upper.starts_with("CAFF"));
}

#[test]
fn uppercase_string_deref_methods() {
    let upper = UppercaseString::new("hello world");
    assert!(upper.starts_with("HELLO"));
    assert!(upper.ends_with("WORLD"));
    assert!(upper.contains("LLO W"));
    assert_eq!(upper.len(), 11);
}

#[test]
fn uppercase_string_into_inner() {
    let upper = UppercaseString::new("hello");
    assert_eq!(upper.into_inner(), "HELLO");
}

#[test]
fn uppercase_string_clone() {
    let upper1 = UppercaseString::new("hello");
    let upper2 = upper1.clone();
    assert_eq!(&*upper1, &*upper2);
}

#[test]
fn uppercase_string_eq() {
    let upper1 = UppercaseString::new("hello");
    let upper2 = UppercaseString::new("HELLO");
    assert_eq!(upper1, upper2);
}

// ==================== describe_length Tests ====================

#[test]
fn describe_length_string() {
    let s = String::from("hello");
    assert_eq!(describe_length(&s), "Length: 5");
}

#[test]
fn describe_length_empty_string() {
    let s = String::from("");
    assert_eq!(describe_length(&s), "Length: 0");
}

#[test]
fn describe_length_long_string() {
    let s = String::from("the quick brown fox jumps over the lazy dog");
    assert_eq!(describe_length(&s), "Length: 43");
}

#[test]
fn describe_length_uppercase_string() {
    let upper = UppercaseString::new("hello");
    assert_eq!(describe_length(&upper), "Length: 5");
}

#[test]
fn describe_length_boxed_str() {
    let boxed: Box<str> = "world".into();
    assert_eq!(describe_length(&boxed), "Length: 5");
}

#[test]
fn describe_length_with_deref() {
    let b = MyBox::new(String::from("test"));
    // We need to deref MyBox<String> to String first, then pass &String
    assert_eq!(describe_length(&*b), "Length: 4");
}

// ==================== Integration Tests ====================

#[test]
fn integration_mybox_string_deref() {
    let b = MyBox::new(String::from("integration test"));
    // Explicitly deref to String, which implements Deref<Target = str>
    assert_eq!(describe_length(&*b), "Length: 16");
}

#[test]
fn integration_nested_deref() {
    let inner = MyBox::new(42);
    let outer = MyBox::new(inner);
    assert_eq!(**outer, 42);
}

#[test]
fn integration_cached_uppercase() {
    let cached = CachedValue::new(UppercaseString::new("hello"));
    // Access via deref chain: CachedValue -> UppercaseString -> str
    assert!(cached.starts_with("HELLO"));
    assert_eq!(cached.access_count(), 1);
}

#[test]
fn integration_nonemptyvec_of_uppercase() {
    let mut nev = NonEmptyVec::new(UppercaseString::new("hello"));
    nev.push(UppercaseString::new("world"));
    assert_eq!(nev.len(), 2);
    assert_eq!(&*nev[0], "HELLO");
    assert_eq!(&*nev[1], "WORLD");
}

#[test]
fn integration_deref_coercion_chain() {
    // This tests the deref coercion chain working correctly
    let b = MyBox::new(String::from("hello"));

    // Function that takes &str
    fn process_str(s: &str) -> String {
        s.to_uppercase()
    }

    // Should work via deref coercion: &MyBox<String> -> &String -> &str
    assert_eq!(process_str(&*b), "HELLO");
}

#[test]
fn integration_cached_value_with_mybox() {
    let cached = CachedValue::new(MyBox::new(vec![1, 2, 3]));
    // Deref through CachedValue to MyBox to Vec
    assert_eq!(cached.len(), 3);
    assert_eq!(cached.access_count(), 1);
}

#[test]
fn integration_modify_nonemptyvec_contents() {
    let mut nev = NonEmptyVec::new(vec![1, 2, 3]);
    nev.push(vec![4, 5, 6]);

    // Modify the inner vec through DerefMut
    nev[0].push(10);
    assert_eq!(nev[0], vec![1, 2, 3, 10]);
}

#[test]
fn integration_all_types_with_describe_length() {
    let string = String::from("one");
    let upper = UppercaseString::new("two");
    let boxed: Box<str> = "three".into();
    let mybox = MyBox::new(String::from("four"));

    assert_eq!(describe_length(&string), "Length: 3");
    assert_eq!(describe_length(&upper), "Length: 3");
    assert_eq!(describe_length(&boxed), "Length: 5");
    // MyBox<String> derefs to String, so we need to deref first
    assert_eq!(describe_length(&*mybox), "Length: 4");
}
