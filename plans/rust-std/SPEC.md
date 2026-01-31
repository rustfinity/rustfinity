# Plan: Create Rust Standard Library Track

## Overview

Create a new comprehensive track `RUST_STD` focusing on teaching Rust's standard library components. This track provides deep coverage of stdlib modules, fills critical gaps in the current curriculum, and includes some intentional overlap with RUST_BASICS to demonstrate stdlib-specific patterns.

## Background

### Current State Analysis

- **Existing Tracks**: RUST_BASICS (76 challenges), CONTROL_FLOW (15 challenges)
- **Well-Covered**: Traits (18), structs (11), error handling (8), basic collections
- **Critical Gaps Identified**:
  1. String manipulation (only basic printing exists)
  2. Advanced collection types (HashSet, BTreeMap, VecDeque)
  3. File I/O and Path operations (only 1 basic challenge)
  4. Iterator combinators beyond basics
  5. Time/Duration operations
  6. Environment variables and system interaction
  7. Number conversions and edge cases
  8. Standard conversion traits (From/Into/AsRef/AsMut)
  9. Formatting and display traits
  10. Commonly used derive macros

### Track Design Philosophy

- **Progression**: BEGINNER → EASY → MEDIUM → HARD
- **Focus**: Practical standard library APIs used in real-world Rust
- **Scope**: 55 challenges covering comprehensive stdlib modules
- **Prerequisites**: Basic Rust knowledge (variables, functions, ownership)
- **Overlap Strategy**: Some intentional overlap with RUST_BASICS to show stdlib patterns (From/Into, derive macros, Display trait)
- **Exclusions**: No unsafe code, FFI, low-level async, or external crates

## Proposed Challenge Breakdown (55 Challenges Total)

### Module 1: String Operations (8 challenges)

#### 1. String Basics (BEGINNER)

- **Slug**: `string-basics`
- **Topics**: String vs &str, `.to_string()`, `String::from()`, `.chars()`, `.bytes()`
- **Task**: Implement functions to convert, count characters, check if ASCII
- **Learning Goals**: Understand String/&str distinction, UTF-8 encoding

#### 2. String Manipulation (EASY)

- **Slug**: `string-manipulation`
- **Topics**: `.split()`, `.trim()`, `.contains()`, `.replace()`, `.to_lowercase()`, `.to_uppercase()`
- **Task**: Parse CSV-like data, clean whitespace, find/replace patterns
- **Learning Goals**: Master common string methods

#### 3. String Parsing (EASY)

- **Slug**: `string-parsing`
- **Topics**: `.parse()`, `FromStr` trait, error handling with Result
- **Task**: Parse structured data (key=value pairs) into typed structs
- **Learning Goals**: Type conversions from strings with error handling

#### 4. String Building (MEDIUM)

- **Slug**: `string-building`
- **Topics**: `format!()`, `write!()`, custom Display implementations, string concatenation
- **Task**: Build formatted output (table, report) using various techniques
- **Learning Goals**: Efficient string construction, formatting traits

#### 5. Unicode and Graphemes (MEDIUM)

- **Slug**: `unicode-operations`
- **Topics**: `.chars()` vs `.bytes()`, character boundaries, string slicing safety
- **Task**: Count grapheme clusters, safely slice strings, handle emojis
- **Learning Goals**: Unicode awareness, safe string manipulation

#### 6. String Iterators (EASY)

- **Slug**: `string-iterators`
- **Topics**: `.chars()`, `.split_whitespace()`, `.lines()`, collecting strings
- **Task**: Iterate over characters, words, lines; transform and collect
- **Learning Goals**: String iteration patterns

#### 7. String Patterns (MEDIUM)

- **Slug**: `string-patterns`
- **Topics**: `.starts_with()`, `.ends_with()`, `.find()`, `.rfind()`, pattern matching
- **Task**: Search for substrings, validate prefixes/suffixes, extract matches
- **Learning Goals**: String searching and pattern matching

#### 8. OsString and Platform Strings (MEDIUM)

- **Slug**: `osstring-basics`
- **Topics**: `OsString`, `OsStr`, platform-specific strings, conversions
- **Task**: Handle file paths from OS, convert between String/OsString
- **Learning Goals**: Platform-agnostic string handling

