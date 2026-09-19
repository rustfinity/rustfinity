pub fn mutating_variables() -> String {
    // 1. Declare a mutable variable `text` with value "hello"

    // 2. Call `mutates_value` with a mutable reference to `text`

    // 3. Return the value of `text` as a String
    unimplemented!()
}

// Do not change this function
pub fn mutates_value(value: &mut String) {
    *value = String::from("bye")
}
