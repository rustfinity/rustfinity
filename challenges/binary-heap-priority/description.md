`BinaryHeap` is Rust's implementation of a priority queue backed by a binary max-heap. Unlike a regular queue where elements are processed in first-in-first-out (FIFO) order, a priority queue processes elements based on their priority - the highest priority element is always served first. In Rust's `BinaryHeap`, the "highest priority" means the largest element according to the `Ord` trait.

Priority queues are essential in many algorithms and real-world applications: task schedulers that need to process urgent tasks first, Dijkstra's shortest path algorithm, event-driven simulations, finding the top-K elements from a stream, and more. The binary heap data structure provides O(log n) insertion and removal of the maximum element, and O(1) access to the maximum element.

By default, `BinaryHeap<T>` is a max-heap - the largest element has the highest priority. If you need a min-heap (where the smallest element has highest priority), you can use `std::cmp::Reverse` to wrap your elements, which reverses the ordering.

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// Max-heap: largest element comes out first
let mut max_heap = BinaryHeap::new();
max_heap.push(3);
max_heap.push(1);
max_heap.push(4);
max_heap.push(1);
max_heap.push(5);

assert_eq!(max_heap.pop(), Some(5));  // Largest first
assert_eq!(max_heap.pop(), Some(4));
assert_eq!(max_heap.pop(), Some(3));

// Min-heap using Reverse: smallest element comes out first
let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
min_heap.push(Reverse(3));
min_heap.push(Reverse(1));
min_heap.push(Reverse(4));

assert_eq!(min_heap.pop(), Some(Reverse(1)));  // Smallest first
assert_eq!(min_heap.pop(), Some(Reverse(3)));
```

## Your Task

Implement the following functions that demonstrate various `BinaryHeap` operations:

1. `create_max_heap(items: &[i32]) -> BinaryHeap<i32>` - Create a max-heap from a slice of integers.

2. `create_min_heap(items: &[i32]) -> BinaryHeap<Reverse<i32>>` - Create a min-heap from a slice of integers using `Reverse`.

3. `pop_max(heap: &mut BinaryHeap<i32>) -> Option<i32>` - Remove and return the maximum element.

4. `peek_max(heap: &BinaryHeap<i32>) -> Option<&i32>` - Return a reference to the maximum element without removing it.

5. `top_k_largest(items: &[i32], k: usize) -> Vec<i32>` - Return the k largest elements in descending order. If k is greater than the number of items, return all items sorted in descending order.

6. `top_k_smallest(items: &[i32], k: usize) -> Vec<i32>` - Return the k smallest elements in ascending order. If k is greater than the number of items, return all items sorted in ascending order.

7. `merge_heaps(heap1: BinaryHeap<i32>, heap2: BinaryHeap<i32>) -> BinaryHeap<i32>` - Merge two heaps into one.

8. `heap_sort_descending(items: &[i32]) -> Vec<i32>` - Sort items in descending order using a BinaryHeap.

## Examples

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// create_max_heap
let max_heap = create_max_heap(&[3, 1, 4, 1, 5]);
assert_eq!(max_heap.peek(), Some(&5));

// create_min_heap
let min_heap = create_min_heap(&[3, 1, 4, 1, 5]);
assert_eq!(min_heap.peek(), Some(&Reverse(1)));

// pop_max
let mut heap = create_max_heap(&[3, 1, 4]);
assert_eq!(pop_max(&mut heap), Some(4));
assert_eq!(pop_max(&mut heap), Some(3));

// peek_max
let heap = create_max_heap(&[10, 5, 20]);
assert_eq!(peek_max(&heap), Some(&20));

// top_k_largest
assert_eq!(
    top_k_largest(&[3, 1, 4, 1, 5, 9, 2, 6], 3),
    vec![9, 6, 5]
);

// top_k_smallest
assert_eq!(
    top_k_smallest(&[3, 1, 4, 1, 5, 9, 2, 6], 3),
    vec![1, 1, 2]
);

// merge_heaps
let heap1 = create_max_heap(&[1, 3, 5]);
let heap2 = create_max_heap(&[2, 4, 6]);
let merged = merge_heaps(heap1, heap2);
assert_eq!(merged.len(), 6);
assert_eq!(merged.peek(), Some(&6));

// heap_sort_descending
assert_eq!(
    heap_sort_descending(&[3, 1, 4, 1, 5]),
    vec![5, 4, 3, 1, 1]
);
```

## Hints

<details>
  <summary>Click here for hints</summary>

- `BinaryHeap::from(slice)` or `slice.iter().copied().collect()` can create a heap from a slice
- For min-heap, wrap values in `Reverse(value)` when inserting, and unwrap with `.0` when retrieving
- `heap.pop()` removes and returns the maximum (or minimum for min-heap wrapped in Reverse)
- `heap.peek()` returns `Option<&T>` for viewing without removing
- For `top_k_largest`, you can pop k elements from a max-heap
- For `top_k_smallest`, use a min-heap (with `Reverse`) and pop k elements, then extract the inner values
- To merge heaps, you can use `.append()` which drains one heap into another, or extend one heap with the other's iterator
- `BinaryHeap::into_sorted_vec()` returns elements in ascending order - you may need to reverse for descending

</details>
