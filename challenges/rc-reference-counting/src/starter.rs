use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Creates a new reference-counted value.
///
/// Wraps the given value in an `Rc<T>` for shared ownership.
pub fn create_shared<T>(value: T) -> Rc<T> {
    // TODO: Implement using Rc::new
    unimplemented!()
}

/// Clones a reference-counted pointer.
///
/// This increments the reference count without cloning the underlying data.
pub fn clone_shared<T>(rc: &Rc<T>) -> Rc<T> {
    // TODO: Implement using Rc::clone
    unimplemented!()
}

/// Returns the strong reference count of an Rc.
pub fn get_strong_count<T>(rc: &Rc<T>) -> usize {
    // TODO: Implement using Rc::strong_count
    unimplemented!()
}

/// Gets a clone of the value inside an Rc.
pub fn get_value<T: Clone>(rc: &Rc<T>) -> T {
    // TODO: Dereference the Rc and clone the value
    unimplemented!()
}

/// A buffer that can be shared across multiple owners.
#[derive(Debug, Clone)]
pub struct SharedBuffer {
    data: Vec<u8>,
}

impl SharedBuffer {
    /// Creates a new shared buffer wrapped in Rc.
    pub fn new(data: Vec<u8>) -> Rc<Self> {
        // TODO: Create a SharedBuffer and wrap it in Rc
        unimplemented!()
    }

    /// Returns the length of the buffer.
    pub fn len(&self) -> usize {
        // TODO: Return the length of the data
        unimplemented!()
    }

    /// Returns true if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        // TODO: Check if the data is empty
        unimplemented!()
    }

    /// Gets a byte at the specified index.
    pub fn get(&self, index: usize) -> Option<u8> {
        // TODO: Return the byte at index, or None if out of bounds
        unimplemented!()
    }

    /// Returns the buffer data as a slice.
    pub fn as_slice(&self) -> &[u8] {
        // TODO: Return a reference to the data as a slice
        unimplemented!()
    }
}

/// Creates a weak reference from an Rc.
///
/// Weak references don't prevent the value from being deallocated.
pub fn create_weak<T>(rc: &Rc<T>) -> Weak<T> {
    // TODO: Implement using Rc::downgrade
    unimplemented!()
}

/// Attempts to upgrade a weak reference to a strong reference.
///
/// Returns `Some(Rc<T>)` if the value still exists, `None` if it was deallocated.
pub fn upgrade_weak<T>(weak: &Weak<T>) -> Option<Rc<T>> {
    // TODO: Implement using weak.upgrade()
    unimplemented!()
}

/// Returns the weak reference count of an Rc.
pub fn get_weak_count<T>(rc: &Rc<T>) -> usize {
    // TODO: Implement using Rc::weak_count
    unimplemented!()
}

/// A graph node that can have multiple parents sharing child nodes.
///
/// Uses `Rc<RefCell<...>>` pattern for interior mutability.
#[derive(Debug)]
pub struct Node<T> {
    value: T,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node<T> {
    /// Creates a new node wrapped in Rc.
    pub fn new(value: T) -> Rc<Self> {
        // TODO: Create a Node with empty children and wrap in Rc
        unimplemented!()
    }

    /// Returns a reference to the node's value.
    pub fn value(&self) -> &T {
        // TODO: Return a reference to the value field
        unimplemented!()
    }

    /// Adds a child node.
    pub fn add_child(&self, child: Rc<Node<T>>) {
        // TODO: Borrow the children RefCell mutably and push the child
        unimplemented!()
    }

    /// Returns a clone of the children vector.
    pub fn children(&self) -> Vec<Rc<Node<T>>> {
        // TODO: Borrow the children and clone the vector
        unimplemented!()
    }

    /// Returns the number of children.
    pub fn children_count(&self) -> usize {
        // TODO: Borrow children and return the length
        unimplemented!()
    }
}

/// Trait for objects that can observe changes to an Observable.
pub trait Observer<T> {
    /// Called when the observed value is updated.
    fn on_update(&self, value: &T);
}

/// A value that can notify observers when it changes.
///
/// Observers are stored as weak references to prevent memory leaks.
pub struct Observable<T> {
    value: T,
    observers: Vec<Weak<dyn Observer<T>>>,
}

impl<T> Observable<T> {
    /// Creates a new Observable with an initial value.
    pub fn new(value: T) -> Self {
        // TODO: Create Observable with the value and an empty observers vector
        unimplemented!()
    }

    /// Returns a reference to the current value.
    pub fn get(&self) -> &T {
        // TODO: Return a reference to the value
        unimplemented!()
    }

    /// Sets a new value.
    pub fn set(&mut self, value: T) {
        // TODO: Update the value field
        unimplemented!()
    }

    /// Subscribes an observer.
    ///
    /// The observer is stored as a weak reference to avoid memory leaks.
    pub fn subscribe(&mut self, observer: Rc<dyn Observer<T>>) {
        // TODO: Create a weak reference from the observer and add to the vector
        // Hint: Use Rc::downgrade
        unimplemented!()
    }

    /// Notifies all live observers of the current value.
    ///
    /// Dead observers (whose Rc has been dropped) are automatically removed.
    pub fn notify(&mut self) {
        // TODO: Iterate through observers, upgrade each weak reference
        // If upgrade succeeds, call on_update with the current value
        // Remove observers that can't be upgraded (their Rc was dropped)
        // Hint: Use Vec::retain with a closure that upgrades and notifies
        unimplemented!()
    }

    /// Returns the number of live observers.
    pub fn observer_count(&self) -> usize {
        // TODO: Count observers whose weak references can be upgraded
        unimplemented!()
    }
}

// Example usage
pub fn main() {
    // Basic Rc operations
    let shared = create_shared(42);
    println!("Value: {}", get_value(&shared));
    println!("Strong count: {}", get_strong_count(&shared));

    let cloned = clone_shared(&shared);
    println!("After clone, strong count: {}", get_strong_count(&shared));

    // Shared buffer
    let buffer = SharedBuffer::new(vec![1, 2, 3, 4, 5]);
    println!("Buffer length: {}", buffer.len());
    println!("Buffer[2]: {:?}", buffer.get(2));

    // Weak references
    let strong = create_shared(String::from("hello"));
    let weak = create_weak(&strong);
    println!("Weak count: {}", get_weak_count(&strong));

    if let Some(upgraded) = upgrade_weak(&weak) {
        println!("Upgraded value: {}", *upgraded);
    }

    // Node tree
    let root = Node::new("root");
    let child1 = Node::new("child1");
    let child2 = Node::new("child2");

    root.add_child(child1);
    root.add_child(child2);
    println!("Root has {} children", root.children_count());

    // Observable pattern
    use std::cell::Cell;

    struct PrintObserver;
    impl Observer<i32> for PrintObserver {
        fn on_update(&self, value: &i32) {
            println!("Value changed to: {}", value);
        }
    }

    let mut observable = Observable::new(0);
    let observer = Rc::new(PrintObserver);
    observable.subscribe(observer);

    observable.set(42);
    observable.notify();
}
