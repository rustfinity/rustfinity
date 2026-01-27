use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Creates a new reference-counted value.
///
/// Wraps the given value in an `Rc<T>` for shared ownership.
///
/// # Arguments
///
/// * `value` - The value to wrap in an Rc
///
/// # Examples
///
/// ```
/// use rc_reference_counting::create_shared;
/// use std::rc::Rc;
///
/// let shared = create_shared(42);
/// assert_eq!(*shared, 42);
/// ```
pub fn create_shared<T>(value: T) -> Rc<T> {
    Rc::new(value)
}

/// Clones a reference-counted pointer.
///
/// This increments the reference count without cloning the underlying data.
///
/// # Arguments
///
/// * `rc` - The Rc to clone
///
/// # Examples
///
/// ```
/// use rc_reference_counting::{create_shared, clone_shared, get_strong_count};
///
/// let original = create_shared(42);
/// let cloned = clone_shared(&original);
/// assert_eq!(get_strong_count(&original), 2);
/// ```
pub fn clone_shared<T>(rc: &Rc<T>) -> Rc<T> {
    Rc::clone(rc)
}

/// Returns the strong reference count of an Rc.
///
/// # Arguments
///
/// * `rc` - The Rc to inspect
///
/// # Examples
///
/// ```
/// use rc_reference_counting::{create_shared, get_strong_count};
///
/// let shared = create_shared("hello");
/// assert_eq!(get_strong_count(&shared), 1);
/// ```
pub fn get_strong_count<T>(rc: &Rc<T>) -> usize {
    Rc::strong_count(rc)
}

/// Gets a clone of the value inside an Rc.
///
/// # Arguments
///
/// * `rc` - The Rc containing the value
///
/// # Examples
///
/// ```
/// use rc_reference_counting::{create_shared, get_value};
///
/// let shared = create_shared(String::from("hello"));
/// let value = get_value(&shared);
/// assert_eq!(value, "hello");
/// ```
pub fn get_value<T: Clone>(rc: &Rc<T>) -> T {
    (**rc).clone()
}

/// A buffer that can be shared across multiple owners.
///
/// Uses `Rc` internally to allow multiple parts of a program to
/// share access to the same buffer data without copying.
#[derive(Debug, Clone)]
pub struct SharedBuffer {
    data: Vec<u8>,
}

impl SharedBuffer {
    /// Creates a new shared buffer wrapped in Rc.
    ///
    /// # Arguments
    ///
    /// * `data` - The byte vector to store in the buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use rc_reference_counting::SharedBuffer;
    ///
    /// let buffer = SharedBuffer::new(vec![1, 2, 3]);
    /// assert_eq!(buffer.len(), 3);
    /// ```
    pub fn new(data: Vec<u8>) -> Rc<Self> {
        Rc::new(SharedBuffer { data })
    }

    /// Returns the length of the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use rc_reference_counting::SharedBuffer;
    ///
    /// let buffer = SharedBuffer::new(vec![1, 2, 3, 4, 5]);
    /// assert_eq!(buffer.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Gets a byte at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the byte to retrieve
    ///
    /// # Examples
    ///
    /// ```
    /// use rc_reference_counting::SharedBuffer;
    ///
    /// let buffer = SharedBuffer::new(vec![10, 20, 30]);
    /// assert_eq!(buffer.get(1), Some(20));
    /// assert_eq!(buffer.get(10), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<u8> {
        self.data.get(index).copied()
    }

    /// Returns the buffer data as a slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use rc_reference_counting::SharedBuffer;
    ///
    /// let buffer = SharedBuffer::new(vec![1, 2, 3]);
    /// assert_eq!(buffer.as_slice(), &[1, 2, 3]);
    /// ```
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
}

/// Creates a weak reference from an Rc.
///
/// Weak references don't prevent the value from being deallocated.
///
/// # Arguments
///
/// * `rc` - The Rc to create a weak reference from
///
/// # Examples
///
/// ```
/// use rc_reference_counting::{create_shared, create_weak, get_weak_count};
///
/// let strong = create_shared(42);
/// let weak = create_weak(&strong);
/// assert_eq!(get_weak_count(&strong), 1);
/// ```
pub fn create_weak<T>(rc: &Rc<T>) -> Weak<T> {
    Rc::downgrade(rc)
}

