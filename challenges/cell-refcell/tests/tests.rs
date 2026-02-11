use cell_refcell::*;
use std::rc::Rc;

// =============================================================================
// Part 1: Counter Tests
// =============================================================================

#[test]
fn test_counter_new() {
    let counter = Counter::new();
    assert_eq!(counter.get(), 0);
}

#[test]
fn test_counter_increment() {
    let counter = Counter::new();
    counter.increment();
    assert_eq!(counter.get(), 1);
    counter.increment();
    assert_eq!(counter.get(), 2);
}

#[test]
fn test_counter_decrement() {
    let counter = Counter::new();
    counter.set(5);
    counter.decrement();
    assert_eq!(counter.get(), 4);
    counter.decrement();
    assert_eq!(counter.get(), 3);
}

#[test]
fn test_counter_set() {
    let counter = Counter::new();
    counter.set(100);
    assert_eq!(counter.get(), 100);
    counter.set(-50);
    assert_eq!(counter.get(), -50);
}

#[test]
fn test_counter_negative() {
    let counter = Counter::new();
    counter.decrement();
    assert_eq!(counter.get(), -1);
    counter.decrement();
    assert_eq!(counter.get(), -2);
}

#[test]
fn test_counter_mixed_operations() {
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    counter.decrement();
    counter.increment();
    assert_eq!(counter.get(), 2);
}

#[test]
fn test_counter_default() {
    let counter = Counter::default();
    assert_eq!(counter.get(), 0);
}

// =============================================================================
// Part 1: CachedValue Tests
// =============================================================================

#[test]
fn test_cached_value_new() {
    let cached = CachedValue::new(42);
    assert_eq!(cached.access_count(), 0);
}

#[test]
fn test_cached_value_get() {
    let cached = CachedValue::new(42);
    assert_eq!(cached.get(), 42);
    assert_eq!(cached.access_count(), 1);
}

#[test]
fn test_cached_value_multiple_gets() {
    let cached = CachedValue::new(100);
    for _ in 0..5 {
        cached.get();
    }
    assert_eq!(cached.access_count(), 5);
}

#[test]
fn test_cached_value_set_resets_count() {
    let cached = CachedValue::new(42);
    cached.get();
    cached.get();
    assert_eq!(cached.access_count(), 2);
    cached.set(100);
    assert_eq!(cached.access_count(), 0);
    assert_eq!(cached.get(), 100);
    assert_eq!(cached.access_count(), 1);
}

#[test]
fn test_cached_value_float() {
    let cached = CachedValue::new(3.14f64);
    assert!((cached.get() - 3.14).abs() < 0.001);
    assert_eq!(cached.access_count(), 1);
}

#[test]
fn test_cached_value_bool() {
    let cached = CachedValue::new(true);
    assert!(cached.get());
    cached.set(false);
    assert!(!cached.get());
}

#[test]
fn test_cached_value_char() {
    let cached = CachedValue::new('a');
    assert_eq!(cached.get(), 'a');
    cached.set('z');
    assert_eq!(cached.get(), 'z');
}

#[test]
fn test_cached_value_default() {
    let cached: CachedValue<i32> = CachedValue::default();
    assert_eq!(cached.get(), 0);
}

// =============================================================================
// Part 2: SharedString Tests
// =============================================================================

#[test]
fn test_shared_string_new() {
    let shared = SharedString::new("hello");
    assert_eq!(shared.get(), "hello");
}

#[test]
fn test_shared_string_empty() {
    let shared = SharedString::new("");
    assert!(shared.is_empty());
    assert_eq!(shared.len(), 0);
}

#[test]
fn test_shared_string_set() {
    let shared = SharedString::new("old");
    shared.set("new");
    assert_eq!(shared.get(), "new");
}

#[test]
fn test_shared_string_append() {
    let shared = SharedString::new("Hello");
    shared.append(", World!");
    assert_eq!(shared.get(), "Hello, World!");
}

#[test]
fn test_shared_string_multiple_appends() {
    let shared = SharedString::new("");
    shared.append("a");
    shared.append("b");
    shared.append("c");
    assert_eq!(shared.get(), "abc");
}

