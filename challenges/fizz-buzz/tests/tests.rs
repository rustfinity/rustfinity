#[cfg(test)]
mod tests {
    use fizz_buzz::*;

    #[test]
    fn should_return_fizz() {
        assert_eq!(fizz_buzz(3), "Fizz");
        assert_eq!(fizz_buzz(6), "Fizz");
        assert_eq!(fizz_buzz(9), "Fizz");
        assert_eq!(fizz_buzz(12), "Fizz");
        assert_eq!(fizz_buzz(18), "Fizz");
    }

    #[test]
    fn should_return_buzz() {
        assert_eq!(fizz_buzz(5), "Buzz");
        assert_eq!(fizz_buzz(10), "Buzz");
        assert_eq!(fizz_buzz(20), "Buzz");
        assert_eq!(fizz_buzz(25), "Buzz");
        assert_eq!(fizz_buzz(35), "Buzz");
    }

    #[test]
    fn should_return_fizz_buzz() {
        assert_eq!(fizz_buzz(15), "FizzBuzz");
        assert_eq!(fizz_buzz(30), "FizzBuzz");
        assert_eq!(fizz_buzz(45), "FizzBuzz");
        assert_eq!(fizz_buzz(60), "FizzBuzz");
        assert_eq!(fizz_buzz(75), "FizzBuzz");
    }

    #[test]
    fn should_return_number() {
        assert_eq!(fizz_buzz(1), "1");
        assert_eq!(fizz_buzz(2), "2");
        assert_eq!(fizz_buzz(4), "4");
        assert_eq!(fizz_buzz(7), "7");
        assert_eq!(fizz_buzz(8), "8");
    }
}
