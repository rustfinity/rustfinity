---
name: create-challenge
description: Guide for creating new Rustfinity challenges following best practices
---

# Rustfinity Challenge Creation Guide

This skill guides you through creating high-quality Rust learning challenges for the Rustfinity platform.

## Challenge Structure Requirements

Every challenge must have the following files in `challenges/<slug>/`:

1. **description.md** - Challenge instructions
2. **Cargo.toml** - Package configuration (name must match directory slug)
3. **src/lib.rs** - Complete solution implementation
4. **src/starter.rs** - Incomplete starter code for learners
5. **tests/tests.rs** - Comprehensive test suite

## Writing description.md

### Structure

1. **Introduction**: Brief explanation of the concept (2-3 paragraphs)
2. **Your Task** or **Requirements**: Clear, bulleted requirements
3. **Example** (optional): Code examples showing expected behavior
4. **Hints** (optional): Collapsible hints section for struggling learners

### Best Practices

- Start with conceptual explanation before diving into implementation details
- Use "You are given..." or "Your job is to..." to frame the task
- Include code examples with `assert_eq!` statements to clarify behavior
- Add inline comments to explain non-obvious aspects
- Use collapsible `<details>` sections for hints:

  ```markdown
  ## Hints

  <details>
    <summary>Click here to reveal hints</summary>

  - Your hints here
  - Multiple bullet points work well

  </details>
  ```

- Be encouraging and educational, not condescending
- Explain the "why" not just the "what"

### Example Patterns

- **Beginner challenges**: Focus on syntax and basic concepts

  - "You are given a function `name(params) -> return_type`..."
  - "Your job is to implement the function meeting the following requirements:"

- **Intermediate challenges**: Introduce traits, generics, error handling

  - "Now that you know about X, let's explore Y..."
  - "Define a generic function that..."

- **Advanced challenges**: Complex trait implementations, operator overloading
  - "You're already familiar with X, let's dive into Y..."
  - "Implement custom behavior for the `+` operator..."

## Writing src/starter.rs

### Rules

1. **Must be incomplete** - Should not compile or pass tests as-is
2. **Provide scaffolding** - Include struct definitions, function signatures
3. **Include TODOs** - Mark incomplete sections with `// TODO: ...`
4. **Show patterns** - Give learners a starting point without giving away the answer
5. **Include example usage** - Add a `main()` function showing how to use the code

### Examples

**Simple function challenge:**

```rust
pub fn fibonacci(n: u32) -> u32 {
    // TODO: Implement the Fibonacci sequence
}
```

**Trait implementation challenge:**

```rust
pub struct Millimeters(pub u32);
pub struct Meters(pub u32);

// Implement the Add trait

// Example usage
pub fn main() {
    let length1 = Millimeters(1500);
    let length2 = Meters(3);

    let result = length1 + length2;
    assert_eq!(result.0, 4500);
}
```

**Generic function with partial scaffold:**

```rust
// TODO: Define the generic function `compare_and_display` with appropriate trait bounds.
pub fn compare_and_display // Complete the function definition

// Example usage
pub fn main() {
     let greater = compare_and_display(10, 20);
     println!("Greater value: {}", greater);
}
```

**Struct with trait implementation:**

```rust
#[derive(Debug)]
pub struct AppConfig {
    pub theme: String,
    pub notifications_enabled: bool,
    pub max_users: u32,
}

// TODO: implement the `Default` trait for `AppConfig`

// Example usage
pub fn main() {
    let default_config = AppConfig::default();
    println!("Default Config: {:?}", default_config);
}
```

## Writing src/lib.rs

### Rules

1. **Complete implementation** - Must compile and pass all tests
2. **High quality code** - Use idiomatic Rust patterns
3. **Add documentation** - Include doc comments for public items
4. **Show best practices** - Demonstrate efficient algorithms and proper error handling
5. **Keep it focused** - Don't add unnecessary complexity

### Example

````rust
/// Dynamic Programming implementation of the Fibonacci sequence.
///
/// # Arguments
///
/// * `n`: u32, the nth number in the fibonacci sequence
///
/// returns: u32 the result of the nth number in the fibonacci sequence
///
/// # Examples
///
/// ```
/// use fibonacci::fibonacci;
/// let result = fibonacci(4);
/// assert_eq!(result, 3);
/// ```
pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    let mut fib = vec![0; (n + 1) as usize];
    fib[1] = 1;
    for i in 2..=n as usize {
        fib[i] = fib[i - 1] + fib[i - 2];
    }
    fib[n as usize]
}
````

## Writing tests/tests.rs

### Rules

1. **Comprehensive coverage** - Test edge cases, typical cases, and boundary conditions
2. **Clear test names** - Use descriptive names like `test_compare_integers`, `zero`, `one`
3. **Multiple test functions** - Break tests into logical groups
4. **Test diverse types** - For generic code, test with multiple type parameters
5. **Include custom structs** - For trait bounds, show tests with custom types

### Test Organization

```rust
use challenge_name::*;

#[test]
fn edge_case_zero() {
    assert_eq!(fibonacci(0), 0);
}

#[test]
fn edge_case_one() {
    assert_eq!(fibonacci(1), 1);
}

#[test]
fn typical_cases() {
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(10), 55);
}
```