/// Attempts to upgrade a weak reference to a strong reference.
///
/// Returns `Some(Rc<T>)` if the value still exists, `None` if it was deallocated.
///
/// # Arguments
///
/// * `weak` - The weak reference to upgrade
///
/// # Examples
///
/// ```
/// use rc_reference_counting::{create_shared, create_weak, upgrade_weak};
///
/// let strong = create_shared(42);
/// let weak = create_weak(&strong);
///
/// // Value still exists
/// assert!(upgrade_weak(&weak).is_some());
///
/// drop(strong);
///
/// // Value was deallocated
/// assert!(upgrade_weak(&weak).is_none());
/// ```
pub fn upgrade_weak<T>(weak: &Weak<T>) -> Option<Rc<T>> {
    weak.upgrade()
}

/// Returns the weak reference count of an Rc.
///
/// # Arguments
///
/// * `rc` - The Rc to inspect
///
/// # Examples
///
/// ```
/// use rc_reference_counting::{create_shared, create_weak, get_weak_count};
///
/// let shared = create_shared("hello");
/// assert_eq!(get_weak_count(&shared), 0);
///
/// let _weak = create_weak(&shared);
/// assert_eq!(get_weak_count(&shared), 1);
/// ```
pub fn get_weak_count<T>(rc: &Rc<T>) -> usize {
    Rc::weak_count(rc)
}

/// A graph node that can have multiple parents sharing child nodes.
///
/// Uses `Rc<RefCell<...>>` pattern for interior mutability, allowing
/// modification of children even through shared references.
#[derive(Debug)]
pub struct Node<T> {
    value: T,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node<T> {
    /// Creates a new node wrapped in Rc.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the node
    ///
    /// # Examples
    ///
    /// ```
    /// use rc_reference_counting::Node;
    ///
    /// let node = Node::new(42);
    /// assert_eq!(*node.value(), 42);
    /// ```
    pub fn new(value: T) -> Rc<Self> {
        Rc::new(Node {
            value,
            children: RefCell::new(Vec::new()),
        })
    }

    /// Returns a reference to the node's value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rc_reference_counting::Node;
    ///
    /// let node = Node::new(String::from("root"));
    /// assert_eq!(node.value(), "root");
    /// ```
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Adds a child node.
    ///
    /// # Arguments
    ///
    /// * `child` - The child node to add
    ///
    /// # Examples
    ///
    /// ```
    /// use rc_reference_counting::Node;
    ///
    /// let parent = Node::new("parent");
    /// let child = Node::new("child");
    /// parent.add_child(child);
    /// assert_eq!(parent.children().len(), 1);
    /// ```
    pub fn add_child(&self, child: Rc<Node<T>>) {
        self.children.borrow_mut().push(child);
    }

    /// Returns a clone of the children vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use rc_reference_counting::Node;
    ///
    /// let node = Node::new(1);
    /// assert!(node.children().is_empty());
    /// ```
    pub fn children(&self) -> Vec<Rc<Node<T>>> {
        self.children.borrow().clone()
    }

    /// Returns the number of children.
    pub fn children_count(&self) -> usize {
        self.children.borrow().len()
    }
}

/// Trait for objects that can observe changes to an Observable.
pub trait Observer<T> {
    /// Called when the observed value is updated.
    fn on_update(&self, value: &T);
}

/// A value that can notify observers when it changes.
///
/// Observers are stored as weak references to prevent memory leaks
/// from circular references.
pub struct Observable<T> {
    value: T,
    observers: Vec<Weak<dyn Observer<T>>>,
}

impl<T> Observable<T> {
    /// Creates a new Observable with an initial value.
    ///
    /// # Arguments
    ///
    /// * `value` - The initial value
    ///
    /// # Examples
    ///
    /// ```
    /// use rc_reference_counting::Observable;
    ///
    /// let observable = Observable::new(42);
    /// assert_eq!(*observable.get(), 42);
    /// ```
    pub fn new(value: T) -> Self {
        Observable {
            value,
            observers: Vec::new(),
        }
    }

