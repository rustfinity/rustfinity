use std::cell::{Cell, Ref, RefCell, RefMut};
use std::rc::Rc;

// =============================================================================
// Part 1: Cell Basics
// =============================================================================

/// A simple counter using `Cell` for interior mutability.
///
/// `Cell<T>` provides interior mutability for `Copy` types with zero overhead.
/// All operations work through shared references (&self).
///
/// # Examples
///
/// ```
/// use cell_refcell::Counter;
///
/// let counter = Counter::new();
/// counter.increment();
/// counter.increment();
/// assert_eq!(counter.get(), 2);
/// ```
#[derive(Debug, Default)]
pub struct Counter {
    value: Cell<i32>,
}

impl Counter {
    /// Creates a new counter with initial value 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::Counter;
    ///
    /// let counter = Counter::new();
    /// assert_eq!(counter.get(), 0);
    /// ```
    pub fn new() -> Self {
        Counter {
            value: Cell::new(0),
        }
    }

    /// Gets the current count.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::Counter;
    ///
    /// let counter = Counter::new();
    /// counter.set(42);
    /// assert_eq!(counter.get(), 42);
    /// ```
    pub fn get(&self) -> i32 {
        self.value.get()
    }

    /// Sets the count to a specific value.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::Counter;
    ///
    /// let counter = Counter::new();
    /// counter.set(100);
    /// assert_eq!(counter.get(), 100);
    /// ```
    pub fn set(&self, value: i32) {
        self.value.set(value);
    }

    /// Increments the count by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::Counter;
    ///
    /// let counter = Counter::new();
    /// counter.increment();
    /// assert_eq!(counter.get(), 1);
    /// ```
    pub fn increment(&self) {
        self.value.set(self.value.get() + 1);
    }

    /// Decrements the count by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::Counter;
    ///
    /// let counter = Counter::new();
    /// counter.set(5);
    /// counter.decrement();
    /// assert_eq!(counter.get(), 4);
    /// ```
    pub fn decrement(&self) {
        self.value.set(self.value.get() - 1);
    }
}

/// A cached value that tracks how many times it's been accessed.
///
/// Uses `Cell` for both the value and access count.
///
/// # Examples
///
/// ```
/// use cell_refcell::CachedValue;
///
/// let cached = CachedValue::new(42);
/// assert_eq!(cached.get(), 42);
/// assert_eq!(cached.get(), 42);
/// assert_eq!(cached.access_count(), 2);
/// ```
#[derive(Debug)]
pub struct CachedValue<T: Copy + Default> {
    value: Cell<T>,
    access_count: Cell<u32>,
}

impl<T: Copy + Default> CachedValue<T> {
    /// Creates a new cached value.
    ///
    /// # Arguments
    ///
    /// * `value` - The initial value to cache
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::CachedValue;
    ///
    /// let cached = CachedValue::new(100);
    /// assert_eq!(cached.access_count(), 0);
    /// ```
    pub fn new(value: T) -> Self {
        CachedValue {
            value: Cell::new(value),
            access_count: Cell::new(0),
        }
    }

    /// Gets the cached value and increments the access count.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::CachedValue;
    ///
    /// let cached = CachedValue::new(42);
    /// assert_eq!(cached.get(), 42);
    /// assert_eq!(cached.access_count(), 1);
    /// ```
    pub fn get(&self) -> T {
        self.access_count.set(self.access_count.get() + 1);
        self.value.get()
    }

    /// Sets a new value and resets the access count.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::CachedValue;
    ///
    /// let cached = CachedValue::new(42);
    /// cached.get(); // access_count = 1
    /// cached.set(100);
    /// assert_eq!(cached.access_count(), 0);
    /// ```
    pub fn set(&self, value: T) {
        self.value.set(value);
        self.access_count.set(0);
    }

    /// Returns the number of times the value was accessed.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::CachedValue;
    ///
    /// let cached = CachedValue::new(42);
    /// cached.get();
    /// cached.get();
    /// cached.get();
    /// assert_eq!(cached.access_count(), 3);
    /// ```
    pub fn access_count(&self) -> u32 {
        self.access_count.get()
    }
}

impl<T: Copy + Default> Default for CachedValue<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

// =============================================================================
// Part 2: RefCell Basics
// =============================================================================

/// A string that can be modified through shared references.
///
/// Uses `RefCell` for interior mutability of the String.
///
/// # Examples
///
/// ```
/// use cell_refcell::SharedString;
///
/// let shared = SharedString::new("Hello");
/// shared.append(", World!");
/// assert_eq!(shared.get(), "Hello, World!");
/// ```
#[derive(Debug, Default)]
pub struct SharedString {
    value: RefCell<String>,
}