#[test]
fn test_shared_string_len() {
    let shared = SharedString::new("hello");
    assert_eq!(shared.len(), 5);
    shared.append(" world");
    assert_eq!(shared.len(), 11);
}

#[test]
fn test_shared_string_is_empty() {
    let shared = SharedString::new("");
    assert!(shared.is_empty());
    shared.append("x");
    assert!(!shared.is_empty());
}

#[test]
fn test_shared_string_unicode() {
    let shared = SharedString::new("hello ");
    shared.append("世界");
    assert_eq!(shared.get(), "hello 世界");
}

#[test]
fn test_shared_string_set_after_append() {
    let shared = SharedString::new("Hello");
    shared.append(" World");
    shared.set("Reset");
    assert_eq!(shared.get(), "Reset");
}

#[test]
fn test_shared_string_default() {
    let shared = SharedString::default();
    assert!(shared.is_empty());
}

// =============================================================================
// Part 2: SharedVec Tests
// =============================================================================

#[test]
fn test_shared_vec_new() {
    let vec: SharedVec<i32> = SharedVec::new();
    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);
}

#[test]
fn test_shared_vec_push() {
    let vec = SharedVec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    assert_eq!(vec.len(), 3);
}

#[test]
fn test_shared_vec_pop() {
    let vec = SharedVec::new();
    vec.push(1);
    vec.push(2);
    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.pop(), Some(1));
    assert_eq!(vec.pop(), None);
}

#[test]
fn test_shared_vec_get() {
    let vec = SharedVec::new();
    vec.push(10);
    vec.push(20);
    vec.push(30);
    assert_eq!(vec.get(0), Some(10));
    assert_eq!(vec.get(1), Some(20));
    assert_eq!(vec.get(2), Some(30));
    assert_eq!(vec.get(3), None);
}

#[test]
fn test_shared_vec_is_empty() {
    let vec: SharedVec<i32> = SharedVec::new();
    assert!(vec.is_empty());
    vec.push(1);
    assert!(!vec.is_empty());
    vec.pop();
    assert!(vec.is_empty());
}

#[test]
fn test_shared_vec_strings() {
    let vec = SharedVec::new();
    vec.push(String::from("hello"));
    vec.push(String::from("world"));
    assert_eq!(vec.get(0), Some(String::from("hello")));
    assert_eq!(vec.get(1), Some(String::from("world")));
}

#[test]
fn test_shared_vec_mixed_operations() {
    let vec = SharedVec::new();
    vec.push(1);
    vec.push(2);
    vec.pop();
    vec.push(3);
    vec.push(4);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.get(0), Some(1));
    assert_eq!(vec.get(1), Some(3));
    assert_eq!(vec.get(2), Some(4));
}

#[test]
fn test_shared_vec_default() {
    let vec: SharedVec<i32> = SharedVec::default();
    assert!(vec.is_empty());
}

// =============================================================================
// Part 3: SafeCell Tests
// =============================================================================

#[test]
fn test_safe_cell_new() {
    let cell = SafeCell::new(42);
    assert_eq!(*cell.try_read().unwrap(), 42);
}

#[test]
fn test_safe_cell_try_read() {
    let cell = SafeCell::new(100);
    let read = cell.try_read();
    assert!(read.is_some());
    assert_eq!(*read.unwrap(), 100);
}

#[test]
fn test_safe_cell_try_write() {
    let cell = SafeCell::new(42);
    {
        let mut write = cell.try_write().unwrap();
        *write = 100;
    }
    assert_eq!(*cell.try_read().unwrap(), 100);
}

#[test]
fn test_safe_cell_multiple_reads() {
    let cell = SafeCell::new(42);
    let _read1 = cell.try_read();
    let read2 = cell.try_read();
    assert!(read2.is_some());
}

#[test]
fn test_safe_cell_write_fails_during_read() {
    let cell = SafeCell::new(42);
    let _read = cell.try_read().unwrap();
    let write = cell.try_write();
    assert!(write.is_none());
}

#[test]
fn test_safe_cell_read_fails_during_write() {
    let cell = SafeCell::new(42);
    let _write = cell.try_write().unwrap();
    let read = cell.try_read();
    assert!(read.is_none());
}

