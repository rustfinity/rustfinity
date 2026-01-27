/// Creates a boxed value on the heap.
///
/// Takes any value and allocates it on the heap, returning a `Box<T>`.
///
/// # Arguments
///
/// * `value` - The value to box
///
/// # Examples
///
/// ```
/// use box_heap_allocation::boxed_value;
///
/// let boxed = boxed_value(42);
/// assert_eq!(*boxed, 42);
///
/// let boxed_string = boxed_value(String::from("hello"));
/// assert_eq!(&*boxed_string, "hello");
/// ```
pub fn boxed_value<T>(value: T) -> Box<T> {
    Box::new(value)
}

/// Unboxes a value, returning the inner value.
///
/// Takes ownership of a `Box<T>` and returns the inner value.
///
/// # Arguments
///
/// * `boxed` - The boxed value to unbox
///
/// # Examples
///
/// ```
/// use box_heap_allocation::unbox;
///
/// let boxed = Box::new(42);
/// let value = unbox(boxed);
/// assert_eq!(value, 42);
/// ```
pub fn unbox<T>(boxed: Box<T>) -> T {
    *boxed
}

/// A recursive linked list demonstrating heap allocation for recursive types.
///
/// Without `Box`, this type would have infinite size because `List` contains
/// `List` directly. Using `Box` gives it a known size (the size of a pointer).
#[derive(Debug, Clone, PartialEq)]
pub enum List {
    /// End of the list
    Nil,
    /// A node containing a value and pointer to the next node
    Cons(i32, Box<List>),
}

impl List {
    /// Creates an empty list.
    ///
    /// # Examples
    ///
    /// ```
    /// use box_heap_allocation::List;
    ///
    /// let list = List::new();
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn new() -> List {
        List::Nil
    }

    /// Adds a value to the front of the list.
    ///
    /// Consumes the list and returns a new list with the value prepended.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add to the front
    ///
    /// # Examples
    ///
    /// ```
    /// use box_heap_allocation::List;
    ///
    /// let list = List::new().prepend(3).prepend(2).prepend(1);
    /// assert_eq!(list.to_vec(), vec![1, 2, 3]);
    /// ```
    pub fn prepend(self, value: i32) -> List {
        List::Cons(value, Box::new(self))
    }

    /// Returns the number of elements in the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use box_heap_allocation::List;
    ///
    /// let list = List::new().prepend(1).prepend(2);
    /// assert_eq!(list.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, tail) => 1 + tail.len(),
        }
    }

    /// Returns true if the list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use box_heap_allocation::List;
    ///
    /// assert!(List::new().is_empty());
    /// assert!(!List::new().prepend(1).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        matches!(self, List::Nil)
    }

    /// Returns the sum of all elements in the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use box_heap_allocation::List;
    ///
    /// let list = List::new().prepend(3).prepend(2).prepend(1);
    /// assert_eq!(list.sum(), 6);
    /// ```
    pub fn sum(&self) -> i32 {
        match self {
            List::Nil => 0,
            List::Cons(value, tail) => value + tail.sum(),
        }
    }

    /// Converts the list to a Vec, preserving order.
    ///
    /// # Examples
    ///
    /// ```
    /// use box_heap_allocation::List;
    ///
    /// let list = List::new().prepend(3).prepend(2).prepend(1);
    /// assert_eq!(list.to_vec(), vec![1, 2, 3]);
    /// ```
    pub fn to_vec(&self) -> Vec<i32> {
        match self {
            List::Nil => Vec::new(),
            List::Cons(value, tail) => {
                let mut result = vec![*value];
                result.extend(tail.to_vec());
                result
            }
        }
    }
}

impl Default for List {
    fn default() -> Self {
        List::new()
    }
}

/// A large data structure demonstrating the benefits of heap allocation.
///
/// Stack space is limited, so large arrays should be boxed to avoid stack overflow.
#[derive(Debug, Clone, PartialEq)]
pub struct LargeData {
    pub data: [u8; 1000],
}

impl LargeData {
    /// Creates new LargeData filled with zeros.
    pub fn new() -> Self {
        LargeData { data: [0; 1000] }
    }

    /// Creates new LargeData filled with the given value.
    pub fn filled(value: u8) -> Self {
        LargeData { data: [value; 1000] }
    }
}

impl Default for LargeData {
    fn default() -> Self {
        LargeData::new()
    }
}

/// Boxes a large data structure.
///
/// This is useful when you want to transfer ownership of large data without
/// copying it on the stack.
///
/// # Arguments
///
/// * `data` - The large data structure to box
///
/// # Examples
///
/// ```
/// use box_heap_allocation::{LargeData, box_large_data};
///
/// let large = LargeData::filled(42);
/// let boxed = box_large_data(large);
/// assert_eq!(boxed.data[0], 42);
/// ```
pub fn box_large_data(data: LargeData) -> Box<LargeData> {
    Box::new(data)
}

/// Modifies a boxed value in place using a closure.
///
/// This demonstrates that `Box<T>` implements `DerefMut`, allowing
/// mutable access to the inner value.
///
/// # Arguments
///
/// * `boxed` - A mutable reference to the boxed value
/// * `f` - A closure that modifies the inner value
///
/// # Examples
///
/// ```
/// use box_heap_allocation::modify_boxed;
///
/// let mut boxed = Box::new(10);
/// modify_boxed(&mut boxed, |n| *n += 5);
/// assert_eq!(*boxed, 15);
/// ```
pub fn modify_boxed<T, F>(boxed: &mut Box<T>, f: F)
where
    F: FnOnce(&mut T),
{
    f(boxed.as_mut())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boxed_value_integer() {
        let boxed = boxed_value(42);
        assert_eq!(*boxed, 42);
    }

    #[test]
    fn test_unbox_integer() {
        let boxed = Box::new(100);
        assert_eq!(unbox(boxed), 100);
    }

    #[test]
    fn test_list_empty() {
        let list = List::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_list_prepend() {
        let list = List::new().prepend(1).prepend(2).prepend(3);
        assert_eq!(list.len(), 3);
        assert_eq!(list.to_vec(), vec![3, 2, 1]);
    }
}
