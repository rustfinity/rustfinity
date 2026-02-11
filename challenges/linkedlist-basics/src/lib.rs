use std::collections::LinkedList;

/// Creates a linked list from a slice, maintaining the order of elements.
///
/// # Arguments
///
/// * `items` - A slice of integers to add to the list
///
/// # Returns
///
/// A LinkedList containing all the items in the same order
///
/// # Examples
///
/// ```
/// use linkedlist_basics::create_list;
///
/// let list = create_list(&[1, 2, 3]);
/// assert_eq!(list.front(), Some(&1));
/// assert_eq!(list.back(), Some(&3));
/// assert_eq!(list.len(), 3);
/// ```
pub fn create_list(items: &[i32]) -> LinkedList<i32> {
    items.iter().copied().collect()
}

/// Adds an element to the front of the list.
///
/// # Arguments
///
/// * `list` - A mutable reference to the LinkedList
/// * `item` - The item to add at the front
///
/// # Examples
///
/// ```
/// use linkedlist_basics::add_front;
/// use std::collections::LinkedList;
///
/// let mut list = LinkedList::new();
/// add_front(&mut list, 1);
/// add_front(&mut list, 2);
/// assert_eq!(list.front(), Some(&2));
/// ```
pub fn add_front<T>(list: &mut LinkedList<T>, item: T) {
    list.push_front(item);
}

/// Adds an element to the back of the list.
///
/// # Arguments
///
/// * `list` - A mutable reference to the LinkedList
/// * `item` - The item to add at the back
///
/// # Examples
///
/// ```
/// use linkedlist_basics::add_back;
/// use std::collections::LinkedList;
///
/// let mut list = LinkedList::new();
/// add_back(&mut list, 1);
/// add_back(&mut list, 2);
/// assert_eq!(list.back(), Some(&2));
/// ```
pub fn add_back<T>(list: &mut LinkedList<T>, item: T) {
    list.push_back(item);
}

/// Removes and returns the front element.
///
/// # Arguments
///
/// * `list` - A mutable reference to the LinkedList
///
/// # Returns
///
/// Some(element) if the list is not empty, None otherwise
///
/// # Examples
///
/// ```
/// use linkedlist_basics::{create_list, remove_front};
///
/// let mut list = create_list(&[1, 2, 3]);
/// assert_eq!(remove_front(&mut list), Some(1));
/// assert_eq!(remove_front(&mut list), Some(2));
/// assert_eq!(list.len(), 1);
/// ```
pub fn remove_front<T>(list: &mut LinkedList<T>) -> Option<T> {
    list.pop_front()
}

/// Removes and returns the back element.
///
/// # Arguments
///
/// * `list` - A mutable reference to the LinkedList
///
/// # Returns
///
/// Some(element) if the list is not empty, None otherwise
///
/// # Examples
///
/// ```
/// use linkedlist_basics::{create_list, remove_back};
///
/// let mut list = create_list(&[1, 2, 3]);
/// assert_eq!(remove_back(&mut list), Some(3));
/// assert_eq!(remove_back(&mut list), Some(2));
/// assert_eq!(list.len(), 1);
/// ```
pub fn remove_back<T>(list: &mut LinkedList<T>) -> Option<T> {
    list.pop_back()
}

/// Returns a reference to the front element without removing it.
///
/// # Arguments
///
/// * `list` - A reference to the LinkedList
///
/// # Returns
///
/// Some(&element) if the list is not empty, None otherwise
///
/// # Examples
///
/// ```
/// use linkedlist_basics::{create_list, peek_front};
///
/// let list = create_list(&[10, 20, 30]);
/// assert_eq!(peek_front(&list), Some(&10));
/// assert_eq!(list.len(), 3); // Unchanged
/// ```
pub fn peek_front<T>(list: &LinkedList<T>) -> Option<&T> {
    list.front()
}

/// Returns a reference to the back element without removing it.
///
/// # Arguments
///
/// * `list` - A reference to the LinkedList
///
/// # Returns
///
/// Some(&element) if the list is not empty, None otherwise
///
/// # Examples
///
/// ```
/// use linkedlist_basics::{create_list, peek_back};
///
/// let list = create_list(&[10, 20, 30]);
/// assert_eq!(peek_back(&list), Some(&30));
/// assert_eq!(list.len(), 3); // Unchanged
/// ```
pub fn peek_back<T>(list: &LinkedList<T>) -> Option<&T> {
    list.back()
}

/// Finds an element by value and moves it to the front.
///
/// This is a key operation for LRU cache behavior where recently accessed
/// items should be moved to the front to mark them as "most recently used".
///
/// # Arguments
///
/// * `list` - A mutable reference to the LinkedList
/// * `value` - The value to find and move
///
/// # Returns
///
/// `true` if the element was found and moved, `false` otherwise
///
/// # Examples
///
/// ```
/// use linkedlist_basics::{create_list, move_to_front};
///
/// let mut list = create_list(&[1, 2, 3, 4, 5]);
/// assert!(move_to_front(&mut list, &3));
/// assert_eq!(list.front(), Some(&3));
/// // List is now: [3, 1, 2, 4, 5]
///
/// assert!(!move_to_front(&mut list, &99));  // Not found
/// ```
pub fn move_to_front<T: PartialEq>(list: &mut LinkedList<T>, value: &T) -> bool {
    // First, check if the value exists and find it
    let mut found = false;
    let mut new_list = LinkedList::new();
    let mut item_to_move = None;

    // Drain the list and rebuild without the target element
    while let Some(item) = list.pop_front() {
        if !found && &item == value {
            found = true;
            item_to_move = Some(item);
        } else {
            new_list.push_back(item);
        }
    }

    // If found, add it to the front of the new list
    if let Some(item) = item_to_move {
        new_list.push_front(item);
    }

    // Replace the original list with the new one
    *list = new_list;

    found
}

/// Concatenates two lists, with list2's elements appended after list1's.
///
/// # Arguments
///
/// * `list1` - The first LinkedList
/// * `list2` - The second LinkedList to append
///
/// # Returns
///
/// A new LinkedList containing all elements from list1 followed by list2
///
/// # Examples
///
/// ```
/// use linkedlist_basics::{create_list, concat_lists};
///
/// let list1 = create_list(&[1, 2]);
/// let list2 = create_list(&[3, 4]);
/// let combined = concat_lists(list1, list2);
/// assert_eq!(combined.len(), 4);
/// assert_eq!(combined.front(), Some(&1));
/// assert_eq!(combined.back(), Some(&4));
/// ```
pub fn concat_lists<T>(mut list1: LinkedList<T>, mut list2: LinkedList<T>) -> LinkedList<T> {
    list1.append(&mut list2);
    list1
}