### Module 2: Collections (9 challenges)

#### 9. HashSet Operations (EASY)

- **Slug**: `hashset-operations`
- **Topics**: `HashSet::new()`, `.insert()`, `.contains()`, `.union()`, `.intersection()`, `.difference()`
- **Task**: Implement set operations (unique elements, common items, symmetric difference)
- **Learning Goals**: Set theory in Rust, deduplication patterns

#### 10. BTreeMap Basics (EASY)

- **Slug**: `btreemap-basics`
- **Topics**: `BTreeMap`, ordered iteration, `.range()`, key ordering
- **Task**: Build a sorted index, range queries, ordered traversal
- **Learning Goals**: When to use BTreeMap vs HashMap

#### 11. VecDeque Usage (MEDIUM)

- **Slug**: `vecdeque-queue`
- **Topics**: `VecDeque`, `.push_back()`, `.pop_front()`, double-ended operations
- **Task**: Implement a task queue with priority reordering
- **Learning Goals**: Efficient queue operations, when VecDeque outperforms Vec

#### 12. Collection Conversions (MEDIUM)

- **Slug**: `collection-conversions`
- **Topics**: `.collect()`, `FromIterator`, `IntoIterator`, type inference
- **Task**: Convert between Vec/HashSet/BTreeMap, chain operations
- **Learning Goals**: Iterator-to-collection patterns, type-driven collection

#### 13. BTreeSet Ranges (MEDIUM)

- **Slug**: `btreeset-ranges`
- **Topics**: `BTreeSet`, `.range()`, ordered set operations, custom Ord
- **Task**: Time-based event filtering, range-based queries
- **Learning Goals**: Ordered sets for range queries

#### 14. Entry API Pattern (HARD)

- **Slug**: `entry-api`
- **Topics**: `.entry()`, `.or_insert()`, `.or_insert_with()`, `.and_modify()`
- **Task**: Build a word frequency counter, cache with computed values
- **Learning Goals**: Efficient map updates without double lookups

#### 15. Binary Heap Operations (MEDIUM)

- **Slug**: `binary-heap-priority`
- **Topics**: `BinaryHeap`, priority queue, `.push()`, `.pop()`, custom Ord
- **Task**: Implement task scheduler, find top-K elements
- **Learning Goals**: Priority queues and heap data structure

#### 16. LinkedList Usage (EASY)

- **Slug**: `linkedlist-basics`
- **Topics**: `LinkedList`, `.push_front()`, `.push_back()`, when to use LinkedList
- **Task**: Implement simple LRU cache behavior
- **Learning Goals**: LinkedList API and appropriate use cases

#### 17. HashMap Advanced (MEDIUM)

- **Slug**: `hashmap-advanced`
- **Topics**: Custom hashers, capacity planning, `.reserve()`, `.shrink_to_fit()`
- **Task**: Optimize HashMap for performance, grouping operations
- **Learning Goals**: HashMap performance optimization

### Module 3: File I/O and Paths (7 challenges)

#### 18. Reading Files (BEGINNER)

- **Slug**: `reading-files`
- **Topics**: `File::open()`, `read_to_string()`, `BufReader`, `lines()`
- **Task**: Read file contents, count lines/words, parse line-by-line
- **Learning Goals**: Basic file reading patterns

#### 19. Writing Files (EASY)

- **Slug**: `writing-files`
- **Topics**: `File::create()`, `.write_all()`, `BufWriter`, appending
- **Task**: Write data to file, append to log, buffered writes
- **Learning Goals**: File writing patterns, buffering for performance

#### 20. Path Operations (EASY)

- **Slug**: `path-operations`
- **Topics**: `Path`, `PathBuf`, `.join()`, `.extension()`, `.file_name()`, `.parent()`
- **Task**: Build paths safely, extract components, check extensions
- **Learning Goals**: Cross-platform path handling

#### 21. Directory Traversal (MEDIUM)

- **Slug**: `directory-traversal`
- **Topics**: `read_dir()`, recursive traversal, filtering, metadata
- **Task**: Find all files matching pattern, compute directory sizes
- **Learning Goals**: Walking directory trees, filesystem metadata

