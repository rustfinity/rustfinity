# Box and Heap Allocation

In Rust, most values are stored on the stack by default. The stack is fast and efficient, but it has limitations: all values must have a known, fixed size at compile time, and the stack has limited space. When you need to store data with a size unknown at compile time, or when you have large data that you don't want to copy, you need heap allocation.

`Box<T>` is Rust's simplest smart pointer for heap allocation. When you create a `Box<T>`, the value of type `T` is allocated on the heap, and the `Box` itself (which is just a pointer) lives on the stack. When the `Box` goes out of scope, both the pointer and the heap data are deallocated automatically thanks to Rust's ownership system.

Common use cases for `Box<T>` include:

- Storing large data without copying it when transferring ownership
- Enabling recursive types (types that contain themselves)
- Trait objects when you need dynamic dispatch (`Box<dyn Trait>`)
- Transferring ownership of data without copying it

## Your Task

Implement the following types and functions that demonstrate heap allocation with `Box<T>`:

1. **`boxed_value<T>(value: T) -> Box<T>`** - A generic function that takes any value and returns it boxed on the heap.

2. **`unbox<T>(boxed: Box<T>) -> T`** - A generic function that takes a boxed value and returns the inner value, moving it out of the box.

3. **`List` enum** - A recursive linked list type:
   - `Nil` - represents the end of the list
   - `Cons(i32, Box<List>)` - a node containing a value and a pointer to the next node

4. **`List::new() -> List`** - Creates an empty list (Nil).

5. **`List::prepend(self, value: i32) -> List`** - Adds a value to the front of the list, returning a new list.

6. **`List::len(&self) -> usize`** - Returns the number of elements in the list.

7. **`List::sum(&self) -> i32`** - Returns the sum of all elements in the list.

8. **`List::to_vec(&self) -> Vec<i32>`** - Converts the list to a Vec, preserving order.

9. **`LargeData` struct** - A struct with a large array `data: [u8; 1000]` to demonstrate boxing large data.

10. **`box_large_data(data: LargeData) -> Box<LargeData>`** - Boxes a large data structure.

11. **`modify_boxed<T, F>(boxed: &mut Box<T>, f: F)`** - Takes a mutable reference to a boxed value and applies a function to modify the inner value. `F: FnOnce(&mut T)`.

## Examples

```rust
// Boxing and unboxing
let boxed = boxed_value(42);
assert_eq!(*boxed, 42);

let value = unbox(boxed);
assert_eq!(value, 42);

// Recursive list
let list = List::new()
    .prepend(3)
    .prepend(2)
    .prepend(1);

assert_eq!(list.len(), 3);
assert_eq!(list.sum(), 6);
assert_eq!(list.to_vec(), vec![1, 2, 3]);

// Boxing large data
let large = LargeData { data: [0; 1000] };
let boxed_large = box_large_data(large);
assert_eq!(boxed_large.data[0], 0);

// Modifying boxed values
let mut boxed_num = Box::new(10);
modify_boxed(&mut boxed_num, |n| *n += 5);
assert_eq!(*boxed_num, 15);
```

## Hints

<details>
  <summary>Click here to reveal hints</summary>

- Use `Box::new(value)` to create a boxed value
- The `*` operator dereferences a `Box` to access the inner value
- For recursive types like `List`, you must use `Box` to give the type a known size
- The `List::prepend` method should create a new `Cons` node with the current list boxed as the tail
- For `modify_boxed`, use `&mut **boxed` or `boxed.as_mut()` to get a mutable reference to the inner value
- Remember that `Box<T>` implements `Deref` and `DerefMut`, so you can often use it like a regular reference

</details>