    /// Returns a reference to the current value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rc_reference_counting::Observable;
    ///
    /// let observable = Observable::new(String::from("hello"));
    /// assert_eq!(observable.get(), "hello");
    /// ```
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Sets a new value.
    ///
    /// # Arguments
    ///
    /// * `value` - The new value to set
    ///
    /// # Examples
    ///
    /// ```
    /// use rc_reference_counting::Observable;
    ///
    /// let mut observable = Observable::new(1);
    /// observable.set(2);
    /// assert_eq!(*observable.get(), 2);
    /// ```
    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    /// Subscribes an observer.
    ///
    /// The observer is stored as a weak reference to avoid memory leaks.
    ///
    /// # Arguments
    ///
    /// * `observer` - The observer to subscribe
    ///
    /// # Examples
    ///
    /// ```
    /// use rc_reference_counting::{Observable, Observer};
    /// use std::rc::Rc;
    /// use std::cell::Cell;
    ///
    /// struct Counter(Cell<i32>);
    /// impl Observer<i32> for Counter {
    ///     fn on_update(&self, value: &i32) {
    ///         self.0.set(*value);
    ///     }
    /// }
    ///
    /// let mut observable = Observable::new(0);
    /// let counter = Rc::new(Counter(Cell::new(0)));
    /// observable.subscribe(counter.clone());
    ///
    /// observable.set(42);
    /// observable.notify();
    /// assert_eq!(counter.0.get(), 42);
    /// ```
    pub fn subscribe(&mut self, observer: Rc<dyn Observer<T>>) {
        self.observers.push(Rc::downgrade(&observer));
    }

    /// Notifies all live observers of the current value.
    ///
    /// Dead observers (whose Rc has been dropped) are automatically removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use rc_reference_counting::{Observable, Observer};
    /// use std::rc::Rc;
    /// use std::cell::Cell;
    ///
    /// struct Logger(Cell<bool>);
    /// impl Observer<&str> for Logger {
    ///     fn on_update(&self, _value: &&str) {
    ///         self.0.set(true);
    ///     }
    /// }
    ///
    /// let mut observable = Observable::new("hello");
    /// let logger = Rc::new(Logger(Cell::new(false)));
    /// observable.subscribe(logger.clone());
    ///
    /// observable.notify();
    /// assert!(logger.0.get());
    /// ```
    pub fn notify(&mut self) {
        // Remove dead observers and notify live ones
        self.observers.retain(|weak| {
            if let Some(observer) = weak.upgrade() {
                observer.on_update(&self.value);
                true
            } else {
                false
            }
        });
    }

    /// Returns the number of live observers.
    pub fn observer_count(&self) -> usize {
        self.observers.iter().filter(|w| w.upgrade().is_some()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn test_create_shared_basic() {
        let shared = create_shared(42);
        assert_eq!(*shared, 42);
    }

    #[test]
    fn test_clone_shared_increments_count() {
        let original = create_shared(42);
        assert_eq!(get_strong_count(&original), 1);

        let cloned = clone_shared(&original);
        assert_eq!(get_strong_count(&original), 2);
        assert_eq!(get_strong_count(&cloned), 2);
    }

    #[test]
    fn test_get_value() {
        let shared = create_shared(String::from("hello"));
        let value = get_value(&shared);
        assert_eq!(value, "hello");
    }

    #[test]
    fn test_shared_buffer_basic() {
        let buffer = SharedBuffer::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(buffer.len(), 5);
        assert!(!buffer.is_empty());
    }

    #[test]
    fn test_node_with_children() {
        let parent = Node::new("parent");
        let child1 = Node::new("child1");
        let child2 = Node::new("child2");

        parent.add_child(child1);
        parent.add_child(child2);

        assert_eq!(parent.children_count(), 2);
    }

    // Helper struct for testing Observer
    struct TestObserver {
        last_value: Cell<i32>,
    }

    impl Observer<i32> for TestObserver {
        fn on_update(&self, value: &i32) {
            self.last_value.set(*value);
        }
    }

    #[test]
    fn test_observable_notify() {
        let mut observable = Observable::new(0);
        let observer = Rc::new(TestObserver {
            last_value: Cell::new(0),
        });

        observable.subscribe(observer.clone());
        observable.set(42);
        observable.notify();

        assert_eq!(observer.last_value.get(), 42);
    }
}
