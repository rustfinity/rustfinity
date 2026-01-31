In this challenge, you will demonstrate your understanding of **control flow** in Rust. The task involves finding the first **decimal palindrome** in a given range.

### What's a decimal palindrome?

**A decimal palindrome is a number whose decimal (base 10,
"normal") digits read the same backward as forward**.
This exercise will require you to find the numerically
least non-negative palindrome in a given range. The easiest
way to do this is to iterate through the range, check each
number to see if it is a palindrome, and return the first
palindrome found. You can use any **control flow** construct
to solve this problem. (There are much more efficient ways
to solve this problem, but the calculations get complex
quickly.)

Palindromes are fascinating numbers, and finding them within a range will require clear control flow logic to ensure you identify the first one accurately.

## Your task

You need to write a function,
`find_first_palindrome(start: i32, end: i32) -> Option<i32>`,
that takes two **integer** arguments `start` and `end`.
The function should return the numerically least
non-negative palindrome number within the range.

- The range is inclusive: for example, if `start == 1`
  and `end == 1` the palindrome `1` is in range.

- The range may have `start > end`, in which case it is still a valid range: for example, `start == 3` and `end == 1` contains the values `1, 2, 3`.

If there are no palindromes in the range, the function should return `None`.

## Requirements

- The function should cover the numbers from `start` to `end` inclusive.
- It should check each non-negative number to determine if it is a **palindrome**.
- The function should **return the numerically least palindrome found**, or `None` if no palindromes exist in the range.
- You must handle "reversed" ranges, when `start` is greater than `end`.

## Example

```rust
let result = find_first_palindrome(10, 30);
assert_eq!(result, Some(11)); // 11 is the first palindrome in the range

let result = find_first_palindrome(100, 105);
assert_eq!(result, Some(101)); // 101 is the first palindrome in the range

let result = find_first_palindrome(123, 130);
assert_eq!(result, None); // No palindromes in this range

let result = find_first_palindrome(-130, -1);
assert_eq!(result, None); // No palindromes in this range

let result = find_first_palindrome(100, -105);
assert_eq!(result, Some(0)); // 0 is the first palindrome in the range
```

## Did you know?

Did you know that **palindromes** are not just limited to numbers? They are found in **words, phrases, and even DNA sequences!** For example, the word **"racecar"** is a palindrome, as it reads the same backward and forward. Check out this ["Weird Al" video](https://youtu.be/JUQDzj6R3p4) for many many examples.

**Palindromes** are fascinating in various fields, including mathematics, literature, and biology, where they often have unique properties and significance.

## Hints

- To check if a number is a palindrome, you probably want to convert it to a `String` and compare it with its **reverse**.
- You can get the `char`s in a `String` by using the `chars()` method on a `String`
- You can iterate over values in reverse order by using the `rev()` method on an iterator. For example, you can get the `char`s in a `String` `s` in reverse order with `s.chars().rev()`
- Use a **loop**, iterator, or any other **control flow** construct to check each number in the range until a **palindrome** is found or the range is exhausted.
- Remember to handle the case **where start is greater than end**, probably by swapping the values if necessary.
