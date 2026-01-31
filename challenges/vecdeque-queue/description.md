`VecDeque<T>` is Rust's implementation of a double-ended queue (deque) backed by a growable ring buffer. Unlike `Vec`, which only provides efficient operations at the back, `VecDeque` allows O(1) amortized insertion and removal at both ends. This makes it ideal for implementing queues, task schedulers, and sliding window algorithms.

The `VecDeque` type is found in `std::collections` and provides methods like `push_front()`, `push_back()`, `pop_front()`, and `pop_back()` for double-ended operations. It also supports indexed access, iteration, and rotation operations.

One key advantage of `VecDeque` over `Vec` is when you need FIFO (first-in-first-out) queue behavior. With `Vec`, removing from the front requires shifting all elements (O(n)), but `VecDeque` does this in O(1) time. This makes `VecDeque` the go-to choice for task queues, breadth-first search, and any algorithm that needs to efficiently add and remove from both ends.

```rust
use std::collections::VecDeque;

let mut queue: VecDeque<&str> = VecDeque::new();

// Add to the back (like a queue)
queue.push_back("first");
queue.push_back("second");
queue.push_back("third");

// Remove from the front (FIFO order)
assert_eq!(queue.pop_front(), Some("first"));
assert_eq!(queue.pop_front(), Some("second"));

// Can also add to front and remove from back
queue.push_front("new_first");
assert_eq!(queue.pop_back(), Some("third"));
```

## Your Task

Implement the following functions for working with `VecDeque`:

1. `create_queue(items: &[T]) -> VecDeque<T>` - Create a VecDeque from a slice, preserving order (first element of slice is front of queue)

2. `enqueue(queue: &mut VecDeque<T>, item: T)` - Add an item to the back of the queue

3. `dequeue(queue: &mut VecDeque<T>) -> Option<T>` - Remove and return the item from the front of the queue

4. `peek_front(queue: &VecDeque<T>) -> Option<&T>` - Return a reference to the front item without removing it

5. `peek_back(queue: &VecDeque<T>) -> Option<&T>` - Return a reference to the back item without removing it

6. `rotate_left(queue: &mut VecDeque<T>, n: usize)` - Rotate the queue so the front n elements move to the back

7. `rotate_right(queue: &mut VecDeque<T>, n: usize)` - Rotate the queue so the back n elements move to the front

## Examples

```rust
use std::collections::VecDeque;

// create_queue
let queue = create_queue(&[1, 2, 3]);
assert_eq!(queue.front(), Some(&1));
assert_eq!(queue.back(), Some(&3));

// enqueue and dequeue (FIFO behavior)
let mut queue = create_queue(&["first", "second"]);
enqueue(&mut queue, "third");
assert_eq!(
    dequeue(&mut queue),
    Some("first")
);
assert_eq!(
    dequeue(&mut queue),
    Some("second")
);
assert_eq!(
    dequeue(&mut queue),
    Some("third")
);

// peek_front and peek_back
let queue = create_queue(&[10, 20, 30]);
assert_eq!(peek_front(&queue), Some(&10));
assert_eq!(peek_back(&queue), Some(&30));

// rotate_left - front elements move to back
let mut queue = create_queue(&[1, 2, 3, 4, 5]);
rotate_left(&mut queue, 2);
// Now: [3, 4, 5, 1, 2]
assert_eq!(queue.front(), Some(&3));

// rotate_right - back elements move to front
let mut queue = create_queue(&[1, 2, 3, 4, 5]);
rotate_right(&mut queue, 2);
// Now: [4, 5, 1, 2, 3]
assert_eq!(queue.front(), Some(&4));
```

## Hints

<details>
  <summary>Click here for hints</summary>

- For `create_queue`, you can use `.iter().cloned().collect()` to collect from a slice into a VecDeque
- For `enqueue`, use the `.push_back()` method
- For `dequeue`, use the `.pop_front()` method which returns `Option<T>`
- For `peek_front` and `peek_back`, use `.front()` and `.back()` methods
- For `rotate_left`, use the `.rotate_left()` method on VecDeque - but be careful with n larger than the queue length!
- For `rotate_right`, use the `.rotate_right()` method - also handle edge cases with n
- When n is 0 or the queue is empty, rotation should be a no-op
- The Clone bound on T is needed for `create_queue` to clone elements from the slice

</details>
