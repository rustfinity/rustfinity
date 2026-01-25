use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Creates a max-heap from a slice of integers.
pub fn create_max_heap(items: &[i32]) -> BinaryHeap<i32> {
    // TODO: Create a BinaryHeap containing all items
    // Hint: You can use .iter().copied().collect() or BinaryHeap::from()
    todo!()
}

/// Creates a min-heap from a slice of integers using Reverse.
pub fn create_min_heap(items: &[i32]) -> BinaryHeap<Reverse<i32>> {
    // TODO: Create a min-heap by wrapping each item in Reverse
    // Hint: Map each item to Reverse(item) before collecting
    todo!()
}

/// Removes and returns the maximum element from the heap.
pub fn pop_max(heap: &mut BinaryHeap<i32>) -> Option<i32> {
    // TODO: Remove and return the maximum element
    // Hint: BinaryHeap has a pop() method
    todo!()
}

/// Returns a reference to the maximum element without removing it.
pub fn peek_max(heap: &BinaryHeap<i32>) -> Option<&i32> {
    // TODO: Return a reference to the max without removing
    // Hint: BinaryHeap has a peek() method
    todo!()
}

/// Returns the k largest elements in descending order.
/// If k > items.len(), return all items sorted descending.
pub fn top_k_largest(items: &[i32], k: usize) -> Vec<i32> {
    // TODO: Return the k largest elements in descending order
    // Hint: Create a max-heap and pop k times
    todo!()
}

/// Returns the k smallest elements in ascending order.
/// If k > items.len(), return all items sorted ascending.
pub fn top_k_smallest(items: &[i32], k: usize) -> Vec<i32> {
    // TODO: Return the k smallest elements in ascending order
    // Hint: Create a min-heap using Reverse and pop k times
    // Don't forget to unwrap Reverse when collecting results
    todo!()
}

/// Merges two heaps into one.
pub fn merge_heaps(heap1: BinaryHeap<i32>, heap2: BinaryHeap<i32>) -> BinaryHeap<i32> {
    // TODO: Merge both heaps into a single heap
    // Hint: Use append() to drain one heap into another
    todo!()
}

/// Sorts items in descending order using a BinaryHeap.
pub fn heap_sort_descending(items: &[i32]) -> Vec<i32> {
    // TODO: Sort items in descending order using a heap
    // Hint: into_sorted_vec() gives ascending order - you may need to reverse
    todo!()
}

fn main() {
    // Create a max-heap
    let max_heap = create_max_heap(&[3, 1, 4, 1, 5, 9, 2, 6]);
    println!("Max element: {:?}", max_heap.peek());

    // Create a min-heap
    let min_heap = create_min_heap(&[3, 1, 4, 1, 5, 9, 2, 6]);
    println!("Min element: {:?}", min_heap.peek().map(|r| r.0));

    // Get top 3 largest
    let top3 = top_k_largest(&[3, 1, 4, 1, 5, 9, 2, 6], 3);
    println!("Top 3 largest: {:?}", top3);

    // Get top 3 smallest
    let bottom3 = top_k_smallest(&[3, 1, 4, 1, 5, 9, 2, 6], 3);
    println!("Top 3 smallest: {:?}", bottom3);

    // Heap sort descending
    let sorted = heap_sort_descending(&[3, 1, 4, 1, 5]);
    println!("Sorted descending: {:?}", sorted);
}