#[test]
fn test_safe_cell_is_borrowed_false() {
    let cell = SafeCell::new(42);
    assert!(!cell.is_borrowed());
}

#[test]
fn test_safe_cell_is_borrowed_during_read() {
    let cell = SafeCell::new(42);
    let _read = cell.try_read().unwrap();
    assert!(cell.is_borrowed());
}

#[test]
fn test_safe_cell_is_borrowed_during_write() {
    let cell = SafeCell::new(42);
    let _write = cell.try_write().unwrap();
    assert!(cell.is_borrowed());
}

#[test]
fn test_safe_cell_with_value() {
    let cell = SafeCell::new(42);
    let result = cell.with_value(|v| *v * 2);
    assert_eq!(result, Some(84));
}

#[test]
fn test_safe_cell_with_value_string() {
    let cell = SafeCell::new(String::from("hello"));
    let result = cell.with_value(|s| s.len());
    assert_eq!(result, Some(5));
}

#[test]
fn test_safe_cell_with_value_mut() {
    let cell = SafeCell::new(42);
    let old = cell.with_value_mut(|v| {
        let old = *v;
        *v = 100;
        old
    });
    assert_eq!(old, Some(42));
    assert_eq!(*cell.try_read().unwrap(), 100);
}

#[test]
fn test_safe_cell_with_value_fails_during_borrow() {
    let cell = SafeCell::new(42);
    let _write = cell.try_write().unwrap();
    let result = cell.with_value(|v| *v);
    assert!(result.is_none());
}

#[test]
fn test_safe_cell_with_value_mut_fails_during_borrow() {
    let cell = SafeCell::new(42);
    let _read = cell.try_read().unwrap();
    let result = cell.with_value_mut(|v| *v = 100);
    assert!(result.is_none());
}

#[test]
fn test_safe_cell_default() {
    let cell: SafeCell<i32> = SafeCell::default();
    assert_eq!(*cell.try_read().unwrap(), 0);
}

// =============================================================================
// Part 4: SharedCounter Tests
// =============================================================================

#[test]
fn test_shared_counter_new() {
    let counter = SharedCounter::new();
    assert_eq!(counter.get(), 0);
}

#[test]
fn test_shared_counter_increment() {
    let counter = SharedCounter::new();
    counter.increment();
    assert_eq!(counter.get(), 1);
    counter.increment();
    assert_eq!(counter.get(), 2);
}

#[test]
fn test_shared_counter_decrement() {
    let counter = SharedCounter::new();
    counter.add(5);
    counter.decrement();
    assert_eq!(counter.get(), 4);
}

#[test]
fn test_shared_counter_add() {
    let counter = SharedCounter::new();
    counter.add(100);
    assert_eq!(counter.get(), 100);
    counter.add(-50);
    assert_eq!(counter.get(), 50);
}

#[test]
fn test_shared_counter_shared_ownership() {
    let counter = SharedCounter::new();
    let counter2 = Rc::clone(&counter);
    let counter3 = Rc::clone(&counter);

    counter.increment();
    counter2.increment();
    counter3.increment();

    assert_eq!(counter.get(), 3);
    assert_eq!(counter2.get(), 3);
    assert_eq!(counter3.get(), 3);
}

#[test]
fn test_shared_counter_rc_count() {
    let counter = SharedCounter::new();
    assert_eq!(Rc::strong_count(&counter), 1);

    let counter2 = Rc::clone(&counter);
    assert_eq!(Rc::strong_count(&counter), 2);

    drop(counter2);
    assert_eq!(Rc::strong_count(&counter), 1);
}

#[test]
fn test_shared_counter_negative() {
    let counter = SharedCounter::new();
    counter.decrement();
    counter.decrement();
    assert_eq!(counter.get(), -2);
}

#[test]
fn test_shared_counter_mixed_operations() {
    let counter = SharedCounter::new();
    let c2 = Rc::clone(&counter);

    counter.add(10);
    c2.decrement();
    counter.increment();
    c2.add(-5);

    assert_eq!(counter.get(), 5);
}

// =============================================================================
// Part 4: TreeNode Tests
// =============================================================================