#### 22. File Metadata (MEDIUM)

- **Slug**: `file-metadata`
- **Topics**: `.metadata()`, file types, permissions, modified time
- **Task**: Filter by modification date, check if executable, file type detection
- **Learning Goals**: Working with filesystem metadata

#### 23. Stdio Operations (EASY)

- **Slug**: `stdio-operations`
- **Topics**: `stdin()`, `stdout()`, `stderr()`, `.read_line()`, `.flush()`
- **Task**: Read user input, write to stdout/stderr, flush buffers
- **Learning Goals**: Standard I/O operations

#### 24. Temporary Files (MEDIUM)

- **Slug**: `tempfile-operations`
- **Topics**: `std::env::temp_dir()`, creating temp files, cleanup patterns
- **Task**: Create temporary files for processing, ensure cleanup
- **Learning Goals**: Working with temporary files safely

### Module 4: Advanced Iterators (6 challenges)

#### 25. Iterator Combinators (EASY)

- **Slug**: `iterator-combinators`
- **Topics**: `.chain()`, `.zip()`, `.take()`, `.skip()`, `.rev()`
- **Task**: Combine multiple sequences, pair elements, sliding windows
- **Learning Goals**: Composing iterator operations

#### 26. Iterator Inspection (MEDIUM)

- **Slug**: `iterator-inspection`
- **Topics**: `.enumerate()`, `.peekable()`, `.inspect()`
- **Task**: Process with indices, lookahead parsing, debugging pipelines
- **Learning Goals**: Inspecting iterator state

#### 27. Fold and Scan (MEDIUM)

- **Slug**: `fold-and-scan`
- **Topics**: `.fold()`, `.scan()`, stateful iteration
- **Task**: Running totals, cumulative operations, custom aggregations
- **Learning Goals**: Stateful iterator transformations

#### 28. Iterator Filtering (EASY)

- **Slug**: `iterator-filtering`
- **Topics**: `.filter()`, `.filter_map()`, `.take_while()`, `.skip_while()`
- **Task**: Filter and transform data, conditional iteration
- **Learning Goals**: Filtering patterns in iterators

#### 29. Iterator Flattening (MEDIUM)

- **Slug**: `iterator-flattening`
- **Topics**: `.flatten()`, `.flat_map()`, nested iteration
- **Task**: Process nested collections, flatten hierarchies
- **Learning Goals**: Working with nested iterators

#### 30. Custom Iterators (HARD)

- **Slug**: `custom-iterators`
- **Topics**: Implementing `Iterator` trait, `.next()`, state management
- **Task**: Build a Fibonacci iterator, range with step, custom sequence
- **Learning Goals**: Creating reusable iterators

### Module 5: Conversion Traits (6 challenges)

#### 31. From and Into Traits (EASY)

- **Slug**: `from-into-traits`
- **Topics**: `From`, `Into`, automatic trait implementations
- **Task**: Implement conversions between custom types
- **Learning Goals**: Standard conversion patterns

#### 32. TryFrom and TryInto (MEDIUM)

- **Slug**: `tryfrom-tryinto`
- **Topics**: `TryFrom`, `TryInto`, fallible conversions
- **Task**: Implement validated conversions with error handling
- **Learning Goals**: Safe fallible conversions

#### 33. AsRef and AsMut (MEDIUM)

- **Slug**: `asref-asmut`
- **Topics**: `AsRef`, `AsMut`, borrowing conversions
- **Task**: Write generic functions accepting multiple reference types
- **Learning Goals**: Reference conversion traits

#### 34. Borrow and ToOwned (MEDIUM)

- **Slug**: `borrow-toowned`
- **Topics**: `Borrow`, `ToOwned`, `Cow`, owned vs borrowed
- **Task**: Implement efficient string/data handling with Cow
- **Learning Goals**: Owned vs borrowed patterns

#### 35. Deref and DerefMut (HARD)

- **Slug**: `deref-derефmut`
- **Topics**: `Deref`, `DerefMut`, deref coercion, smart pointers
- **Task**: Implement wrapper types with deref coercion
- **Learning Goals**: Deref trait and smart pointer patterns

