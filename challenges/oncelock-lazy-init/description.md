Some values are expensive to build and should only be built if something actually asks for them: a parsed config, a compiled regex, a lookup table. A `static` cannot help, because a `static` initializer has to be a constant expression evaluated at compile time.

`OnceLock<T>` is the answer. It is a slot that starts empty and can be filled exactly once:

```rust
static SETTINGS: OnceLock<Settings> = OnceLock::new();

fn settings() -> &'static Settings {
    SETTINGS.get_or_init(build_settings)
}
```

`get_or_init` is where the concurrency lives. If ten threads call it at the same moment on an empty lock, exactly one runs `build_settings`. The other nine block until it finishes and then get a reference to that same value. The initializer never runs twice, and no caller ever sees a half-built value.

Because the value can never be replaced, handing out `&'static T` is sound, and callers need no lock at all to read it.

`LazyLock<T>` is the same machinery with the initializer stored inside, so it can be used like a normal static:

```rust
static TABLE: LazyLock<Vec<u64>> =
    LazyLock::new(|| expensive());
```

Reach for `LazyLock` when the initializer is a self-contained closure, and `OnceLock` when the value comes from somewhere the static cannot see, such as a command-line argument set during startup.

## Your Task

Fill in three items. `Settings` and the `BUILD_COUNT` counter are given to you.

```rust
pub struct Settings {
    pub name: String,
    pub retries: u32,
}
```

`build_settings()` is also given. It bumps `BUILD_COUNT` and returns `Settings { name: "rustfinity", retries: 3 }`. Do not change it: the tests use that counter to prove your initializer runs exactly once.

### The storage slot

Declare a `static` `OnceLock<Settings>` for `settings()` to fill.

### Implement `settings`

```rust
pub fn settings() -> &'static Settings
```

Returns the shared settings, building them with `build_settings` on the first call only. Every call, from every thread, must return a reference to the same value.

### Implement `build_count`

```rust
pub fn build_count() -> usize
```

Reads `BUILD_COUNT`. After any number of `settings()` calls from any number of threads, this must be 1.

### Declare `TRIANGULAR`

```rust
pub static TRIANGULAR: LazyLock<Vec<u64>>
```

The first 32 triangular numbers, computed on first use: `0, 1, 3, 6, 10, 15, ...`, where entry `n` is the sum of every integer from 0 to `n`.

```rust
assert_eq!(TRIANGULAR[..6], [0, 1, 3, 6, 10, 15]);
assert_eq!(TRIANGULAR.len(), 32);
```

### Notes

- `OnceLock::new()` is a `const fn`, which is what lets it appear in a `static`.
- There is a closed form for the nth triangular number, but a running sum works just as well.
- `LazyLock<Vec<u64>>` derefs to `Vec<u64>`, so `TRIANGULAR[3]` and `TRIANGULAR.len()` work without any extra unwrapping.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- The slot is `static SETTINGS: OnceLock<Settings> = OnceLock::new();`.
- `get_or_init` takes the initializer as a function, so `SETTINGS.get_or_init(build_settings)` is the whole body of `settings`. No parentheses after `build_settings`: you are passing the function, not calling it.
- `build_count` is one `load(Ordering::Relaxed)` on the counter.
- For the table: `(0..32).map(|n: u64| n * (n + 1) / 2).collect()`.
- If the compiler complains that the closure type does not match, remember `LazyLock::new` wants a closure taking no arguments.

</details>
