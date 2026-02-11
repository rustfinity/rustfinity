use rc_reference_counting::*;
use std::cell::Cell;
use std::rc::Rc;

// =============================================================================
// Basic Rc Operations Tests
// =============================================================================

mod create_shared_tests {
    use super::*;

    #[test]
    fn creates_shared_integer() {
        let shared = create_shared(42);
        assert_eq!(*shared, 42);
    }

    #[test]
    fn creates_shared_string() {
        let shared = create_shared(String::from("hello"));
        assert_eq!(*shared, "hello");
    }

    #[test]
    fn creates_shared_vec() {
        let shared = create_shared(vec![1, 2, 3]);
        assert_eq!(*shared, vec![1, 2, 3]);
    }

    #[test]
    fn creates_shared_zero() {
        let shared = create_shared(0);
        assert_eq!(*shared, 0);
    }

    #[test]
    fn creates_shared_negative() {
        let shared = create_shared(-100);
        assert_eq!(*shared, -100);
    }

    #[test]
    fn creates_shared_float() {
        let shared = create_shared(3.14f64);
        assert!((*shared - 3.14).abs() < f64::EPSILON);
    }

    #[test]
    fn creates_shared_tuple() {
        let shared = create_shared((1, "two", 3.0));
        assert_eq!(*shared, (1, "two", 3.0));
    }

    #[test]
    fn initial_count_is_one() {
        let shared = create_shared(42);
        assert_eq!(get_strong_count(&shared), 1);
    }
}

mod clone_shared_tests {
    use super::*;

    #[test]
    fn clone_increments_count() {
        let original = create_shared(42);
        let _cloned = clone_shared(&original);
        assert_eq!(get_strong_count(&original), 2);
    }

    #[test]
    fn multiple_clones() {
        let original = create_shared(42);
        let _c1 = clone_shared(&original);
        let _c2 = clone_shared(&original);
        let _c3 = clone_shared(&original);
        assert_eq!(get_strong_count(&original), 4);
    }

    #[test]
    fn clones_share_same_value() {
        let original = create_shared(42);
        let cloned = clone_shared(&original);
        assert_eq!(*original, *cloned);
    }

    #[test]
    fn clone_of_clone() {
        let original = create_shared(42);
        let c1 = clone_shared(&original);
        let _c2 = clone_shared(&c1);
        assert_eq!(get_strong_count(&original), 3);
    }

    #[test]
    fn drop_decrements_count() {
        let original = create_shared(42);
        {
            let _cloned = clone_shared(&original);
            assert_eq!(get_strong_count(&original), 2);
        }
        assert_eq!(get_strong_count(&original), 1);
    }

    #[test]
    fn clone_string() {
        let original = create_shared(String::from("hello"));
        let cloned = clone_shared(&original);
        assert_eq!(*cloned, "hello");
        assert_eq!(get_strong_count(&original), 2);
    }
}

mod get_strong_count_tests {
    use super::*;

    #[test]
    fn count_one() {
        let shared = create_shared(42);
        assert_eq!(get_strong_count(&shared), 1);
    }

    #[test]
    fn count_increases_with_clones() {
        let shared = create_shared(42);
        let clones: Vec<_> = (0..5).map(|_| clone_shared(&shared)).collect();
        assert_eq!(get_strong_count(&shared), 6);
        drop(clones);
        assert_eq!(get_strong_count(&shared), 1);
    }

    #[test]
    fn count_through_clone() {
        let original = create_shared(42);
        let cloned = clone_shared(&original);
        assert_eq!(get_strong_count(&cloned), 2);
    }
}

mod get_value_tests {
    use super::*;

    #[test]
    fn gets_integer_value() {
        let shared = create_shared(42);
        assert_eq!(get_value(&shared), 42);
    }

    #[test]
    fn gets_string_value() {
        let shared = create_shared(String::from("hello"));
        let value = get_value(&shared);
        assert_eq!(value, "hello");
    }

    #[test]
    fn gets_vec_value() {
        let shared = create_shared(vec![1, 2, 3]);
        let value = get_value(&shared);
        assert_eq!(value, vec![1, 2, 3]);
    }

    #[test]
    fn value_is_independent_clone() {
        let shared = create_shared(String::from("hello"));
        let mut value = get_value(&shared);
        value.push_str(" world");
        assert_eq!(*shared, "hello"); // Original unchanged
        assert_eq!(value, "hello world");
    }