#### 36. ToString and Display (EASY)

- **Slug**: `tostring-display`
- **Topics**: `ToString`, `Display`, `Debug`, formatting
- **Task**: Implement Display for custom types, format output
- **Learning Goals**: String representation traits

### Module 6: Time and Environment (4 challenges)

#### 37. Duration Operations (EASY)

- **Slug**: `duration-operations`
- **Topics**: `Duration`, `.as_secs()`, `.as_millis()`, arithmetic
- **Task**: Time formatting, duration comparisons, elapsed time calculations
- **Learning Goals**: Working with time spans

#### 38. SystemTime Usage (MEDIUM)

- **Slug**: `systemtime-usage`
- **Topics**: `SystemTime::now()`, `UNIX_EPOCH`, duration since epoch
- **Task**: Timestamp generation, time comparisons, timeout checking
- **Learning Goals**: Wall clock time operations

#### 39. Environment Variables (EASY)

- **Slug**: `env-variables`
- **Topics**: `env::var()`, `env::args()`, `env::current_dir()`
- **Task**: Read config from env, parse CLI args, get working directory
- **Learning Goals**: System environment interaction

#### 40. Process and Exit (MEDIUM)

- **Slug**: `process-exit`
- **Topics**: `std::process::exit()`, exit codes, `std::process::id()`
- **Task**: Handle program exit, set exit codes, get process info
- **Learning Goals**: Process lifecycle management

### Module 7: Number Handling (4 challenges)

#### 41. Number Conversions (MEDIUM)

- **Slug**: `number-conversions`
- **Topics**: `TryFrom`/`TryInto`, `.checked_add()`, `.saturating_mul()`, `.wrapping_sub()`
- **Task**: Safe type conversions, overflow detection, saturating arithmetic
- **Learning Goals**: Handling numeric edge cases safely

#### 42. Floating Point Edge Cases (MEDIUM)

- **Slug**: `floating-point`
- **Topics**: NaN, infinity, `.is_finite()`, `.round()`, `.trunc()`, comparison issues
- **Task**: Validate floats, handle special values, approximate comparisons
- **Learning Goals**: Floating-point pitfalls and best practices

#### 43. Integer Parsing (EASY)

- **Slug**: `integer-parsing`
- **Topics**: `.parse()`, radix parsing, `from_str_radix()`, error handling
- **Task**: Parse integers in different bases, handle parse errors
- **Learning Goals**: Integer parsing patterns

#### 44. Number Formatting (EASY)

- **Slug**: `number-formatting`
- **Topics**: Format specifiers, padding, precision, hex/octal/binary
- **Task**: Format numbers with various specifications
- **Learning Goals**: Number formatting patterns

### Module 8: Derive Macros and Common Traits (6 challenges)

#### 45. Clone and Copy (BEGINNER)

- **Slug**: `clone-copy-traits`
- **Topics**: `Clone`, `Copy`, derive macros, deep vs shallow copy
- **Task**: Implement Clone for custom types, understand Copy constraints
- **Learning Goals**: Copying and cloning patterns

#### 46. Debug and Display Derive (EASY)

- **Slug**: `debug-display-derive`
- **Topics**: `#[derive(Debug)]`, implementing `Display`, pretty printing
- **Task**: Add debug formatting, custom display implementations
- **Learning Goals**: Formatting traits and derives

#### 47. PartialEq and Eq (EASY)

- **Slug**: `partialeq-eq`
- **Topics**: `PartialEq`, `Eq`, custom equality, derive macros
- **Task**: Implement custom equality for types
- **Learning Goals**: Equality traits and semantics

#### 48. PartialOrd and Ord (MEDIUM)

- **Slug**: `partialord-ord`
- **Topics**: `PartialOrd`, `Ord`, `Ordering`, custom ordering
- **Task**: Implement custom sorting logic for types
- **Learning Goals**: Ordering traits and comparisons

#### 49. Hash Trait (MEDIUM)

- **Slug**: `hash-trait`
- **Topics**: `Hash`, custom hashing, using types as HashMap keys
- **Task**: Implement Hash for custom types, use in HashSet/HashMap
- **Learning Goals**: Hashing for collections

