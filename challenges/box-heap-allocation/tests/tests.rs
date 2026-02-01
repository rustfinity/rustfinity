use box_heap_allocation::*;

// ==================== boxed_value tests ====================

#[test]
fn boxed_value_integer() {
    let boxed = boxed_value(42);
    assert_eq!(*boxed, 42);
}

#[test]
fn boxed_value_zero() {
    let boxed = boxed_value(0);
    assert_eq!(*boxed, 0);
}

#[test]
fn boxed_value_negative() {
    let boxed = boxed_value(-100);
    assert_eq!(*boxed, -100);
}

#[test]
fn boxed_value_string() {
    let boxed = boxed_value(String::from("hello"));
    assert_eq!(&*boxed, "hello");
}

#[test]
fn boxed_value_empty_string() {
    let boxed = boxed_value(String::new());
    assert_eq!(&*boxed, "");
}

#[test]
fn boxed_value_float() {
    let boxed: Box<f64> = boxed_value(3.14);
    assert!((*boxed - 3.14).abs() < f64::EPSILON);
}

#[test]
fn boxed_value_bool() {
    let boxed_true = boxed_value(true);
    let boxed_false = boxed_value(false);
    assert!(*boxed_true);
    assert!(!*boxed_false);
}

#[test]
fn boxed_value_tuple() {
    let boxed = boxed_value((1, "two", 3.0));
    assert_eq!(*boxed, (1, "two", 3.0));
}

#[test]
fn boxed_value_vec() {
    let boxed = boxed_value(vec![1, 2, 3]);
    assert_eq!(*boxed, vec![1, 2, 3]);
}

#[test]
fn boxed_value_option() {
    let some = boxed_value(Some(42));
    let none: Box<Option<i32>> = boxed_value(None);
    assert_eq!(*some, Some(42));
    assert_eq!(*none, None);
}

// ==================== unbox tests ====================

#[test]
fn unbox_integer() {
    let boxed = Box::new(42);
    assert_eq!(unbox(boxed), 42);
}

#[test]
fn unbox_zero() {
    let boxed = Box::new(0);
    assert_eq!(unbox(boxed), 0);
}

#[test]
fn unbox_string() {
    let boxed = Box::new(String::from("hello"));
    assert_eq!(unbox(boxed), "hello");
}

#[test]
fn unbox_vec() {
    let boxed = Box::new(vec![1, 2, 3]);
    assert_eq!(unbox(boxed), vec![1, 2, 3]);
}

#[test]
fn unbox_nested_box() {
    let inner = Box::new(42);
    let outer = Box::new(inner);
    let unboxed_outer = unbox(outer);
    let unboxed_inner = unbox(unboxed_outer);
    assert_eq!(unboxed_inner, 42);
}

#[test]
fn boxed_then_unbox_roundtrip() {
    let original = 12345;
    let boxed = boxed_value(original);
    let unboxed = unbox(boxed);
    assert_eq!(unboxed, original);
}

// ==================== List::new tests ====================

#[test]
fn list_new_empty() {
    let list = List::new();
    assert!(list.is_empty());
}

#[test]
fn list_new_len_zero() {
    let list = List::new();
    assert_eq!(list.len(), 0);
}

#[test]
fn list_new_sum_zero() {
    let list = List::new();
    assert_eq!(list.sum(), 0);
}

#[test]
fn list_new_to_vec_empty() {
    let list = List::new();
    assert_eq!(list.to_vec(), Vec::<i32>::new());
}

#[test]
fn list_default() {
    let list = List::default();
    assert!(list.is_empty());
}

// ==================== List::prepend tests ====================

#[test]
fn list_prepend_single() {
    let list = List::new().prepend(1);
    assert_eq!(list.len(), 1);
    assert_eq!(list.to_vec(), vec![1]);
}

#[test]
fn list_prepend_multiple() {
    let list = List::new().prepend(3).prepend(2).prepend(1);
    assert_eq!(list.len(), 3);
    assert_eq!(list.to_vec(), vec![1, 2, 3]);
}

#[test]
fn list_prepend_zero() {
    let list = List::new().prepend(0);
    assert_eq!(list.len(), 1);
    assert_eq!(list.to_vec(), vec![0]);
}

#[test]
fn list_prepend_negative() {
    let list = List::new().prepend(-3).prepend(-2).prepend(-1);
    assert_eq!(list.to_vec(), vec![-1, -2, -3]);
}

#[test]
fn list_prepend_mixed() {
    let list = List::new().prepend(-1).prepend(0).prepend(1);
    assert_eq!(list.to_vec(), vec![1, 0, -1]);
}

#[test]
fn list_prepend_duplicates() {
    let list = List::new().prepend(1).prepend(1).prepend(1);
    assert_eq!(list.to_vec(), vec![1, 1, 1]);
}

// ==================== List::len tests ====================

#[test]
fn list_len_zero() {
    assert_eq!(List::new().len(), 0);
}

#[test]
fn list_len_one() {
    assert_eq!(List::new().prepend(1).len(), 1);
}