impl SharedString {
    /// Creates a new shared string.
    ///
    /// # Arguments
    ///
    /// * `s` - The initial string content
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SharedString;
    ///
    /// let shared = SharedString::new("test");
    /// assert_eq!(shared.get(), "test");
    /// ```
    pub fn new(s: &str) -> Self {
        SharedString {
            value: RefCell::new(s.to_string()),
        }
    }

    /// Gets a clone of the string.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SharedString;
    ///
    /// let shared = SharedString::new("hello");
    /// assert_eq!(shared.get(), "hello");
    /// ```
    pub fn get(&self) -> String {
        self.value.borrow().clone()
    }

    /// Sets a new string value.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SharedString;
    ///
    /// let shared = SharedString::new("old");
    /// shared.set("new");
    /// assert_eq!(shared.get(), "new");
    /// ```
    pub fn set(&self, s: &str) {
        *self.value.borrow_mut() = s.to_string();
    }

    /// Appends a string to the current value.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SharedString;
    ///
    /// let shared = SharedString::new("Hello");
    /// shared.append(" World");
    /// assert_eq!(shared.get(), "Hello World");
    /// ```
    pub fn append(&self, s: &str) {
        self.value.borrow_mut().push_str(s);
    }

    /// Returns the length of the string.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SharedString;
    ///
    /// let shared = SharedString::new("hello");
    /// assert_eq!(shared.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.value.borrow().len()
    }

    /// Returns true if the string is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SharedString;
    ///
    /// let shared = SharedString::new("");
    /// assert!(shared.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.value.borrow().is_empty()
    }
}

/// A vector with interior mutability.
///
/// Uses `RefCell` to allow modification through shared references.
///
/// # Examples
///
/// ```
/// use cell_refcell::SharedVec;
///
/// let vec: SharedVec<i32> = SharedVec::new();
/// vec.push(1);
/// vec.push(2);
/// assert_eq!(vec.len(), 2);
/// ```
#[derive(Debug, Default)]
pub struct SharedVec<T> {
    data: RefCell<Vec<T>>,
}

impl<T> SharedVec<T> {
    /// Creates a new empty shared vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SharedVec;
    ///
    /// let vec: SharedVec<i32> = SharedVec::new();
    /// assert!(vec.is_empty());
    /// ```
    pub fn new() -> Self {
        SharedVec {
            data: RefCell::new(Vec::new()),
        }
    }

    /// Adds an element to the end of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SharedVec;
    ///
    /// let vec = SharedVec::new();
    /// vec.push(42);
    /// assert_eq!(vec.len(), 1);
    /// ```
    pub fn push(&self, value: T) {
        self.data.borrow_mut().push(value);
    }

    /// Removes and returns the last element, or None if empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SharedVec;
    ///
    /// let vec = SharedVec::new();
    /// vec.push(1);
    /// vec.push(2);
    /// assert_eq!(vec.pop(), Some(2));
    /// ```
    pub fn pop(&self) -> Option<T> {
        self.data.borrow_mut().pop()
    }

    /// Returns the current number of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SharedVec;
    ///
    /// let vec: SharedVec<i32> = SharedVec::new();
    /// assert_eq!(vec.len(), 0);
    /// vec.push(1);
    /// assert_eq!(vec.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.data.borrow().len()
    }

    /// Returns true if the vector is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SharedVec;
    ///
    /// let vec: SharedVec<i32> = SharedVec::new();
    /// assert!(vec.is_empty());
    /// vec.push(1);
    /// assert!(!vec.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.borrow().is_empty()
    }
}

impl<T: Clone> SharedVec<T> {
    /// Gets a clone of the element at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the element to retrieve
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SharedVec;
    ///
    /// let vec = SharedVec::new();
    /// vec.push(10);
    /// vec.push(20);
    /// assert_eq!(vec.get(0), Some(10));
    /// assert_eq!(vec.get(1), Some(20));
    /// assert_eq!(vec.get(2), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<T> {
        self.data.borrow().get(index).cloned()
    }
}

// =============================================================================
// Part 3: Safe Borrowing with try_borrow
// =============================================================================

/// A RefCell wrapper that provides safe, non-panicking borrow operations.
///
/// Uses `try_borrow` and `try_borrow_mut` internally to avoid panics.
///
/// # Examples
///
/// ```
/// use cell_refcell::SafeCell;
///
/// let cell = SafeCell::new(42);
/// let result = cell.with_value(|v| *v * 2);
/// assert_eq!(result, Some(84));
/// ```
#[derive(Debug, Default)]
pub struct SafeCell<T> {
    value: RefCell<T>,
}