#[test]
fn test_tree_node_new() {
    let node = TreeNode::new(42);
    assert_eq!(*node.value(), 42);
    assert_eq!(node.children_count(), 0);
}

#[test]
fn test_tree_node_string_value() {
    let node = TreeNode::new(String::from("root"));
    assert_eq!(*node.value(), "root");
}

#[test]
fn test_tree_node_set_value() {
    let node = TreeNode::new(42);
    node.set_value(100);
    assert_eq!(*node.value(), 100);
}

#[test]
fn test_tree_node_add_child() {
    let parent = TreeNode::new("parent");
    let child = TreeNode::new("child");
    parent.add_child(child);
    assert_eq!(parent.children_count(), 1);
}

#[test]
fn test_tree_node_multiple_children() {
    let parent = TreeNode::new("parent");
    parent.add_child(TreeNode::new("child1"));
    parent.add_child(TreeNode::new("child2"));
    parent.add_child(TreeNode::new("child3"));
    assert_eq!(parent.children_count(), 3);
}

#[test]
fn test_tree_node_shared_child() {
    let parent1 = TreeNode::new("parent1");
    let parent2 = TreeNode::new("parent2");
    let shared_child = TreeNode::new("shared");

    parent1.add_child(Rc::clone(&shared_child));
    parent2.add_child(Rc::clone(&shared_child));

    assert_eq!(parent1.children_count(), 1);
    assert_eq!(parent2.children_count(), 1);
    assert_eq!(Rc::strong_count(&shared_child), 3); // Original + 2 parents
}

#[test]
fn test_tree_node_nested() {
    let root = TreeNode::new("root");
    let child = TreeNode::new("child");
    let grandchild = TreeNode::new("grandchild");

    child.add_child(grandchild);
    root.add_child(child);

    assert_eq!(root.children_count(), 1);
}

