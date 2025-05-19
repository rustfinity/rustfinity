Enums in Rust can combine unit, tuple and named field variants to represent a wide range of concepts. This challenge uses enums to represent playing cards in a card game.

You will implement a `Card` enum that can represent either:

- A face card (e.g., `King`, `Queen`, `Jack`), which is a unit variant.
- A numbered card with a value (e.g., `7 of Hearts`), which is a tuple variant.

## Your Task

Create an enum `Card` with the following variants:

- `King`, `Queen`, `Jack` (unit variants for face cards).
- `Numbered(u8, String)` representing a numbered card with its value and suit.

Write a function `card_description` that takes a `Card` and returns a description of the card:

- For `King`, return `"King"`.
- For `Queen`, return `"Queen"`.
- For `Jack`, return `"Jack"`.
- For `Numbered(value, suit)`, return `"{value} of {suit}"`, e.g., `"7 of Hearts"`.
- Ignore error handling for this challenge.

## Hints

<details>
<summary>Click here to reveal hints</summary>

- Use the `match` statement to handle each enum variant.
- String interpolation can be done with the `format!` macro for the `Numbered` variant.

</details>