impl<T> SafeCell<T> {
    /// Creates a new SafeCell.
    ///
    /// # Arguments
    ///
    /// * `value` - The initial value
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SafeCell;
    ///
    /// let cell = SafeCell::new("hello");
    /// ```
    pub fn new(value: T) -> Self {
        SafeCell {
            value: RefCell::new(value),
        }
    }

    /// Tries to borrow the value immutably.
    ///
    /// Returns `None` if the value is currently mutably borrowed.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SafeCell;
    ///
    /// let cell = SafeCell::new(42);
    /// let val = cell.try_read();
    /// assert!(val.is_some());
    /// assert_eq!(*val.unwrap(), 42);
    /// ```
    pub fn try_read(&self) -> Option<Ref<'_, T>> {
        self.value.try_borrow().ok()
    }

    /// Tries to borrow the value mutably.
    ///
    /// Returns `None` if the value is currently borrowed (either mutably or immutably).
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SafeCell;
    ///
    /// let cell = SafeCell::new(42);
    /// if let Some(mut val) = cell.try_write() {
    ///     *val = 100;
    /// }
    /// assert_eq!(*cell.try_read().unwrap(), 100);
    /// ```
    pub fn try_write(&self) -> Option<RefMut<'_, T>> {
        self.value.try_borrow_mut().ok()
    }

    /// Checks if the value is currently borrowed.
    ///
    /// Returns `true` if there are any active borrows (mutable or immutable).
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SafeCell;
    ///
    /// let cell = SafeCell::new(42);
    /// assert!(!cell.is_borrowed());
    ///
    /// let _borrow = cell.try_read();
    /// // While _borrow is alive, the cell is borrowed
    /// ```
    pub fn is_borrowed(&self) -> bool {
        self.value.try_borrow_mut().is_err()
    }

    /// Applies a function to the value if not currently borrowed.
    ///
    /// Returns `None` if the value cannot be borrowed.
    ///
    /// # Arguments
    ///
    /// * `f` - A function to apply to the value
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SafeCell;
    ///
    /// let cell = SafeCell::new(42);
    /// let doubled = cell.with_value(|v| *v * 2);
    /// assert_eq!(doubled, Some(84));
    /// ```
    pub fn with_value<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        self.try_read().map(|val| f(&*val))
    }

    /// Applies a mutating function to the value if not currently borrowed.
    ///
    /// Returns `None` if the value cannot be mutably borrowed.
    ///
    /// # Arguments
    ///
    /// * `f` - A function to apply to the mutable value
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SafeCell;
    ///
    /// let cell = SafeCell::new(42);
    /// let old_value = cell.with_value_mut(|v| {
    ///     let old = *v;
    ///     *v = 100;
    ///     old
    /// });
    /// assert_eq!(old_value, Some(42));
    /// assert_eq!(*cell.try_read().unwrap(), 100);
    /// ```
    pub fn with_value_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        self.try_write().map(|mut val| f(&mut *val))
    }
}

// =============================================================================
// Part 4: Combining Rc and RefCell
// =============================================================================

/// A reference-counted counter with interior mutability.
///
/// Combines `Rc` for shared ownership with `RefCell` for interior mutability.
/// This allows multiple owners to increment/decrement the same counter.
///
/// # Examples
///
/// ```
/// use cell_refcell::SharedCounter;
/// use std::rc::Rc;
///
/// let counter = SharedCounter::new();
/// let counter2 = Rc::clone(&counter);
///
/// counter.increment();
/// counter2.increment();
/// assert_eq!(counter.get(), 2);
/// ```
#[derive(Debug, Default)]
pub struct SharedCounter {
    value: RefCell<i32>,
}

impl SharedCounter {
    /// Creates a new shared counter wrapped in Rc.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SharedCounter;
    ///
    /// let counter = SharedCounter::new();
    /// assert_eq!(counter.get(), 0);
    /// ```
    pub fn new() -> Rc<Self> {
        Rc::new(SharedCounter {
            value: RefCell::new(0),
        })
    }

    /// Gets the current value.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SharedCounter;
    ///
    /// let counter = SharedCounter::new();
    /// counter.add(10);
    /// assert_eq!(counter.get(), 10);
    /// ```
    pub fn get(&self) -> i32 {
        *self.value.borrow()
    }