#### 50. Default Trait Patterns (EASY)

- **Slug**: `default-trait-patterns`
- **Topics**: `Default`, struct update syntax, builder patterns
- **Task**: Implement Default, use in configuration patterns
- **Learning Goals**: Default value patterns

### Module 9: Memory and Pointers (5 challenges)

#### 51. Box and Heap Allocation (EASY)

- **Slug**: `box-heap-allocation`
- **Topics**: `Box`, heap vs stack, recursive types
- **Task**: Box large data, implement recursive data structures
- **Learning Goals**: Heap allocation patterns

#### 52. Rc and Reference Counting (MEDIUM)

- **Slug**: `rc-reference-counting`
- **Topics**: `Rc`, `.clone()`, strong/weak counts, shared ownership
- **Task**: Implement shared data structures, circular references with Weak
- **Learning Goals**: Reference counting patterns

#### 53. Arc and Thread Safety (MEDIUM)

- **Slug**: `arc-thread-safety`
- **Topics**: `Arc`, atomic reference counting, thread-safe sharing
- **Task**: Share data across threads safely
- **Learning Goals**: Thread-safe reference counting

#### 54. Cell and RefCell (HARD)

- **Slug**: `cell-refcell`
- **Topics**: `Cell`, `RefCell`, interior mutability, borrow checking at runtime
- **Task**: Implement interior mutability patterns, handle runtime borrow errors
- **Learning Goals**: Interior mutability patterns

#### 55. Cow (Copy-on-Write) (MEDIUM)

- **Slug**: `cow-copy-on-write`
- **Topics**: `Cow`, borrowed vs owned, `.to_mut()`, efficient string handling
- **Task**: Use Cow for efficient data handling, minimize clones
- **Learning Goals**: Copy-on-write optimization patterns

## Implementation Plan

### Phase 1: Metadata Setup

1. **Get next challenge ID**: Use `challenges::get_max_id() + 1` (currently 92)
2. **Add 55 entries to challenges.json**:
   - IDs: 92-146 (sequential)
   - Track: `"RUST_STD"`
   - Difficulty distribution:
     - BEGINNER: 3 challenges (string-basics, reading-files, clone-copy-traits)
     - EASY: 23 challenges
     - MEDIUM: 25 challenges
     - HARD: 4 challenges (entry-api, custom-iterators, deref-derefmut, cell-refcell)
   - Add ISO 8601 timestamps for created_at/updated_at
   - Tags: Based on topics covered (e.g., ["strings", "unicode", "parsing", "collections", "io", "iterators", "conversion-traits", "time", "numbers", "derive-macros", "smart-pointers"])

### Phase 2: Challenge Creation Order

Create challenges in module order (easier to maintain consistency):

#### Suggested Creation Priority:

1. **Start with Module 1 (Strings)** - Foundation for text processing
2. **Module 8 (Derive Macros)** - Useful across other modules
3. **Module 5 (Conversion Traits)** - Core patterns used everywhere
4. **Module 2 (Collections)** - Builds on conversion traits
5. **Module 4 (Iterators)** - Works with collections
6. **Module 3 (File I/O)** - Combines strings, conversions, error handling
7. **Module 7 (Numbers)** - Standalone numeric topics
8. **Module 6 (Time/Environment)** - System interaction
9. **Module 9 (Memory/Pointers)** - Advanced smart pointer topics

For each challenge:

1. **Create directory**: `mkdir -p challenges/<slug>/{src,tests}`
2. **Write description.md**: Follow patterns from existing challenges
   - Introduction explaining the stdlib concept (2-3 paragraphs)
   - Real-world motivation ("why this matters")
   - Clear requirements with bullet points
   - Code examples with `assert_eq!()` statements
   - Collapsible hints section
3. **Write src/lib.rs**: Complete, documented solution
   - Doc comments with examples
   - Idiomatic Rust
   - Use appropriate stdlib APIs
   - Show best practices
4. **Write tests/tests.rs**: Comprehensive test suite
   - 6-10 tests per challenge
   - Edge cases (empty input, large input, special characters for strings)
   - Boundary conditions
   - Multiple type tests for generics
