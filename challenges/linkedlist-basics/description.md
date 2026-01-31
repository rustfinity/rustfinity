`LinkedList` is Rust's implementation of a doubly-linked list. Unlike `Vec`, which stores elements contiguously in memory, a linked list stores each element in a separate node that contains pointers to the previous and next nodes. This makes insertions and removals at both ends O(1) operations, but random access becomes O(n) since you must traverse the list to reach a specific element.

In practice, `LinkedList` is rarely the best choice in Rust because `Vec` and `VecDeque` are more cache-friendly and often faster due to better memory locality. However, `LinkedList` shines in specific scenarios: when you need to frequently insert or remove elements in the middle of the list (if you already have a cursor at that position), when you need to split or merge lists efficiently, or when implementing certain algorithms like LRU caches where you need to move elements to the front or back without shifting other elements.

The `LinkedList` API includes operations at both ends: `push_front()` and `push_back()` for adding elements, `pop_front()` and `pop_back()` for removing elements. It also supports efficient list manipulation through `append()` to concatenate lists and `split_off()` to divide a list at a given index.

```rust
use std::collections::LinkedList;

let mut list = LinkedList::new();

// Add elements at both ends
list.push_back(1);
list.push_back(2);
list.push_front(0);
// List is now: [0, 1, 2]

// Remove from both ends
assert_eq!(list.pop_front(), Some(0));
assert_eq!(list.pop_back(), Some(2));
// List is now: [1]

// Peek without removing
assert_eq!(list.front(), Some(&1));
assert_eq!(list.back(), Some(&1));
```

## Your Task

Implement the following functions that demonstrate various `LinkedList` operations:

1. `create_list(items: &[i32]) -> LinkedList<i32>` - Create a linked list from a slice, maintaining the order of elements.

2. `add_front<T>(list: &mut LinkedList<T>, item: T)` - Add an element to the front of the list.

3. `add_back<T>(list: &mut LinkedList<T>, item: T)` - Add an element to the back of the list.

4. `remove_front<T>(list: &mut LinkedList<T>) -> Option<T>` - Remove and return the front element.

5. `remove_back<T>(list: &mut LinkedList<T>) -> Option<T>` - Remove and return the back element.

6. `peek_front<T>(list: &LinkedList<T>) -> Option<&T>` - Return a reference to the front element without removing it.

7. `peek_back<T>(list: &LinkedList<T>) -> Option<&T>` - Return a reference to the back element without removing it.

8. `move_to_front<T: PartialEq>(list: &mut LinkedList<T>, value: &T) -> bool` - Find an element by value and move it to the front. Returns `true` if the element was found and moved, `false` otherwise. This is a key operation for LRU cache behavior.

9. `concat_lists<T>(list1: LinkedList<T>, list2: LinkedList<T>) -> LinkedList<T>` - Concatenate two lists, with list2's elements appended after list1's.

## Examples

```rust
use std::collections::LinkedList;

// create_list
let list = create_list(&[1, 2, 3]);
assert_eq!(list.front(), Some(&1));
assert_eq!(list.back(), Some(&3));
assert_eq!(list.len(), 3);

// add_front / add_back
let mut list = LinkedList::new();
add_back(&mut list, 2);
add_front(&mut list, 1);
add_back(&mut list, 3);
// List is: [1, 2, 3]

// remove_front / remove_back
let mut list = create_list(&[1, 2, 3]);
assert_eq!(remove_front(&mut list), Some(1));
assert_eq!(remove_back(&mut list), Some(3));
// List is: [2]

// peek_front / peek_back
let list = create_list(&[10, 20, 30]);
assert_eq!(peek_front(&list), Some(&10));
assert_eq!(peek_back(&list), Some(&30));

// move_to_front (LRU-style operation)
let mut list = create_list(&[1, 2, 3, 4, 5]);
// Move 3 to front
assert!(move_to_front(&mut list, &3));
// List is now: [3, 1, 2, 4, 5]
// 99 not found
assert!(!move_to_front(&mut list, &99));

// concat_lists
let list1 = create_list(&[1, 2]);
let list2 = create_list(&[3, 4]);
let combined = concat_lists(list1, list2);
// combined is: [1, 2, 3, 4]
```

## Hints

<details>
  <summary>Click here for hints</summary>

- `LinkedList::new()` creates an empty list
- Use `.extend()` or iterate and push to create a list from a slice
- `push_front()` and `push_back()` add to the respective ends
- `pop_front()` and `pop_back()` remove from the respective ends
- `front()` and `back()` return `Option<&T>` for peeking
- For `move_to_front`, you'll need to remove the element and re-add it at the front
- Since LinkedList doesn't have random access, you may need to rebuild it to remove an element from the middle
- `append()` drains one list into another - be careful about ownership

</details>