#[test]
fn test_tree_node_modify_after_add() {
    let root = TreeNode::new("root");
    let child = TreeNode::new("original");
    root.add_child(Rc::clone(&child));

    child.set_value("modified");
    assert_eq!(*child.value(), "modified");
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn test_counter_through_function() {
    fn modify_counter(c: &Counter) {
        c.increment();
        c.increment();
        c.decrement();
    }

    let counter = Counter::new();
    modify_counter(&counter);
    modify_counter(&counter);
    assert_eq!(counter.get(), 2);
}

#[test]
fn test_cached_value_in_struct() {
    struct Config {
        value: CachedValue<i32>,
    }

    let config = Config {
        value: CachedValue::new(42),
    };

    fn read_config(c: &Config) -> i32 {
        c.value.get()
    }

    assert_eq!(read_config(&config), 42);
    assert_eq!(read_config(&config), 42);
    assert_eq!(config.value.access_count(), 2);
}

#[test]
fn test_shared_string_builder_pattern() {
    let builder = SharedString::new("");
    builder.append("SELECT ");
    builder.append("* ");
    builder.append("FROM ");
    builder.append("users");
    assert_eq!(builder.get(), "SELECT * FROM users");
}

#[test]
fn test_shared_vec_stack_behavior() {
    let stack = SharedVec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    let mut result = Vec::new();
    while let Some(v) = stack.pop() {
        result.push(v);
    }

    assert_eq!(result, vec![3, 2, 1]);
}

#[test]
fn test_safe_cell_transaction_pattern() {
    let cell = SafeCell::new(100);

    // Simulate a transaction
    let result = cell.with_value_mut(|balance| {
        if *balance >= 50 {
            *balance -= 50;
            true
        } else {
            false
        }
    });

    assert_eq!(result, Some(true));
    assert_eq!(*cell.try_read().unwrap(), 50);
}

#[test]
fn test_shared_counter_parallel_style() {
    let counter = SharedCounter::new();
    let handles: Vec<Rc<SharedCounter>> = (0..5).map(|_| Rc::clone(&counter)).collect();

    for handle in &handles {
        handle.increment();
    }

    assert_eq!(counter.get(), 5);
}

#[test]
fn test_tree_node_file_system() {
    // Simulate a file system structure
    let root = TreeNode::new("/");
    let home = TreeNode::new("home");
    let user = TreeNode::new("user");
    let docs = TreeNode::new("documents");
    let pics = TreeNode::new("pictures");

    user.add_child(docs);
    user.add_child(pics);
    home.add_child(user);
    root.add_child(home);

    assert_eq!(root.children_count(), 1);
}

#[test]
fn test_cell_vs_refcell_difference() {
    // Cell: simple get/set for Copy types
    let counter = Counter::new();
    counter.set(42);
    assert_eq!(counter.get(), 42);

    // RefCell: borrowing for non-Copy types
    let shared = SharedString::new("hello");
    shared.append(" world");
    assert_eq!(shared.get(), "hello world");
}

#[test]
fn test_rc_refcell_pattern() {
    // The Rc<RefCell<T>> pattern allows multiple owners to mutate shared data
    let counter = SharedCounter::new();
    let c1 = Rc::clone(&counter);
    let c2 = Rc::clone(&counter);

    // All clones can mutate the same counter
    c1.add(10);
    c2.add(20);
    counter.increment();

    // All clones see the same value
    assert_eq!(c1.get(), 31);
    assert_eq!(c2.get(), 31);
    assert_eq!(counter.get(), 31);
}

#[test]
fn test_safe_cell_concurrent_access_simulation() {
    let cell = SafeCell::new(0);

    // Simulate checking before modifying
    if !cell.is_borrowed() {
        let _ = cell.with_value_mut(|v| *v += 1);
    }

    assert_eq!(*cell.try_read().unwrap(), 1);
}

#[test]
fn test_tree_modification_through_shared_ref() {
    let tree = TreeNode::new("root");
    let child = TreeNode::new("child");

    fn add_child_to_tree<T>(tree: &TreeNode<T>, child: Rc<TreeNode<T>>) {
        tree.add_child(child);
    }

    add_child_to_tree(&tree, child);
    assert_eq!(tree.children_count(), 1);
}

#[test]
fn test_counter_immutable_interface() {
    // Even though we have &Counter, we can modify it
    fn use_counter(c: &Counter) {
        c.increment();
    }

    let counter = Counter::new();
    use_counter(&counter);
    use_counter(&counter);
    assert_eq!(counter.get(), 2);
}

#[test]
fn test_shared_vec_with_complex_type() {
    #[derive(Clone, PartialEq, Debug)]
    struct Item {
        id: i32,
        name: String,
    }

    let vec = SharedVec::new();
    vec.push(Item {
        id: 1,
        name: "first".to_string(),
    });
    vec.push(Item {
        id: 2,
        name: "second".to_string(),
    });

    assert_eq!(
        vec.get(0),
        Some(Item {
            id: 1,
            name: "first".to_string()
        })
    );
}

#[test]
fn test_cached_value_reset_workflow() {
    let cached = CachedValue::new(42);

    // Access multiple times
    for _ in 0..10 {
        cached.get();
    }
    assert_eq!(cached.access_count(), 10);

    // Update resets count
    cached.set(100);
    assert_eq!(cached.access_count(), 0);
    assert_eq!(cached.get(), 100);
    assert_eq!(cached.access_count(), 1);
}

#[test]
fn test_safe_cell_guard_lifetime() {
    let cell = SafeCell::new(vec![1, 2, 3]);

    // Guard keeps the borrow alive
    {
        let guard = cell.try_read().unwrap();
        assert_eq!(guard.len(), 3);
        // guard dropped here
    }

    // Now we can write
    let result = cell.with_value_mut(|v| {
        v.push(4);
        v.len()
    });
    assert_eq!(result, Some(4));
}

#[test]
fn test_tree_diamond_pattern() {
    // Diamond pattern: two parents share the same child
    let shared_child = TreeNode::new("shared");
    let parent_a = TreeNode::new("A");
    let parent_b = TreeNode::new("B");

    parent_a.add_child(Rc::clone(&shared_child));
    parent_b.add_child(Rc::clone(&shared_child));

    // Modifying shared child is visible from both parents
    shared_child.set_value("modified");

    assert_eq!(*shared_child.value(), "modified");
    assert_eq!(Rc::strong_count(&shared_child), 3);
}
