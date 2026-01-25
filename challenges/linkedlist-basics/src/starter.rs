use std::collections::LinkedList;

/// Creates a linked list from a slice, maintaining the order of elements.
pub fn create_list(items: &[i32]) -> LinkedList<i32> {
    // TODO: Create a LinkedList containing all items in order
    // Hint: You can use .iter().copied().collect() or iterate and push_back
    todo!()
}

/// Adds an element to the front of the list.
pub fn add_front<T>(list: &mut LinkedList<T>, item: T) {
    // TODO: Add item to the front of the list
    // Hint: LinkedList has a push_front() method
    todo!()
}

/// Adds an element to the back of the list.
pub fn add_back<T>(list: &mut LinkedList<T>, item: T) {
    // TODO: Add item to the back of the list
    // Hint: LinkedList has a push_back() method
    todo!()
}

/// Removes and returns the front element.
pub fn remove_front<T>(list: &mut LinkedList<T>) -> Option<T> {
    // TODO: Remove and return the front element
    // Hint: LinkedList has a pop_front() method
    todo!()
}

/// Removes and returns the back element.
pub fn remove_back<T>(list: &mut LinkedList<T>) -> Option<T> {
    // TODO: Remove and return the back element
    // Hint: LinkedList has a pop_back() method
    todo!()
}

/// Returns a reference to the front element without removing it.
pub fn peek_front<T>(list: &LinkedList<T>) -> Option<&T> {
    // TODO: Return a reference to the front element
    // Hint: LinkedList has a front() method
    todo!()
}

/// Returns a reference to the back element without removing it.
pub fn peek_back<T>(list: &LinkedList<T>) -> Option<&T> {
    // TODO: Return a reference to the back element
    // Hint: LinkedList has a back() method
    todo!()
}

/// Finds an element by value and moves it to the front.
/// Returns true if found and moved, false if not found.
pub fn move_to_front<T: PartialEq>(list: &mut LinkedList<T>, value: &T) -> bool {
    // TODO: Find the element matching value and move it to the front
    // This is trickier - LinkedList doesn't support random access removal
    // You may need to drain the list and rebuild it
    // Hint: Use pop_front() in a loop to drain, then rebuild with the found
    // element at the front
    todo!()
}

/// Concatenates two lists, with list2's elements appended after list1's.
pub fn concat_lists<T>(list1: LinkedList<T>, list2: LinkedList<T>) -> LinkedList<T> {
    // TODO: Combine list1 and list2, with list2's elements after list1's
    // Hint: append() drains one list into another
    todo!()
}

fn main() {
    // Create a list
    let list = create_list(&[1, 2, 3, 4, 5]);
    println!("Created list with {} elements", list.len());
    println!("Front: {:?}, Back: {:?}", list.front(), list.back());

    // Add elements at both ends
    let mut list = LinkedList::new();
    add_back(&mut list, 2);
    add_front(&mut list, 1);
    add_back(&mut list, 3);
    println!("After adding 1 at front, 2 and 3 at back: {:?}", list.iter().collect::<Vec<_>>());

    // Remove elements
    let mut list = create_list(&[10, 20, 30]);
    println!("Removed from front: {:?}", remove_front(&mut list));
    println!("Removed from back: {:?}", remove_back(&mut list));
    println!("Remaining: {:?}", list.iter().collect::<Vec<_>>());

    // LRU-style move to front
    let mut list = create_list(&[1, 2, 3, 4, 5]);
    println!("Before move_to_front(&3): {:?}", list.iter().collect::<Vec<_>>());
    move_to_front(&mut list, &3);
    println!("After move_to_front(&3): {:?}", list.iter().collect::<Vec<_>>());

    // Concatenate lists
    let list1 = create_list(&[1, 2]);
    let list2 = create_list(&[3, 4]);
    let combined = concat_lists(list1, list2);
    println!("Combined list: {:?}", combined.iter().collect::<Vec<_>>());
}
