/// Creates a boxed value on the heap.
///
/// Takes any value and allocates it on the heap, returning a `Box<T>`.
pub fn boxed_value<T>(value: T) -> Box<T> {
    // TODO: Use Box::new to allocate the value on the heap
    unimplemented!()
}

/// Unboxes a value, returning the inner value.
///
/// Takes ownership of a `Box<T>` and returns the inner value.
pub fn unbox<T>(boxed: Box<T>) -> T {
    // TODO: Dereference the box to get the inner value
    unimplemented!()
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
    // TODO: Define the Cons variant with an i32 and a Box<List>
}

impl List {
    /// Creates an empty list.
    pub fn new() -> List {
        // TODO: Return an empty list (Nil)
        unimplemented!()
    }

    /// Adds a value to the front of the list.
    ///
    /// Consumes the list and returns a new list with the value prepended.
    pub fn prepend(self, value: i32) -> List {
        // TODO: Create a Cons node with the value and box the current list as the tail
        unimplemented!()
    }

    /// Returns the number of elements in the list.
    pub fn len(&self) -> usize {
        // TODO: Recursively count elements
        unimplemented!()
    }

    /// Returns true if the list is empty.
    pub fn is_empty(&self) -> bool {
        // TODO: Check if this is Nil
        unimplemented!()
    }

    /// Returns the sum of all elements in the list.
    pub fn sum(&self) -> i32 {
        // TODO: Recursively sum elements
        unimplemented!()
    }

    /// Converts the list to a Vec, preserving order.
    pub fn to_vec(&self) -> Vec<i32> {
        // TODO: Convert the list to a vector
        unimplemented!()
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
pub fn box_large_data(data: LargeData) -> Box<LargeData> {
    // TODO: Box the large data structure
    unimplemented!()
}

/// Modifies a boxed value in place using a closure.
///
/// This demonstrates that `Box<T>` implements `DerefMut`, allowing
/// mutable access to the inner value.
pub fn modify_boxed<T, F>(boxed: &mut Box<T>, f: F)
where
    F: FnOnce(&mut T),
{
    // TODO: Apply the closure to the inner value
    unimplemented!()
}

// Example usage
pub fn main() {
    // Boxing and unboxing
    let boxed = boxed_value(42);
    println!("Boxed value: {}", *boxed);

    let value = unbox(boxed);
    println!("Unboxed value: {}", value);

    // Recursive list
    let list = List::new().prepend(3).prepend(2).prepend(1);
    println!("List length: {}", list.len());
    println!("List sum: {}", list.sum());
    println!("List as vec: {:?}", list.to_vec());

    // Boxing large data
    let large = LargeData::filled(42);
    let boxed_large = box_large_data(large);
    println!("First byte of large data: {}", boxed_large.data[0]);

    // Modifying boxed values
    let mut boxed_num = Box::new(10);
    modify_boxed(&mut boxed_num, |n| *n += 5);
    println!("Modified boxed value: {}", *boxed_num);
}