### For Generic Functions

Test with multiple types to ensure trait bounds work:

```rust
#[test]
fn test_compare_integers() {
    assert_eq!(compare_and_display(10, 20), 20);
}

#[test]
fn test_compare_strings() {
    assert_eq!(compare_and_display("Apple", "Orange"), "Orange");
}

#[test]
fn test_compare_custom_struct() {
    #[derive(Debug, PartialOrd, PartialEq)]
    struct Point { x: i32, y: i32 }

    impl std::fmt::Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Point({}, {})", self.x, self.y)
        }
    }

    assert_eq!(compare_and_display(Point{x:1,y:2}, Point{x:3,y:4}), Point{x:3,y:4});
}
```

## Writing Cargo.toml

### Template

```toml
[package]
name = "challenge-slug"  # Must match directory name
version = "0.1.0"
edition = "2021"

[dependencies]
# Add only necessary dependencies
```

### Rules

- Package name MUST match directory slug exactly
- Use minimal dependencies
- Only add dependencies if truly needed for the challenge concept

### When to Use syntest

The `syntest` crate is **only needed when you need to capture stdout/stderr output** (e.g., testing `println!` output). For most challenges that test return values, you don't need it.

**Add syntest only for output capture:**

```toml
[package]
name = "challenge-slug"
version = "0.1.0"
edition = "2021"

[dev-dependencies]
syntest = { path = "../../crates/syntest" }
```

**Example test using syntest to capture printed output:**

```rust
use my_challenge::*;
use syntest::quote;

#[test]
fn test_prints_hello() {
    let code = quote! {
        use my_challenge::*;

        greet("Alice");  // Function that prints to stdout
    };

    let result = syntest::create_bin_and_run(&code);

    assert!(
        result.stdout().contains("Hello, Alice"),
        "Expected greeting to be printed"
    );
}
```

**For most challenges, no syntest needed:**

```toml
[package]
name = "challenge-slug"
version = "0.1.0"
edition = "2021"

# No dev-dependencies needed for testing return values
```

## Metadata in challenges.json

### Required Fields

```json
{
  "id": 999, // Get next ID with challenges::get_max_id() + 1
  "title": "Challenge Title",
  "slug": "challenge-slug", // Must match directory name
  "short_description": "Brief one-liner description",
  "language": "RUST",
  "difficulty": "BEGINNER|EASY|MEDIUM|HARD|ADVANCED",
  "track": "RUST_BASICS|CONTROL_FLOW|DSA",
  "tags": ["tag1", "tag2"],
  "created_at": "2024-06-07T00:00:00Z",
  "updated_at": "2024-06-07T00:00:00Z"
}
```

### Rules

- Use unique, sequential ID (never reuse IDs)
- Slug must match directory name exactly
- Use appropriate difficulty level
- Add 3-5 relevant tags
- Use ISO 8601 timestamp format with Z suffix
- `created_at` and `updated_at` start as same value

## Challenge Creation Workflow

1. **Plan the challenge**

   - Identify the Rust concept to teach
   - Determine appropriate difficulty level
   - Outline learning objectives

2. **Create directory structure**

   ```bash
   mkdir -p challenges/<slug>/{src,tests}
   ```

3. **Write files in order**

   - Start with description.md (clarify requirements)
   - Write src/lib.rs (complete solution)
   - Write tests/tests.rs (validate solution works)
   - Write src/starter.rs (scaffold for learners)
   - Write Cargo.toml (package config)

4. **Add metadata**

   - Get next available ID: `challenges::get_max_id() + 1`
   - Add entry to challenges.json

5. **Validate**

   ```bash
   # Run challenge tests
   cargo test -p <slug>

   # Run structure validation
   cd challenges && cargo test
   ```

6. **Test learner experience**
   - Copy src/starter.rs to src/lib.rs
   - Verify tests fail appropriately
   - Solve challenge yourself to ensure clarity

## Quality Checklist

Before submitting a challenge, verify:

- [ ] Description is clear and educational
- [ ] Starter code provides helpful scaffolding but is incomplete
- [ ] Solution is idiomatic Rust with good documentation
- [ ] Tests are comprehensive and well-organized
- [ ] Cargo.toml package name matches directory slug
- [ ] Metadata entry exists in challenges.json with unique ID
- [ ] All validation tests pass: `cargo test -p challenges`
- [ ] Challenge tests pass: `cargo test -p <slug>`
- [ ] Attempted solving from starter code to verify difficulty/clarity

## Common Pitfalls to Avoid

1. **Too much scaffolding** - Don't give away the solution
2. **Unclear requirements** - Be explicit about expected behavior
3. **Missing edge cases** - Test boundary conditions
4. **Over-complexity** - Focus on one concept at a time
5. **Mismatched names** - Directory, package, and slug must all match
6. **Duplicate IDs** - Always use `get_max_id() + 1`
7. **Invalid timestamps** - Must use ISO 8601 with Z suffix
8. **No examples** - Learners benefit from seeing expected behavior
9. **Condescending tone** - Be encouraging and supportive
10. **Missing hints** - Struggling learners need guidance
