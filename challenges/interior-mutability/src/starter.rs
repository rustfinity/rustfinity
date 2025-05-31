use std::cell::RefCell;
use std::rc::Rc;

pub fn push<T>(data: Rc<RefCell<Vec<T>>>, element: T) {
    // 1. Finish the function
}

pub fn iterate_and_print_shared_data<T>(data: Rc<RefCell<Vec<T>>>) {
    // 2. Borrow the data and print each item
}

pub fn plus_one(data: Rc<RefCell<i32>>) {
    // 3. Finish the function
}

// Example usage
pub fn main() {
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));

    // Updating shared data
    push(Rc::clone(&shared_data), 4);
    push(Rc::clone(&shared_data), 5);

    // Iterating and printing shared data
    println!("Shared Data:");
    iterate_and_print_shared_data(Rc::clone(&shared_data));

    let my_num = Rc::new(RefCell::new(5));
    plus_one(Rc::clone(&my_num));
    assert_eq!(*my_num.borrow(), 6, "Value was not incremented correctly.");
}