#[test]
fn list_len_many() {
    let list = List::new()
        .prepend(1)
        .prepend(2)
        .prepend(3)
        .prepend(4)
        .prepend(5);
    assert_eq!(list.len(), 5);
}

// ==================== List::is_empty tests ====================

#[test]
fn list_is_empty_true() {
    assert!(List::new().is_empty());
}

#[test]
fn list_is_empty_false() {
    assert!(!List::new().prepend(1).is_empty());
}

// ==================== List::sum tests ====================

#[test]
fn list_sum_empty() {
    assert_eq!(List::new().sum(), 0);
}

#[test]
fn list_sum_single() {
    assert_eq!(List::new().prepend(42).sum(), 42);
}

#[test]
fn list_sum_multiple() {
    let list = List::new().prepend(3).prepend(2).prepend(1);
    assert_eq!(list.sum(), 6);
}

#[test]
fn list_sum_negative() {
    let list = List::new().prepend(-3).prepend(-2).prepend(-1);
    assert_eq!(list.sum(), -6);
}

#[test]
fn list_sum_mixed() {
    let list = List::new().prepend(-5).prepend(10).prepend(-5);
    assert_eq!(list.sum(), 0);
}

#[test]
fn list_sum_zeros() {
    let list = List::new().prepend(0).prepend(0).prepend(0);
    assert_eq!(list.sum(), 0);
}

#[test]
fn list_sum_large() {
    let list = List::new()
        .prepend(1000)
        .prepend(2000)
        .prepend(3000)
        .prepend(4000);
    assert_eq!(list.sum(), 10000);
}

// ==================== List::to_vec tests ====================

#[test]
fn list_to_vec_empty() {
    assert_eq!(List::new().to_vec(), Vec::<i32>::new());
}

#[test]
fn list_to_vec_single() {
    assert_eq!(List::new().prepend(42).to_vec(), vec![42]);
}

#[test]
fn list_to_vec_preserves_order() {
    let list = List::new().prepend(3).prepend(2).prepend(1);
    // Prepending 1 last means it's at the front
    assert_eq!(list.to_vec(), vec![1, 2, 3]);
}

#[test]
fn list_to_vec_with_negatives() {
    let list = List::new().prepend(3).prepend(-2).prepend(1);
    assert_eq!(list.to_vec(), vec![1, -2, 3]);
}

// ==================== List clone and equality tests ====================

#[test]
fn list_clone() {
    let list = List::new().prepend(3).prepend(2).prepend(1);
    let cloned = list.clone();
    assert_eq!(list.to_vec(), cloned.to_vec());
}

#[test]
fn list_eq() {
    let list1 = List::new().prepend(2).prepend(1);
    let list2 = List::new().prepend(2).prepend(1);
    assert_eq!(list1, list2);
}

#[test]
fn list_ne() {
    let list1 = List::new().prepend(1);
    let list2 = List::new().prepend(2);
    assert_ne!(list1, list2);
}

// ==================== LargeData tests ====================

#[test]
fn large_data_new() {
    let data = LargeData::new();
    assert_eq!(data.data[0], 0);
    assert_eq!(data.data[999], 0);
}

#[test]
fn large_data_filled() {
    let data = LargeData::filled(42);
    assert_eq!(data.data[0], 42);
    assert_eq!(data.data[500], 42);
    assert_eq!(data.data[999], 42);
}

#[test]
fn large_data_default() {
    let data = LargeData::default();
    assert_eq!(data.data[0], 0);
}

#[test]
fn large_data_size() {
    let data = LargeData::new();
    assert_eq!(data.data.len(), 1000);
}

// ==================== box_large_data tests ====================

#[test]
fn box_large_data_basic() {
    let large = LargeData::new();
    let boxed = box_large_data(large);
    assert_eq!(boxed.data[0], 0);
}

#[test]
fn box_large_data_filled() {
    let large = LargeData::filled(255);
    let boxed = box_large_data(large);
    assert_eq!(boxed.data[0], 255);
    assert_eq!(boxed.data[999], 255);
}

#[test]
fn box_large_data_modify_via_deref() {
    let large = LargeData::new();
    let mut boxed = box_large_data(large);
    boxed.data[0] = 100;
    assert_eq!(boxed.data[0], 100);
}

#[test]
fn box_large_data_clone() {
    let large = LargeData::filled(42);
    let boxed = box_large_data(large);
    let cloned = boxed.clone();
    assert_eq!(boxed.data, cloned.data);
}

// ==================== modify_boxed tests ====================

#[test]
fn modify_boxed_integer_add() {
    let mut boxed = Box::new(10);
    modify_boxed(&mut boxed, |n| *n += 5);
    assert_eq!(*boxed, 15);
}

#[test]
fn modify_boxed_integer_multiply() {
    let mut boxed = Box::new(7);
    modify_boxed(&mut boxed, |n| *n *= 3);
    assert_eq!(*boxed, 21);
}

#[test]
fn modify_boxed_integer_set() {
    let mut boxed = Box::new(0);
    modify_boxed(&mut boxed, |n| *n = 100);
    assert_eq!(*boxed, 100);
}

