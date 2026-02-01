# Cell and RefCell: Interior Mutability

Rust's ownership and borrowing rules are enforced at compile time, ensuring memory safety without a garbage collector. However, there are situations where you need to mutate data even when there are immutable references to it. This pattern is called **interior mutability**.

The standard library provides two key types for interior mutability:

- **`Cell<T>`**: Provides interior mutability for `Copy` types. It moves values in and out rather than borrowing, making it zero-cost but limited to types that implement `Copy`.
- **`RefCell<T>`**: Provides interior mutability for any type by tracking borrows at runtime. It enforces borrowing rules dynamically and will panic if you violate them (e.g., two mutable borrows at once).

## When to Use Each

Use **`Cell<T>`** when:

- Working with `Copy` types (integers, booleans, small structs)
- You need simple get/set operations
- You want zero runtime overhead

Use **`RefCell<T>`** when:

- Working with non-`Copy` types (String, Vec, etc.)
- You need to borrow the inner value (get a reference)
- You're okay with runtime borrow checking

## Key Operations

**Cell<T>:**

- `Cell::new(value)` - Create a new Cell
- `cell.get()` - Get a copy of the value (requires `T: Copy`)
- `cell.set(value)` - Set a new value
- `cell.replace(value)` - Set value and return old value
- `cell.take()` - Take the value, leaving `Default::default()` (requires `T: Default`)

**RefCell<T>:**

- `RefCell::new(value)` - Create a new RefCell
- `refcell.borrow()` - Get immutable reference (`Ref<T>`), panics if mutably borrowed
- `refcell.borrow_mut()` - Get mutable reference (`RefMut<T>`), panics if any borrows exist
- `refcell.try_borrow()` - Returns `Result<Ref<T>, BorrowError>`
- `refcell.try_borrow_mut()` - Returns `Result<RefMut<T>, BorrowMutError>`

## Your Task

Implement the following types and functions to demonstrate interior mutability patterns:

### Part 1: Cell Basics

1. **`Counter`** - A simple counter using Cell:
   - `new() -> Self` - Create counter with value 0
   - `get(&self) -> i32` - Get current count
   - `set(&self, value: i32)` - Set the count
   - `increment(&self)` - Increment by 1
   - `decrement(&self)` - Decrement by 1

2. **`CachedValue<T: Copy + Default>`** - A value that tracks access count:
   - `new(value: T) -> Self` - Create with initial value
   - `get(&self) -> T` - Get value (increments access count)
   - `set(&self, value: T)` - Set value (resets access count)
   - `access_count(&self) -> u32` - Get number of times value was accessed

### Part 2: RefCell Basics

3. **`SharedString`** - A string that can be modified through shared references:
   - `new(s: &str) -> Self` - Create with initial string
   - `get(&self) -> String` - Get a clone of the string
   - `set(&self, s: &str)` - Set a new string
   - `append(&self, s: &str)` - Append to the string
   - `len(&self) -> usize` - Get string length
   - `is_empty(&self) -> bool` - Check if empty

4. **`SharedVec<T>`** - A vector with interior mutability:
   - `new() -> Self` - Create empty vector
   - `push(&self, value: T)` - Add element
   - `pop(&self) -> Option<T>` - Remove and return last element
   - `len(&self) -> usize` - Get length
   - `is_empty(&self) -> bool` - Check if empty
   - `get(&self, index: usize) -> Option<T>` where `T: Clone` - Get element at index

### Part 3: Safe Borrowing with try_borrow

5. **`SafeCell<T>`** - A RefCell wrapper with safe borrowing:
   - `new(value: T) -> Self` - Create with initial value
   - `try_read(&self) -> Option<std::cell::Ref<T>>` - Try to borrow immutably
   - `try_write(&self) -> Option<std::cell::RefMut<T>>` - Try to borrow mutably
   - `is_borrowed(&self) -> bool` - Check if currently borrowed (any type)
   - `with_value<F, R>(&self, f: F) -> Option<R>` where `F: FnOnce(&T) -> R` - Apply function if not borrowed
   - `with_value_mut<F, R>(&self, f: F) -> Option<R>` where `F: FnOnce(&mut T) -> R` - Apply mutating function

### Part 4: Combining Rc and RefCell

6. **`SharedCounter`** - A reference-counted counter with interior mutability:
   - `new() -> Rc<Self>` - Create new counter wrapped in Rc
   - `get(&self) -> i32` - Get current value
   - `increment(&self)` - Increment by 1
   - `decrement(&self)` - Decrement by 1
   - `add(&self, n: i32)` - Add n to counter

7. **`TreeNode<T>`** - A tree node where children can be added through shared references:
   - `new(value: T) -> Rc<Self>` - Create leaf node
   - `value(&self) -> std::cell::Ref<T>` - Get reference to value
   - `set_value(&self, value: T)` - Set new value
   - `add_child(&self, child: Rc<TreeNode<T>>)` - Add a child node
   - `children_count(&self) -> usize` - Get number of children

## Examples

```rust
use cell_refcell::*;
use std::rc::Rc;

// Cell-based Counter
let counter = Counter::new();
counter.increment();
counter.increment();
assert_eq!(counter.get(), 2);

// CachedValue with access tracking
let cached = CachedValue::new(42);
assert_eq!(cached.get(), 42);
assert_eq!(cached.get(), 42);
assert_eq!(cached.access_count(), 2);

// RefCell-based SharedString
let shared = SharedString::new("Hello");
shared.append(", World!");
assert_eq!(shared.get(), "Hello, World!");

// SharedVec
let vec: SharedVec<i32> = SharedVec::new();
vec.push(1);
vec.push(2);
assert_eq!(vec.pop(), Some(2));
assert_eq!(vec.len(), 1);

// SafeCell with try_borrow
let safe = SafeCell::new(42);
let result = safe.with_value(|v| *v * 2);
assert_eq!(result, Some(84));

// Rc<RefCell> pattern with SharedCounter
let counter = SharedCounter::new();
let counter2 = Rc::clone(&counter);
counter.increment();
counter2.increment();
assert_eq!(counter.get(), 2);

// Tree with interior mutability
let root = TreeNode::new("root");
let child1 = TreeNode::new("child1");
root.add_child(child1);
assert_eq!(root.children_count(), 1);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- `Cell<T>` uses `.get()` and `.set()` - no borrowing, just copying values
- `RefCell<T>` uses `.borrow()` for `&T` and `.borrow_mut()` for `&mut T`
- `try_borrow()` and `try_borrow_mut()` return `Result` instead of panicking
- When combining `Rc<RefCell<T>>`, use `Rc::clone()` for sharing and `borrow()/borrow_mut()` for access
- `Ref<T>` and `RefMut<T>` implement `Deref` and `DerefMut`, so you can use them like regular references
- For `SafeCell::is_borrowed()`, you can try to borrow and see if it succeeds
- Remember that `RefCell` will panic on invalid borrows - use `try_*` methods when unsure
- The `TreeNode` pattern (`Rc<RefCell<Vec<Rc<TreeNode>>>>`) is common for tree structures in Rust

</details>
