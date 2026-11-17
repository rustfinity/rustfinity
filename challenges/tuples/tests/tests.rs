#[cfg(test)]
mod tests {
    use tuples::*;

    #[test]
    fn test_create_tuple() {
        let result = create_tuple(42, 3.14, "hello");
        assert_eq!(result, (3.14, 42, String::from("hello")));
    }

    #[test]
    fn test_create_tuple_empty_string() {
        let result = create_tuple(0, 0.0, "");
        assert_eq!(result, (0.0, 0, String::from("")));
    }

    #[test]
    fn test_create_tuple_negative_values() {
        let result = create_tuple(-42, -3.14, "negative");
        assert_eq!(result, (-3.14, -42, String::from("negative")));
    }
}