    /// Increments the counter by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SharedCounter;
    ///
    /// let counter = SharedCounter::new();
    /// counter.increment();
    /// assert_eq!(counter.get(), 1);
    /// ```
    pub fn increment(&self) {
        *self.value.borrow_mut() += 1;
    }

    /// Decrements the counter by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SharedCounter;
    ///
    /// let counter = SharedCounter::new();
    /// counter.add(5);
    /// counter.decrement();
    /// assert_eq!(counter.get(), 4);
    /// ```
    pub fn decrement(&self) {
        *self.value.borrow_mut() -= 1;
    }

    /// Adds n to the counter.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to add
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::SharedCounter;
    ///
    /// let counter = SharedCounter::new();
    /// counter.add(100);
    /// assert_eq!(counter.get(), 100);
    /// ```
    pub fn add(&self, n: i32) {
        *self.value.borrow_mut() += n;
    }
}

/// A tree node with interior mutability for children.
///
/// Combines `Rc` for shared ownership with `RefCell` for mutable children list.
/// This allows adding children through shared references.
///
/// # Examples
///
/// ```
/// use cell_refcell::TreeNode;
/// use std::rc::Rc;
///
/// let root = TreeNode::new("root");
/// let child = TreeNode::new("child");
/// root.add_child(Rc::clone(&child));
/// assert_eq!(root.children_count(), 1);
/// ```
#[derive(Debug)]
pub struct TreeNode<T> {
    value: RefCell<T>,
    children: RefCell<Vec<Rc<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    /// Creates a new tree node wrapped in Rc.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in the node
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::TreeNode;
    ///
    /// let node = TreeNode::new(42);
    /// assert_eq!(*node.value(), 42);
    /// ```
    pub fn new(value: T) -> Rc<Self> {
        Rc::new(TreeNode {
            value: RefCell::new(value),
            children: RefCell::new(Vec::new()),
        })
    }

    /// Gets a reference to the node's value.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::TreeNode;
    ///
    /// let node = TreeNode::new("hello");
    /// assert_eq!(*node.value(), "hello");
    /// ```
    pub fn value(&self) -> Ref<'_, T> {
        self.value.borrow()
    }

    /// Sets a new value for the node.
    ///
    /// # Arguments
    ///
    /// * `value` - The new value to set
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::TreeNode;
    ///
    /// let node = TreeNode::new(42);
    /// node.set_value(100);
    /// assert_eq!(*node.value(), 100);
    /// ```
    pub fn set_value(&self, value: T) {
        *self.value.borrow_mut() = value;
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
    /// use cell_refcell::TreeNode;
    /// use std::rc::Rc;
    ///
    /// let parent = TreeNode::new("parent");
    /// let child = TreeNode::new("child");
    /// parent.add_child(child);
    /// assert_eq!(parent.children_count(), 1);
    /// ```
    pub fn add_child(&self, child: Rc<TreeNode<T>>) {
        self.children.borrow_mut().push(child);
    }

    /// Returns the number of children.
    ///
    /// # Examples
    ///
    /// ```
    /// use cell_refcell::TreeNode;
    ///
    /// let node = TreeNode::new(42);
    /// assert_eq!(node.children_count(), 0);
    /// ```
    pub fn children_count(&self) -> usize {
        self.children.borrow().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_basic() {
        let counter = Counter::new();
        assert_eq!(counter.get(), 0);
        counter.increment();
        assert_eq!(counter.get(), 1);
    }

    #[test]
    fn test_cached_value_basic() {
        let cached = CachedValue::new(42);
        assert_eq!(cached.get(), 42);
        assert_eq!(cached.access_count(), 1);
    }

    #[test]
    fn test_shared_string_basic() {
        let shared = SharedString::new("Hello");
        shared.append(", World!");
        assert_eq!(shared.get(), "Hello, World!");
    }

    #[test]
    fn test_shared_vec_basic() {
        let vec: SharedVec<i32> = SharedVec::new();
        vec.push(1);
        vec.push(2);
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn test_safe_cell_basic() {
        let cell = SafeCell::new(42);
        let result = cell.with_value(|v| *v * 2);
        assert_eq!(result, Some(84));
    }

    #[test]
    fn test_shared_counter_basic() {
        let counter = SharedCounter::new();
        let counter2 = Rc::clone(&counter);
        counter.increment();
        counter2.increment();
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_tree_node_basic() {
        let root = TreeNode::new("root");
        let child = TreeNode::new("child");
        root.add_child(child);
        assert_eq!(root.children_count(), 1);
    }
}
