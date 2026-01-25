use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Creates a max-heap from a slice of integers.
///
/// # Arguments
///
/// * `items` - A slice of integers to add to the heap
///
/// # Returns
///
/// A BinaryHeap containing all the items
///
/// # Examples
///
/// ```
/// use binary_heap_priority::create_max_heap;
///
/// let heap = create_max_heap(&[3, 1, 4, 1, 5]);
/// assert_eq!(heap.peek(), Some(&5));
/// assert_eq!(heap.len(), 5);
/// ```
pub fn create_max_heap(items: &[i32]) -> BinaryHeap<i32> {
    items.iter().copied().collect()
}

/// Creates a min-heap from a slice of integers using Reverse.
///
/// # Arguments
///
/// * `items` - A slice of integers to add to the heap
///
/// # Returns
///
/// A BinaryHeap of Reverse<i32> where the smallest element has highest priority
///
/// # Examples
///
/// ```
/// use binary_heap_priority::create_min_heap;
/// use std::cmp::Reverse;
///
/// let heap = create_min_heap(&[3, 1, 4, 1, 5]);
/// assert_eq!(heap.peek(), Some(&Reverse(1)));
/// assert_eq!(heap.len(), 5);
/// ```
pub fn create_min_heap(items: &[i32]) -> BinaryHeap<Reverse<i32>> {
    items.iter().copied().map(Reverse).collect()
}

/// Removes and returns the maximum element from the heap.
///
/// # Arguments
///
/// * `heap` - A mutable reference to the BinaryHeap
///
/// # Returns
///
/// Some(max) if the heap is not empty, None otherwise
///
/// # Examples
///
/// ```
/// use binary_heap_priority::{create_max_heap, pop_max};
///
/// let mut heap = create_max_heap(&[3, 1, 4]);
/// assert_eq!(pop_max(&mut heap), Some(4));
/// assert_eq!(pop_max(&mut heap), Some(3));
/// assert_eq!(pop_max(&mut heap), Some(1));
/// assert_eq!(pop_max(&mut heap), None);
/// ```
pub fn pop_max(heap: &mut BinaryHeap<i32>) -> Option<i32> {
    heap.pop()
}

/// Returns a reference to the maximum element without removing it.
///
/// # Arguments
///
/// * `heap` - A reference to the BinaryHeap
///
/// # Returns
///
/// Some(&max) if the heap is not empty, None otherwise
///
/// # Examples
///
/// ```
/// use binary_heap_priority::{create_max_heap, peek_max};
///
/// let heap = create_max_heap(&[10, 5, 20]);
/// assert_eq!(peek_max(&heap), Some(&20));
/// assert_eq!(heap.len(), 3); // Unchanged
/// ```
pub fn peek_max(heap: &BinaryHeap<i32>) -> Option<&i32> {
    heap.peek()
}

/// Returns the k largest elements in descending order.
///
/// If k is greater than the number of items, returns all items sorted descending.
///
/// # Arguments
///
/// * `items` - A slice of integers
/// * `k` - The number of largest elements to return
///
/// # Returns
///
/// A Vec containing the k largest elements in descending order
///
/// # Examples
///
/// ```
/// use binary_heap_priority::top_k_largest;
///
/// assert_eq!(top_k_largest(&[3, 1, 4, 1, 5, 9, 2, 6], 3), vec![9, 6, 5]);
/// assert_eq!(top_k_largest(&[5, 2, 8], 5), vec![8, 5, 2]); // k > len
/// ```
pub fn top_k_largest(items: &[i32], k: usize) -> Vec<i32> {
    let mut heap = create_max_heap(items);
    let mut result = Vec::with_capacity(k.min(items.len()));
    for _ in 0..k {
        match heap.pop() {
            Some(val) => result.push(val),
            None => break,
        }
    }
    result
}

/// Returns the k smallest elements in ascending order.
///
/// If k is greater than the number of items, returns all items sorted ascending.
///
/// # Arguments
///
/// * `items` - A slice of integers
/// * `k` - The number of smallest elements to return
///
/// # Returns
///
/// A Vec containing the k smallest elements in ascending order
///
/// # Examples
///
/// ```
/// use binary_heap_priority::top_k_smallest;
///
/// assert_eq!(top_k_smallest(&[3, 1, 4, 1, 5, 9, 2, 6], 3), vec![1, 1, 2]);
/// assert_eq!(top_k_smallest(&[5, 2, 8], 5), vec![2, 5, 8]); // k > len
/// ```
pub fn top_k_smallest(items: &[i32], k: usize) -> Vec<i32> {
    let mut heap = create_min_heap(items);
    let mut result = Vec::with_capacity(k.min(items.len()));
    for _ in 0..k {
        match heap.pop() {
            Some(Reverse(val)) => result.push(val),
            None => break,
        }
    }
    result
}

/// Merges two heaps into one.
///
/// # Arguments
///
/// * `heap1` - The first BinaryHeap
/// * `heap2` - The second BinaryHeap
///
/// # Returns
///
/// A new BinaryHeap containing all elements from both heaps
///
/// # Examples
///
/// ```
/// use binary_heap_priority::{create_max_heap, merge_heaps};
///
/// let heap1 = create_max_heap(&[1, 3, 5]);
/// let heap2 = create_max_heap(&[2, 4, 6]);
/// let merged = merge_heaps(heap1, heap2);
/// assert_eq!(merged.len(), 6);
/// assert_eq!(merged.peek(), Some(&6));
/// ```
pub fn merge_heaps(mut heap1: BinaryHeap<i32>, mut heap2: BinaryHeap<i32>) -> BinaryHeap<i32> {
    heap1.append(&mut heap2);
    heap1
}

/// Sorts items in descending order using a BinaryHeap.
///
/// # Arguments
///
/// * `items` - A slice of integers to sort
///
/// # Returns
///
/// A Vec containing all items sorted in descending order
///
/// # Examples
///
/// ```
/// use binary_heap_priority::heap_sort_descending;
///
/// assert_eq!(heap_sort_descending(&[3, 1, 4, 1, 5]), vec![5, 4, 3, 1, 1]);
/// assert_eq!(heap_sort_descending(&[]), vec![]);
/// ```
pub fn heap_sort_descending(items: &[i32]) -> Vec<i32> {
    let heap = create_max_heap(items);
    // into_sorted_vec returns ascending order, so we reverse
    let mut sorted = heap.into_sorted_vec();
    sorted.reverse();
    sorted
}
