// TODO: Define a generic struct with a single field `item: T`.
pub struct ItemContainer {
    // Add the field here
}

impl<T> ItemContainer<T> {
    // TODO: Implement the `get_item` method to return a reference to the item.
}

// Example usage
pub fn main() {
    let item_1 = ItemContainer { item: 42 };
    assert_eq!(*item_1.get_item(), 42);

    let item_2 = ItemContainer {
        item: String::from("Hello"),
    };

    assert_eq!(item_2.get_item(), "Hello");
}