5. **Write src/starter.rs**: Incomplete template
   - Function signatures with `// TODO:` comments
   - Helpful scaffolding without giving away solution
   - `main()` function showing usage examples
6. **Write Cargo.toml**: Package name matches slug exactly

### Phase 3: Validation (Per Challenge)

```bash
# Test the solution
cargo test -p <slug>

# Validate structure
cd challenges && cargo test
```

### Phase 4: Learner Experience Testing (Per Challenge)

1. Copy `src/starter.rs` to `src/lib.rs`
2. Verify tests fail appropriately
3. Solve the challenge to ensure clarity
4. Adjust description/hints if needed

### Phase 5: Final Validation (All 55 Challenges)

```bash
# Run all RUST_STD challenge tests
for slug in string-basics string-manipulation ... cow-copy-on-write; do
  cargo test -p $slug || echo "Failed: $slug"
done

# Validate all challenge structures
cd challenges && cargo test

# Verify metadata
cd challenges && cargo run --bin cli
```

## Critical Files to Modify

1. **challenges/challenges.json**
   - Add 55 new challenge entries (IDs 92-146)
   - Each with track: "RUST_STD"

2. **New challenge directories** (55 total):

   **Module 1 - Strings (8):**
   - `challenges/string-basics/`
   - `challenges/string-manipulation/`
   - `challenges/string-parsing/`
   - `challenges/string-building/`
   - `challenges/unicode-operations/`
   - `challenges/string-iterators/`
   - `challenges/string-patterns/`
   - `challenges/osstring-basics/`

   **Module 2 - Collections (9):**
   - `challenges/hashset-operations/`
   - `challenges/btreemap-basics/`
   - `challenges/vecdeque-queue/`
   - `challenges/collection-conversions/`
   - `challenges/btreeset-ranges/`
   - `challenges/entry-api/`
   - `challenges/binary-heap-priority/`
   - `challenges/linkedlist-basics/`
   - `challenges/hashmap-advanced/`

   **Module 3 - File I/O and Paths (7):**
   - `challenges/reading-files/`
   - `challenges/writing-files/`
   - `challenges/path-operations/`
   - `challenges/directory-traversal/`
   - `challenges/file-metadata/`
   - `challenges/stdio-operations/`
   - `challenges/tempfile-operations/`

   **Module 4 - Advanced Iterators (6):**
   - `challenges/iterator-combinators/`
   - `challenges/iterator-inspection/`
   - `challenges/fold-and-scan/`
   - `challenges/iterator-filtering/`
   - `challenges/iterator-flattening/`
   - `challenges/custom-iterators/`

   **Module 5 - Conversion Traits (6):**
   - `challenges/from-into-traits/`
   - `challenges/tryfrom-tryinto/`
   - `challenges/asref-asmut/`
   - `challenges/borrow-toowned/`
   - `challenges/deref-derefmut/`
   - `challenges/tostring-display/`

   **Module 6 - Time and Environment (4):**
   - `challenges/duration-operations/`
   - `challenges/systemtime-usage/`
   - `challenges/env-variables/`
   - `challenges/process-exit/`

   **Module 7 - Number Handling (4):**
   - `challenges/number-conversions/`
   - `challenges/floating-point/`
   - `challenges/integer-parsing/`
   - `challenges/number-formatting/`

   **Module 8 - Derive Macros and Common Traits (6):**
   - `challenges/clone-copy-traits/`
   - `challenges/debug-display-derive/`
   - `challenges/partialeq-eq/`
   - `challenges/partialord-ord/`
   - `challenges/hash-trait/`
   - `challenges/default-trait-patterns/`

   **Module 9 - Memory and Pointers (5):**
   - `challenges/box-heap-allocation/`
   - `challenges/rc-reference-counting/`
   - `challenges/arc-thread-safety/`
   - `challenges/cell-refcell/`
   - `challenges/cow-copy-on-write/`

## Quality Standards

### Description.md Template