#[test]
fn modify_boxed_string_push() {
    let mut boxed = Box::new(String::from("hello"));
    modify_boxed(&mut boxed, |s| s.push_str(" world"));
    assert_eq!(&*boxed, "hello world");
}

#[test]
fn modify_boxed_string_clear() {
    let mut boxed = Box::new(String::from("hello"));
    modify_boxed(&mut boxed, |s| s.clear());
    assert_eq!(&*boxed, "");
}

#[test]
fn modify_boxed_vec_push() {
    let mut boxed = Box::new(vec![1, 2, 3]);
    modify_boxed(&mut boxed, |v| v.push(4));
    assert_eq!(*boxed, vec![1, 2, 3, 4]);
}

#[test]
fn modify_boxed_vec_clear() {
    let mut boxed = Box::new(vec![1, 2, 3]);
    modify_boxed(&mut boxed, |v| v.clear());
    assert!(boxed.is_empty());
}

#[test]
fn modify_boxed_vec_sort() {
    let mut boxed = Box::new(vec![3, 1, 2]);
    modify_boxed(&mut boxed, |v| v.sort());
    assert_eq!(*boxed, vec![1, 2, 3]);
}

#[test]
fn modify_boxed_struct() {
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    let mut boxed = Box::new(Point { x: 0, y: 0 });
    modify_boxed(&mut boxed, |p| {
        p.x = 10;
        p.y = 20;
    });
    assert_eq!(*boxed, Point { x: 10, y: 20 });
}

#[test]
fn modify_boxed_large_data() {
    let large = LargeData::new();
    let mut boxed = box_large_data(large);
    modify_boxed(&mut boxed, |d| {
        d.data[0] = 1;
        d.data[999] = 2;
    });
    assert_eq!(boxed.data[0], 1);
    assert_eq!(boxed.data[999], 2);
}

// ==================== Integration tests ====================

#[test]
fn integration_box_unbox_list() {
    let list = List::new().prepend(3).prepend(2).prepend(1);
    let boxed = boxed_value(list);
    assert_eq!(boxed.len(), 3);
    let unboxed = unbox(boxed);
    assert_eq!(unboxed.sum(), 6);
}

#[test]
fn integration_nested_boxes() {
    let inner = boxed_value(42);
    let outer = boxed_value(inner);
    assert_eq!(**outer, 42);
}

#[test]
fn integration_box_and_modify() {
    let mut boxed = boxed_value(vec![1, 2, 3]);
    modify_boxed(&mut boxed, |v| {
        v.push(4);
        v.push(5);
    });
    assert_eq!(*boxed, vec![1, 2, 3, 4, 5]);
}

#[test]
fn integration_list_workflow() {
    // Build a list
    let list = List::new()
        .prepend(5)
        .prepend(4)
        .prepend(3)
        .prepend(2)
        .prepend(1);

    // Check properties
    assert!(!list.is_empty());
    assert_eq!(list.len(), 5);
    assert_eq!(list.sum(), 15);
    assert_eq!(list.to_vec(), vec![1, 2, 3, 4, 5]);

    // Clone and verify independence
    let cloned = list.clone();
    assert_eq!(list, cloned);
}

#[test]
fn integration_large_data_workflow() {
    // Create and box large data
    let large = LargeData::filled(10);
    let mut boxed = box_large_data(large);

    // Modify via closure
    modify_boxed(&mut boxed, |d| {
        for i in 0..100 {
            d.data[i] = 20;
        }
    });

    // Verify
    assert_eq!(boxed.data[0], 20);
    assert_eq!(boxed.data[99], 20);
    assert_eq!(boxed.data[100], 10);
}

#[test]
fn integration_generic_types() {
    // Test boxed_value with various types
    let boxed_int = boxed_value(42i32);
    let boxed_float = boxed_value(3.14f64);
    let boxed_string = boxed_value(String::from("test"));
    let boxed_option: Box<Option<i32>> = boxed_value(Some(1));

    assert_eq!(unbox(boxed_int), 42);
    assert!((unbox(boxed_float) - 3.14).abs() < f64::EPSILON);
    assert_eq!(unbox(boxed_string), "test");
    assert_eq!(unbox(boxed_option), Some(1));
}

#[test]
fn integration_recursive_list_deep() {
    // Create a longer list to test recursion
    let mut list = List::new();
    for i in (1..=10).rev() {
        list = list.prepend(i);
    }
    assert_eq!(list.len(), 10);
    assert_eq!(list.sum(), 55); // 1+2+...+10 = 55
    assert_eq!(list.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn integration_modify_multiple_times() {
    let mut boxed = Box::new(0);
    for i in 1..=5 {
        modify_boxed(&mut boxed, |n| *n += i);
    }
    assert_eq!(*boxed, 15); // 1+2+3+4+5
}

#[test]
fn integration_box_of_boxes() {
    // Box containing a Box containing a value
    let inner = Box::new(42);
    let middle = Box::new(inner);
    let outer = Box::new(middle);

    assert_eq!(***outer, 42);
}
