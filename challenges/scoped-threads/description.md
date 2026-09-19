`thread::spawn` demands `'static`, which means a spawned thread can never borrow a local variable. The usual workaround is to clone the data or wrap it in an `Arc`, and for a short parallel scan over a slice you already own that feels like a lot of ceremony.

`std::thread::scope` removes the need for it. You hand it a closure, and inside that closure you spawn threads with `s.spawn(...)`. The scope will not return until every thread it spawned has been joined. Since the compiler knows that, it can prove the borrows end before the borrowed data does, and ordinary references are allowed straight through.

That also means mutable borrows work, as long as they follow the usual rules. Two scoped threads can each hold a `&mut` to a **different** local at the same time.

## Your Task

Implement two functions. Both must use `thread::scope`, and neither may clone the input or reach for `Arc`.

### Implement `parallel_max`

```rust
pub fn parallel_max(data: &[i32]) -> Option<i32>
```

Split `data` in half, find the maximum of each half concurrently, and return the larger. An empty slice has no maximum.

```rust
assert_eq!(parallel_max(&[3, 9, 1, 4]), Some(9));
assert_eq!(parallel_max(&[]), None);
```

### Implement `split_evens_odds`

```rust
pub fn split_evens_odds(
    data: &[i32],
) -> (Vec<i32>, Vec<i32>)
```

Use one scoped thread to collect the even values and another to collect the odd ones, then return `(evens, odds)`. Each vector keeps the relative order of the input.

```rust
let (e, o) = split_evens_odds(&[1, 2, 3, 4]);
assert_eq!(e, vec![2, 4]);
assert_eq!(o, vec![1, 3]);
```

### Notes

- 0 is even, and negative numbers follow the same rule: `-2` is even, `-1` is odd.
- The scope joins for you when the closure ends, so you only call `join()` when you actually want a returned value back.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- `slice::split_at(mid)` gives you two subslices without copying anything.
- `thread::scope(|s| { ... })` returns whatever its closure returns, so you can compute the answer inside and let it fall out.
- `s.spawn(...)` gives a scoped handle. Call `join()` on it when you need the value, or ignore it and let the scope clean up.
- Combining two `Option<i32>` values: match on the pair, and `a.or(b)` covers the cases where at most one is `Some`.
- For the split, borrow `evens` in one closure and `odds` in the other. Borrowing the same vector in both will not compile, and that is the point.

</details>
