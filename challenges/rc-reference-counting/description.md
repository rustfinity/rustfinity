# Rc and Reference Counting

In Rust, ownership rules dictate that each value has a single owner, and when the owner goes out of scope, the value is dropped. But what if you need multiple parts of your program to share ownership of the same data? This is where `Rc<T>` (Reference Counted) comes in.

`Rc<T>` is a smart pointer that enables **shared ownership** of a value. It keeps track of how many references (or "owners") exist to a value, and only deallocates the value when the last reference is dropped. This is called **reference counting**.

`Rc<T>` is useful in scenarios like:
- Graph data structures where nodes may have multiple incoming edges
- Caching systems where multiple components need access to the same data
- Event systems where multiple listeners observe the same source

**Important**: `Rc<T>` is only for single-threaded scenarios. For multi-threaded code, use `Arc<T>` (Atomic Reference Counted) instead.

## Key Operations

- `Rc::new(value)` - Create a new reference-counted pointer
- `Rc::clone(&rc)` - Create a new reference (increments count, doesn't deep clone)
- `Rc::strong_count(&rc)` - Get the current number of strong references
- `Rc::weak_count(&rc)` - Get the current number of weak references
- `Rc::downgrade(&rc)` - Create a `Weak<T>` reference that doesn't prevent deallocation
- `weak.upgrade()` - Try to convert a `Weak<T>` back to `Rc<T>` (returns `Option`)

## Your Task

Implement the following types and functions to demonstrate `Rc<T>` patterns:

### Part 1: Basic Rc Operations

1. **`create_shared<T>(value: T) -> Rc<T>`** - Create a new reference-counted value
2. **`clone_shared<T>(rc: &Rc<T>) -> Rc<T>`** - Clone a reference (increment count)
3. **`get_strong_count<T>(rc: &Rc<T>) -> usize`** - Get the strong reference count
4. **`get_value<T: Clone>(rc: &Rc<T>) -> T`** - Get a clone of the inner value

### Part 2: Shared Data Structure

5. **`SharedBuffer`** - A buffer that can be shared across multiple owners:
   - `new(data: Vec<u8>) -> Rc<Self>` - Create a new shared buffer
   - `len(&self) -> usize` - Get the buffer length
   - `get(&self, index: usize) -> Option<u8>` - Get a byte at index
   - `as_slice(&self) -> &[u8]` - Get the buffer as a slice

### Part 3: Weak References

6. **`create_weak<T>(rc: &Rc<T>) -> Weak<T>`** - Create a weak reference
7. **`upgrade_weak<T>(weak: &Weak<T>) -> Option<Rc<T>>`** - Try to upgrade a weak reference
8. **`get_weak_count<T>(rc: &Rc<T>) -> usize`** - Get the weak reference count

### Part 4: Graph Node with Rc

9. **`Node<T>`** - A graph node that can have multiple parents sharing child nodes:
   - `new(value: T) -> Rc<Self>` - Create a new node
   - `value(&self) -> &T` - Get reference to the value
   - `add_child(&mut self, child: Rc<Node<T>>)` - Add a child node
   - `children(&self) -> &[Rc<Node<T>>]` - Get all children

### Part 5: Observer Pattern

10. **`Observable<T>`** - A value with weak observers:
    - `new(value: T) -> Self` - Create observable with initial value
    - `get(&self) -> &T` - Get reference to value
    - `set(&mut self, value: T)` - Set new value
    - `subscribe(&mut self, observer: Rc<dyn Observer<T>>)` - Add an observer (stored as Weak)
    - `notify(&self)` - Notify all live observers

11. **`Observer<T>` trait**:
    - `on_update(&self, value: &T)` - Called when observable changes

## Examples

```rust
use rc_reference_counting::*;
use std::rc::Rc;

// Basic Rc operations
let shared = create_shared(42);
assert_eq!(get_strong_count(&shared), 1);

let cloned = clone_shared(&shared);
assert_eq!(get_strong_count(&shared), 2);
assert_eq!(get_value(&shared), 42);

// Shared buffer
let buffer = SharedBuffer::new(vec![1, 2, 3, 4, 5]);
assert_eq!(buffer.len(), 5);
assert_eq!(buffer.get(2), Some(3));

// Weak references
let strong = create_shared(String::from("hello"));
let weak = create_weak(&strong);
assert_eq!(get_weak_count(&strong), 1);

if let Some(upgraded) = upgrade_weak(&weak) {
    assert_eq!(*upgraded, "hello");
}

drop(strong);
assert!(upgrade_weak(&weak).is_none()); // Value was deallocated
```

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `Rc::new()` to wrap values for shared ownership
- `Rc::clone(&rc)` is cheap - it only increments a counter, doesn't copy data
- `Rc::downgrade()` creates a `Weak<T>` that doesn't count toward keeping the value alive
- `Weak::upgrade()` returns `Option<Rc<T>>` - `None` if the value was already dropped
- For `Node` with mutable children, you might need `RefCell` inside the node (interior mutability)
- Weak references are perfect for observer patterns to avoid memory leaks from circular references
- Remember: `Rc<T>` only works in single-threaded code!

</details>
