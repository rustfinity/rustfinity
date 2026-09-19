use move_closures_threads::*;

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

mod count_chars_tests {
    use super::*;

    #[test]
    fn counts_two_words() {
        assert_eq!(count_chars_in_thread(strings(&["ab", "cde"])), 5);
    }

    #[test]
    fn empty_input_is_zero() {
        assert_eq!(count_chars_in_thread(vec![]), 0);
    }

    #[test]
    fn empty_strings_count_as_zero() {
        assert_eq!(count_chars_in_thread(strings(&["", ""])), 0);
    }

    #[test]
    fn counts_characters_not_bytes() {
        assert_eq!(count_chars_in_thread(strings(&["héllo"])), 5);
    }

    #[test]
    fn counts_many_words() {
        let words = strings(&["a", "bb", "ccc", "dddd", "eeeee"]);
        assert_eq!(count_chars_in_thread(words), 15);
    }
}

mod greet_each_tests {
    use super::*;

    #[test]
    fn greets_two_names() {
        let got = greet_each("Hi".to_string(), strings(&["Ada", "Bo"]));
        assert_eq!(got, vec!["Hi, Ada!", "Hi, Bo!"]);
    }

    #[test]
    fn no_names_gives_no_greetings() {
        let got = greet_each("Hi".to_string(), vec![]);
        assert!(got.is_empty());
    }

    #[test]
    fn preserves_input_order() {
        let names = strings(&["d", "c", "b", "a"]);
        let got = greet_each("Yo".to_string(), names);
        assert_eq!(got, vec!["Yo, d!", "Yo, c!", "Yo, b!", "Yo, a!"]);
    }

    #[test]
    fn every_thread_sees_the_same_prefix() {
        let names = strings(&["one", "two", "three", "four", "five"]);
        let got = greet_each("Hello".to_string(), names);

        assert_eq!(got.len(), 5);
        assert!(got.iter().all(|g| g.starts_with("Hello, ")));
    }

    #[test]
    fn duplicate_names_are_all_kept() {
        let got = greet_each("Hi".to_string(), strings(&["Ada", "Ada"]));
        assert_eq!(got, vec!["Hi, Ada!", "Hi, Ada!"]);
    }

    #[test]
    fn works_with_an_empty_prefix() {
        let got = greet_each(String::new(), strings(&["Ada"]));
        assert_eq!(got, vec![", Ada!"]);
    }
}
