You now have a good understanding of iterators and closures and how they can be used to process collections of data.

In this challenge, you will use iterators to filter out duplicate items from a collection of potentially redundant entries.

## Your Task

Define a function that takes a collection of items (as an iterator) and returns a collection of unique items in sorted order. The items are represented as strings. You are expected to handle duplicates and ignore entries that are **empty** or consist solely of whitespace.

Your function signature and return types must be determined by you, but the input must implement the `Iterator` trait, ensuring flexibility for different iterator sources.

### Requirements

- **Filter duplicates:** Ensure that the resulting collection contains only unique items.
- **Ignore invalid entries:** Exclude entries that are empty or only consist of whitespace.
- **Sort the output:** Return the items sorted in ascending lexicographical order.
- **Flexibility:** The function should work with any iterator that has `String`, `&String`, or `&str` items.

## Hints

<details>
<summary>Click here to reveal hints</summary>

- Use trait bounds to ensure that the input iterator can provide items that can be converted to strings. e.g.
  ```rust
  fn unique_items<I, T>(items: I) -> Vec<String>
  where
        I: Iterator<Item = T>,
        T: AsRef<str>,
  {
      // Your code here
  }
  ```
- Use a `HashSet` to track unique items.
- Use the `filter_map` method to remove invalid entries (e.g., empty or whitespace-only strings).
- You can use the `trim` method on strings to handle whitespace effectively.
- `HashSet` provides a method `insert` that returns a `bool` indicating whether the item was already present.
- To sort, you can convert to a `Vec` and call the `sort` method.

</details>
