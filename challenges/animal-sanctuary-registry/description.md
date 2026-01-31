## Hashmaps

Hashmaps are a powerful data structure that allow you to store **key-value pairs**. In Rust, the `HashMap` type is provided that uses the a **hashing algorithm** called **SipHash** to store keys and values in a way that allows for **fast and secure** lookups. Think of them as a dictionary in Python or an object in JavaScript.

In this challenge, we want to build a sanctuary registry that allows us to manage animals in different sections of the sanctuary. We'll use a `HashMap` to store the sections as keys and a `Vec` to store the animals in each section. Each key is a section name `String` and each value is a list of animals in that section `Vec<String>`.

## Task

You are given a type of `Collection` which is a `HashMap<String, Vec<String>>`. The key is the section name and the value is a list of animals in that section.

Your task is to implement the following functions:

1. `add_animal_to_section`: This function should add an animal to a section in the registry. If the section does not exist, it should be created. If the animal is already in the section, it should not be added again.

2. `get_animals_in_section`: This function should return a list of animals **sorted alphabetically** in a given section. If the section does not exist, it should return an empty list.

3. `get_all_animals_sorted`: This function should return a copy of the entire registry with all animals **sorted alphabetically** in each section.

## Example

```rust
let mut registry = Collection::new();

add_animal_to_section("Eagle", "Birds", &mut registry);
assert_eq!(
    get_animals_in_section("Birds", &registry),
    vec!["Eagle"]
);
```

## Hints

- Use the `insert` method on a `HashMap` to add a new section or update an existing section.
- Use the `entry` method on a `HashMap` to get a mutable reference to a section.
- Use the `iter` method on a `HashMap` to iterate over the key-value pairs.
- Use the `sort` method on a `Vec` to sort the animals alphabetically.
