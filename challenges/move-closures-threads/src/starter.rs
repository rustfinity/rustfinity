use std::thread;

/// Counts the characters in every word, on a separate thread.
pub fn count_chars_in_thread(words: Vec<String>) -> usize {
    // TODO: move `words` into a spawned thread, count, and join
    unimplemented!()
}

/// Greets every name on its own thread, preserving input order.
pub fn greet_each(prefix: String, names: Vec<String>) -> Vec<String> {
    // TODO: spawn one thread per name and join them in spawn order
    unimplemented!()
}

// Example usage
pub fn main() {
    let words = vec!["hello".to_string(), "world".to_string()];
    println!("chars: {}", count_chars_in_thread(words));

    let names = vec!["Ada".to_string(), "Bo".to_string()];
    println!("{:?}", greet_each("Hi".to_string(), names));
}