```markdown
[Concept introduction - 2-3 paragraphs explaining the stdlib feature]

[Real-world motivation - why this matters]

## Your Task

- Implement function X that...
- The function should...
- [Specific requirements as bullets]

## Example

\`\`\`rust
// Example usage
assert_eq!(function_name(input), expected_output);
\`\`\`

## Hints

<details>
  <summary>Click here for hints</summary>

- Use `std::module::Type` for...
- Consider the `.method()` for...
- [Helpful guidance without full solution]

</details>
```

### Test Coverage Checklist

- [ ] Empty/zero input
- [ ] Single element
- [ ] Multiple elements
- [ ] Edge cases specific to topic (e.g., Unicode for strings, NaN for floats)
- [ ] Boundary conditions
- [ ] Error cases (for Result-returning functions)

### Starter Code Pattern

```rust
// Provide struct definitions if needed
pub struct Thing { /* fields */ }

// Function signatures with TODOs
pub fn do_something(input: Type) -> ReturnType {
    // TODO: Implement using std::module::API
    unimplemented!()
}

// Example usage in main
pub fn main() {
    let result = do_something(example_input);
    println!("{:?}", result);
}
```

## Verification Steps

After creating all 25 challenges:

1. **Metadata validation**:

   ```bash
   cd challenges && cargo test
   ```

   This validates:
   - No duplicate IDs or slugs
   - All required files exist
   - Package names match directory names
   - Valid timestamps

2. **Challenge tests**:

   ```bash
   # Test all new challenges
   for slug in string-basics string-manipulation ...; do
     cargo test -p $slug
   done
   ```

3. **Manual review**:
   - Difficulty progression is smooth within each module
   - Descriptions are clear and educational
   - Hints provide guidance without spoiling
   - Starter code is incomplete but helpful

## Expected Outcomes

### Learning Path

Students completing this track will gain practical experience with:

- **String handling**: Manipulation, parsing, Unicode, platform strings
- **Collections mastery**: HashSet, BTreeMap, VecDeque, BinaryHeap, LinkedList, Entry API
- **File I/O**: Reading, writing, paths, directories, metadata, stdio, temp files
- **Iterator expertise**: Combinators, filtering, flattening, fold/scan, custom iterators
- **Conversion patterns**: From/Into, TryFrom/TryInto, AsRef/AsMut, Borrow/ToOwned, Deref
- **System interaction**: Time/Duration, SystemTime, environment variables, process management
- **Number handling**: Safe conversions, floating-point edge cases, parsing, formatting
- **Derive macros**: Clone/Copy, Debug/Display, equality, ordering, hashing, Default
- **Smart pointers**: Box, Rc/Arc, Cell/RefCell, Cow (copy-on-write)

### Platform Benefits

- Fills critical gaps in stdlib coverage
- Provides 55 new challenges (60% expansion relative to RUST_BASICS)
- Creates a comprehensive track for practical Rust skills
- Includes intentional overlap with RUST_BASICS to show stdlib-specific patterns
- Enables near-complete Rust stdlib education
- Similar scale to existing RUST_BASICS track (76 challenges)

### Success Metrics

- All 55 challenges pass structure validation
- Each challenge has 6-10 comprehensive tests
- Starter code provides helpful scaffolding but is incomplete
- Complete solutions pass all tests
- Descriptions are clear and educational (verified through self-solving)
- Smooth difficulty progression within each module
- Challenges reference each other where appropriate (e.g., "now that you've learned From/Into...")

## Notes

- **Track name**: `RUST_STD` follows existing convention (short, uppercase)
- **Challenge count**: 55 challenges provides comprehensive coverage without redundancy
- **Difficulty distribution**: 3 BEGINNER, 23 EASY, 25 MEDIUM, 4 HARD
  - Weighted toward EASY/MEDIUM (appropriate for stdlib APIs)
  - HARD challenges focus on complex patterns (Entry API, custom iterators, Deref, RefCell)
- **Prerequisites**: Basic Rust knowledge (variables, functions, ownership basics)
- **Intentional overlap**: Some topics like From/Into, derive macros, Display covered to show stdlib patterns
- **Exclusions**: No unsafe code, FFI, low-level async, or external crates (per user requirements)
- **Module organization**: 9 modules grouped by topic for logical progression
- **Future expansion**: Could add advanced topics later (unsafe, FFI, module system, procedural macros) as separate track