    #[test]
    fn gets_empty_string() {
        let shared = create_shared(String::new());
        assert_eq!(get_value(&shared), "");
    }

    #[test]
    fn gets_negative() {
        let shared = create_shared(-999);
        assert_eq!(get_value(&shared), -999);
    }
}

// =============================================================================
// SharedBuffer Tests
// =============================================================================

mod shared_buffer_tests {
    use super::*;

    #[test]
    fn new_creates_buffer() {
        let buffer = SharedBuffer::new(vec![1, 2, 3]);
        assert_eq!(buffer.len(), 3);
    }

    #[test]
    fn empty_buffer() {
        let buffer = SharedBuffer::new(vec![]);
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn get_valid_index() {
        let buffer = SharedBuffer::new(vec![10, 20, 30, 40, 50]);
        assert_eq!(buffer.get(0), Some(10));
        assert_eq!(buffer.get(2), Some(30));
        assert_eq!(buffer.get(4), Some(50));
    }

    #[test]
    fn get_invalid_index() {
        let buffer = SharedBuffer::new(vec![1, 2, 3]);
        assert_eq!(buffer.get(3), None);
        assert_eq!(buffer.get(100), None);
    }

    #[test]
    fn get_from_empty() {
        let buffer = SharedBuffer::new(vec![]);
        assert_eq!(buffer.get(0), None);
    }

    #[test]
    fn as_slice() {
        let buffer = SharedBuffer::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(buffer.as_slice(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn as_slice_empty() {
        let buffer = SharedBuffer::new(vec![]);
        assert_eq!(buffer.as_slice(), &[]);
    }

    #[test]
    fn shared_between_owners() {
        let buffer = SharedBuffer::new(vec![1, 2, 3]);
        let buffer2 = Rc::clone(&buffer);

        assert_eq!(buffer.len(), buffer2.len());
        assert_eq!(buffer.get(1), buffer2.get(1));
        assert_eq!(Rc::strong_count(&buffer), 2);
    }

    #[test]
    fn large_buffer() {
        let data: Vec<u8> = (0..255).collect();
        let buffer = SharedBuffer::new(data);
        assert_eq!(buffer.len(), 255);
        assert_eq!(buffer.get(100), Some(100));
    }

    #[test]
    fn is_empty_false() {
        let buffer = SharedBuffer::new(vec![1]);
        assert!(!buffer.is_empty());
    }
}

// =============================================================================
// Weak Reference Tests
// =============================================================================

mod weak_reference_tests {
    use super::*;

    #[test]
    fn create_weak_basic() {
        let strong = create_shared(42);
        let _weak = create_weak(&strong);
        assert_eq!(get_weak_count(&strong), 1);
    }

    #[test]
    fn weak_doesnt_affect_strong_count() {
        let strong = create_shared(42);
        let _weak = create_weak(&strong);
        assert_eq!(get_strong_count(&strong), 1);
    }

    #[test]
    fn multiple_weak_refs() {
        let strong = create_shared(42);
        let _w1 = create_weak(&strong);
        let _w2 = create_weak(&strong);
        let _w3 = create_weak(&strong);
        assert_eq!(get_weak_count(&strong), 3);
    }

    #[test]
    fn upgrade_succeeds_when_alive() {
        let strong = create_shared(42);
        let weak = create_weak(&strong);

        let upgraded = upgrade_weak(&weak);
        assert!(upgraded.is_some());
        assert_eq!(*upgraded.unwrap(), 42);
    }

    #[test]
    fn upgrade_fails_when_dropped() {
        let weak = {
            let strong = create_shared(42);
            create_weak(&strong)
        };

        assert!(upgrade_weak(&weak).is_none());
    }

    #[test]
    fn upgrade_increments_count() {
        let strong = create_shared(42);
        let weak = create_weak(&strong);

        let upgraded = upgrade_weak(&weak).unwrap();
        assert_eq!(get_strong_count(&strong), 2);
        drop(upgraded);
        assert_eq!(get_strong_count(&strong), 1);
    }

    #[test]
    fn weak_count_zero_initially() {
        let strong = create_shared(42);
        assert_eq!(get_weak_count(&strong), 0);
    }

    #[test]
    fn weak_with_string() {
        let strong = create_shared(String::from("hello"));
        let weak = create_weak(&strong);

        let upgraded = upgrade_weak(&weak).unwrap();
        assert_eq!(*upgraded, "hello");
    }

    #[test]
    fn weak_survives_strong_drop() {
        let weak = {
            let strong = create_shared(42);
            create_weak(&strong)
            // strong dropped here
        };
        // weak still exists, but can't upgrade
        assert!(upgrade_weak(&weak).is_none());
    }

    #[test]
    fn multiple_upgrade_attempts() {
        let strong = create_shared(42);
        let weak = create_weak(&strong);

        assert!(upgrade_weak(&weak).is_some());
        assert!(upgrade_weak(&weak).is_some());
        assert!(upgrade_weak(&weak).is_some());
    }
}

// =============================================================================
// Node Tests
// =============================================================================

mod node_tests {
    use super::*;

    #[test]
    fn new_node() {
        let node = Node::new(42);
        assert_eq!(*node.value(), 42);
    }

    #[test]
    fn node_with_string() {
        let node = Node::new(String::from("root"));
        assert_eq!(node.value(), "root");
    }

    #[test]
    fn empty_children() {
        let node = Node::new(1);
        assert!(node.children().is_empty());
        assert_eq!(node.children_count(), 0);
    }

    #[test]
    fn add_single_child() {
        let parent = Node::new("parent");
        let child = Node::new("child");

        parent.add_child(child);

        assert_eq!(parent.children_count(), 1);
    }

    #[test]
    fn add_multiple_children() {
        let parent = Node::new(0);
        let c1 = Node::new(1);
        let c2 = Node::new(2);
        let c3 = Node::new(3);

        parent.add_child(c1);
        parent.add_child(c2);
        parent.add_child(c3);

        assert_eq!(parent.children_count(), 3);
    }

    #[test]
    fn children_preserve_values() {
        let parent = Node::new("parent");
        let child = Node::new("child");
        parent.add_child(child);

        let children = parent.children();
        assert_eq!(*children[0].value(), "child");
    }

    #[test]
    fn shared_child() {
        let parent1 = Node::new("p1");
        let parent2 = Node::new("p2");
        let child = Node::new("shared");

        parent1.add_child(Rc::clone(&child));
        parent2.add_child(Rc::clone(&child));

        // Child is shared by both parents
        assert_eq!(Rc::strong_count(&child), 3); // original + 2 parents
    }

    #[test]
    fn nested_tree() {
        let root = Node::new(0);
        let level1 = Node::new(1);
        let level2 = Node::new(2);

        level1.add_child(level2);
        root.add_child(level1);

        assert_eq!(root.children_count(), 1);
        let children = root.children();
        assert_eq!(children[0].children_count(), 1);
    }

    #[test]
    fn node_reference_counting() {
        let node = Node::new(42);
        assert_eq!(Rc::strong_count(&node), 1);

        let cloned = Rc::clone(&node);
        assert_eq!(Rc::strong_count(&node), 2);

        drop(cloned);
        assert_eq!(Rc::strong_count(&node), 1);
    }

    #[test]
    fn children_are_clones() {
        let parent = Node::new("parent");
        let child = Node::new("child");
        parent.add_child(Rc::clone(&child));

        // Getting children returns clones
        let children1 = parent.children();
        let children2 = parent.children();

        // Same underlying node
        assert_eq!(children1[0].value(), children2[0].value());
    }
}

// =============================================================================
// Observable Tests
// =============================================================================

struct CountingObserver {
    count: Cell<i32>,
    last_value: Cell<i32>,
}

impl CountingObserver {
    fn new() -> Self {
        CountingObserver {
            count: Cell::new(0),
            last_value: Cell::new(0),
        }
    }
}

impl Observer<i32> for CountingObserver {
    fn on_update(&self, value: &i32) {
        self.count.set(self.count.get() + 1);
        self.last_value.set(*value);
    }
}

struct StringObserver {
    received: Cell<bool>,
}

impl Observer<String> for StringObserver {
    fn on_update(&self, _value: &String) {
        self.received.set(true);
    }
}

mod observable_tests {
    use super::*;

    #[test]
    fn new_observable() {
        let observable = Observable::new(42);
        assert_eq!(*observable.get(), 42);
    }

    #[test]
    fn set_value() {
        let mut observable = Observable::new(1);
        observable.set(2);
        assert_eq!(*observable.get(), 2);
    }

    #[test]
    fn multiple_sets() {
        let mut observable = Observable::new(0);
        observable.set(1);
        observable.set(2);
        observable.set(3);
        assert_eq!(*observable.get(), 3);
    }

    #[test]
    fn subscribe_observer() {
        let mut observable = Observable::new(0);
        let observer = Rc::new(CountingObserver::new());

        observable.subscribe(observer.clone());
        assert_eq!(observable.observer_count(), 1);
    }

    #[test]
    fn notify_observers() {
        let mut observable = Observable::new(0);
        let observer = Rc::new(CountingObserver::new());

        observable.subscribe(observer.clone());
        observable.set(42);
        observable.notify();

        assert_eq!(observer.count.get(), 1);
        assert_eq!(observer.last_value.get(), 42);
    }

    #[test]
    fn multiple_observers() {
        let mut observable = Observable::new(0);
        let obs1 = Rc::new(CountingObserver::new());
        let obs2 = Rc::new(CountingObserver::new());

        observable.subscribe(obs1.clone());
        observable.subscribe(obs2.clone());

        observable.set(10);
        observable.notify();

        assert_eq!(obs1.last_value.get(), 10);
        assert_eq!(obs2.last_value.get(), 10);
    }

    #[test]
    fn multiple_notifications() {
        let mut observable = Observable::new(0);
        let observer = Rc::new(CountingObserver::new());

        observable.subscribe(observer.clone());

        observable.set(1);
        observable.notify();
        observable.set(2);
        observable.notify();
        observable.set(3);
        observable.notify();

        assert_eq!(observer.count.get(), 3);
        assert_eq!(observer.last_value.get(), 3);
    }

    #[test]
    fn dead_observer_removed() {
        let mut observable = Observable::new(0);

        {
            let observer = Rc::new(CountingObserver::new());
            observable.subscribe(observer.clone());
            assert_eq!(observable.observer_count(), 1);
        }
        // observer dropped

        observable.notify(); // Should clean up dead observer
        assert_eq!(observable.observer_count(), 0);
    }

    #[test]
    fn some_observers_survive() {
        let mut observable = Observable::new(0);
        let survivor = Rc::new(CountingObserver::new());

        observable.subscribe(survivor.clone());
        {
            let temp = Rc::new(CountingObserver::new());
            observable.subscribe(temp.clone());
            assert_eq!(observable.observer_count(), 2);
        }

        observable.set(5);
        observable.notify();

        assert_eq!(observable.observer_count(), 1);
        assert_eq!(survivor.last_value.get(), 5);
    }

    #[test]
    fn string_observable() {
        let mut observable = Observable::new(String::from("hello"));
        let observer = Rc::new(StringObserver {
            received: Cell::new(false),
        });

        observable.subscribe(observer.clone());
        observable.set(String::from("world"));
        observable.notify();

        assert!(observer.received.get());
    }

    #[test]
    fn no_observers() {
        let mut observable = Observable::new(42);
        observable.notify(); // Should not panic
        assert_eq!(observable.observer_count(), 0);
    }

    #[test]
    fn observer_count_accurate() {
        let mut observable = Observable::new(0);

        assert_eq!(observable.observer_count(), 0);

        let o1 = Rc::new(CountingObserver::new());
        observable.subscribe(o1.clone());
        assert_eq!(observable.observer_count(), 1);

        let o2 = Rc::new(CountingObserver::new());
        observable.subscribe(o2.clone());
        assert_eq!(observable.observer_count(), 2);

        drop(o1);
        // Count still 2 until notify cleans up
        observable.notify();
        assert_eq!(observable.observer_count(), 1);
    }
}

// =============================================================================
// Integration Tests
// =============================================================================

mod integration_tests {
    use super::*;

    #[test]
    fn shared_buffer_workflow() {
        // Create a shared buffer
        let buffer = SharedBuffer::new(vec![1, 2, 3, 4, 5]);

        // Share among multiple owners
        let owner1 = Rc::clone(&buffer);
        let owner2 = Rc::clone(&buffer);

        // All see the same data
        assert_eq!(owner1.as_slice(), owner2.as_slice());
        assert_eq!(Rc::strong_count(&buffer), 3);

        // One owner drops
        drop(owner1);
        assert_eq!(Rc::strong_count(&buffer), 2);

        // Data still accessible
        assert_eq!(buffer.get(2), Some(3));
    }

    #[test]
    fn tree_structure() {
        // Build a tree
        let root = Node::new("root");
        let a = Node::new("a");
        let b = Node::new("b");
        let a1 = Node::new("a1");
        let a2 = Node::new("a2");

        a.add_child(a1);
        a.add_child(a2);
        root.add_child(a);
        root.add_child(b);

        // Verify structure
        assert_eq!(root.children_count(), 2);
        let children = root.children();
        assert_eq!(*children[0].value(), "a");
        assert_eq!(children[0].children_count(), 2);
    }

    #[test]
    fn diamond_pattern() {
        // Diamond dependency: A -> B, A -> C, B -> D, C -> D
        let d = Node::new("D");
        let b = Node::new("B");
        let c = Node::new("C");
        let a = Node::new("A");

        b.add_child(Rc::clone(&d));
        c.add_child(Rc::clone(&d));
        a.add_child(Rc::clone(&b));
        a.add_child(Rc::clone(&c));

        // D is shared by both B and C
        assert_eq!(Rc::strong_count(&d), 3); // original + B's + C's
    }

    #[test]
    fn weak_ref_observer_pattern() {
        let mut observable = Observable::new(0);
        let weak_check = {
            let observer = Rc::new(CountingObserver::new());
            observable.subscribe(observer.clone());

            // Create a weak ref to check if observer is alive
            Rc::downgrade(&observer)
        };

        // Observer was dropped, weak should fail to upgrade
        assert!(weak_check.upgrade().is_none());

        // Observable should clean up dead observer on notify
        observable.notify();
        assert_eq!(observable.observer_count(), 0);
    }

    #[test]
    fn rc_vs_weak_semantics() {
        let strong = create_shared(String::from("data"));
        let weak = create_weak(&strong);

        // Both can access
        assert_eq!(get_value(&strong), "data");
        assert_eq!(*upgrade_weak(&weak).unwrap(), "data");

        // Strong keeps data alive
        assert_eq!(get_strong_count(&strong), 1);
        assert_eq!(get_weak_count(&strong), 1);

        // Drop strong
        drop(strong);

        // Weak can't access anymore
        assert!(upgrade_weak(&weak).is_none());
    }

    #[test]
    fn clone_vs_get_value() {
        let shared = create_shared(vec![1, 2, 3]);

        // clone_shared increments count, shares data
        let cloned = clone_shared(&shared);
        assert_eq!(get_strong_count(&shared), 2);

        // get_value creates independent copy
        let value = get_value(&shared);
        assert_eq!(get_strong_count(&shared), 2); // Still 2, no new Rc

        // Verify they're the same content
        assert_eq!(*cloned, value);
    }

    #[test]
    fn observable_lifecycle() {
        let mut observable = Observable::new(100);

        // Add observers
        let o1 = Rc::new(CountingObserver::new());
        let o2 = Rc::new(CountingObserver::new());
        let o3 = Rc::new(CountingObserver::new());

        observable.subscribe(o1.clone());
        observable.subscribe(o2.clone());
        observable.subscribe(o3.clone());

        // Notify all
        observable.set(200);
        observable.notify();
        assert_eq!(o1.last_value.get(), 200);
        assert_eq!(o2.last_value.get(), 200);
        assert_eq!(o3.last_value.get(), 200);

        // Drop one
        drop(o2);
        observable.set(300);
        observable.notify();

        // Remaining observers updated
        assert_eq!(o1.last_value.get(), 300);
        assert_eq!(o3.last_value.get(), 300);
        assert_eq!(observable.observer_count(), 2);
    }

    #[test]
    fn buffer_with_weak_reference() {
        let weak = {
            let buffer = SharedBuffer::new(vec![1, 2, 3, 4, 5]);
            let weak = Rc::downgrade(&buffer);

            // Buffer still alive
            assert!(weak.upgrade().is_some());
            weak
        };

        // Buffer dropped, weak can't upgrade
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn node_reference_cleanup() {
        let parent = Node::new("parent");

        {
            let child = Node::new("child");
            parent.add_child(Rc::clone(&child));
            assert_eq!(Rc::strong_count(&child), 2);
        }
        // child's original Rc dropped, but parent still holds one

        // Children still accessible through parent
        let children = parent.children();
        assert_eq!(children.len(), 1);
        assert_eq!(*children[0].value(), "child");
    }

    #[test]
    fn complex_sharing() {
        // Shared data accessed by multiple components
        let shared_data = create_shared(vec![1, 2, 3, 4, 5]);

        let processors: Vec<_> = (0..5)
            .map(|_| clone_shared(&shared_data))
            .collect();

        assert_eq!(get_strong_count(&shared_data), 6);

        // All processors see the same data
        for p in &processors {
            assert_eq!(get_value(p), vec![1, 2, 3, 4, 5]);
        }

        // Drop all processors
        drop(processors);
        assert_eq!(get_strong_count(&shared_data), 1);
    }
}
