# RUST_STD Track Progress

## Completed Challenges

### Module 1: String Operations (8/8 completed)

| # | Challenge | Slug | Difficulty | ID | Status |
|---|-----------|------|------------|-----|--------|
| 1 | String Basics | `string-basics` | BEGINNER | 93 | Done |
| 2 | String Manipulation | `string-manipulation` | EASY | 94 | Done |
| 3 | String Parsing | `string-parsing` | EASY | 95 | Done |
| 4 | String Building | `string-building` | MEDIUM | 96 | Done |
| 5 | Unicode and Graphemes | `unicode-operations` | MEDIUM | 97 | Done |
| 6 | String Iterators | `string-iterators` | EASY | 98 | Done |
| 7 | String Patterns | `string-patterns` | MEDIUM | 99 | Done |
| 8 | OsString and Platform Strings | `osstring-basics` | MEDIUM | 100 | Done |

### Module 2: Collections (9/9 completed)

| # | Challenge | Slug | Difficulty | ID | Status |
|---|-----------|------|------------|-----|--------|
| 9 | HashSet Operations | `hashset-operations` | EASY | 101 | Done |
| 10 | BTreeMap Basics | `btreemap-basics` | EASY | 102 | Done |
| 11 | VecDeque Usage | `vecdeque-queue` | MEDIUM | 103 | Done |
| 12 | Collection Conversions | `collection-conversions` | MEDIUM | 104 | Done |
| 13 | BTreeSet Ranges | `btreeset-ranges` | MEDIUM | 105 | Done |
| 14 | Entry API Pattern | `entry-api` | HARD | 106 | Done |
| 15 | Binary Heap Operations | `binary-heap-priority` | MEDIUM | 107 | Done |
| 16 | LinkedList Usage | `linkedlist-basics` | EASY | 108 | Done |
| 17 | HashMap Advanced | `hashmap-advanced` | MEDIUM | 109 | Done |

### Module 3: File I/O and Paths (7/7 completed)

| # | Challenge | Slug | Difficulty | ID | Status |
|---|-----------|------|------------|-----|--------|
| 18 | Reading Files | `reading-files` | BEGINNER | 110 | Done |
| 19 | Writing Files | `writing-files` | EASY | 111 | Done |
| 20 | Path Operations | `path-operations` | EASY | 112 | Done |
| 21 | Directory Traversal | `directory-traversal` | MEDIUM | 113 | Done |
| 22 | File Metadata | `file-metadata` | MEDIUM | 114 | Done |
| 23 | Stdio Operations | `stdio-operations` | EASY | 115 | Done |
| 24 | Temporary Files | `tempfile-operations` | MEDIUM | 116 | Done |

### Module 4: Advanced Iterators (6/6 completed)

| # | Challenge | Slug | Difficulty | ID | Status |
|---|-----------|------|------------|-----|--------|
| 25 | Iterator Combinators | `iterator-combinators` | EASY | 117 | Done |
| 26 | Iterator Inspection | `iterator-inspection` | MEDIUM | 118 | Done |
| 27 | Fold and Scan | `fold-and-scan` | MEDIUM | 119 | Done |
| 28 | Iterator Filtering | `iterator-filtering` | EASY | 120 | Done |
| 29 | Iterator Flattening | `iterator-flattening` | MEDIUM | 121 | Done |
| 30 | Custom Iterators | `custom-iterators` | HARD | 122 | Done |

### Module 5: Conversion Traits (1/6 completed)

| # | Challenge | Slug | Difficulty | ID | Status |
|---|-----------|------|------------|-----|--------|
| 31 | From and Into Traits | `from-into-traits` | EASY | 123 | Done |
| 32 | TryFrom and TryInto | `tryfrom-tryinto` | MEDIUM | - | Pending |
| 33 | AsRef and AsMut | `asref-asmut` | MEDIUM | - | Pending |
| 34 | Borrow and ToOwned | `borrow-toowned` | MEDIUM | - | Pending |
| 35 | Deref and DerefMut | `deref-derefmut` | HARD | - | Pending |
| 36 | ToString and Display | `tostring-display` | EASY | - | Pending |

### Module 6: Time and Environment (0/4 completed)

| # | Challenge | Slug | Difficulty | ID | Status |
|---|-----------|------|------------|-----|--------|
| 37 | Duration Operations | `duration-operations` | EASY | - | Pending |
| 38 | SystemTime Usage | `systemtime-usage` | MEDIUM | - | Pending |
| 39 | Environment Variables | `env-variables` | EASY | - | Pending |
| 40 | Process and Exit | `process-exit` | MEDIUM | - | Pending |

### Module 7: Number Handling (0/4 completed)

| # | Challenge | Slug | Difficulty | ID | Status |
|---|-----------|------|------------|-----|--------|
| 41 | Number Conversions | `number-conversions` | MEDIUM | - | Pending |
| 42 | Floating Point Edge Cases | `floating-point` | MEDIUM | - | Pending |
| 43 | Integer Parsing | `integer-parsing` | EASY | - | Pending |
| 44 | Number Formatting | `number-formatting` | EASY | - | Pending |

### Module 8: Derive Macros and Common Traits (0/6 completed)

| # | Challenge | Slug | Difficulty | ID | Status |
|---|-----------|------|------------|-----|--------|
| 45 | Clone and Copy | `clone-copy-traits` | BEGINNER | - | Pending |
| 46 | Debug and Display Derive | `debug-display-derive` | EASY | - | Pending |
| 47 | PartialEq and Eq | `partialeq-eq` | EASY | - | Pending |
| 48 | PartialOrd and Ord | `partialord-ord` | MEDIUM | - | Pending |
| 49 | Hash Trait | `hash-trait` | MEDIUM | - | Pending |
| 50 | Default Trait Patterns | `default-trait-patterns` | EASY | - | Pending |

### Module 9: Memory and Pointers (0/5 completed)

| # | Challenge | Slug | Difficulty | ID | Status |
|---|-----------|------|------------|-----|--------|
| 51 | Box and Heap Allocation | `box-heap-allocation` | EASY | - | Pending |
| 52 | Rc and Reference Counting | `rc-reference-counting` | MEDIUM | - | Pending |
| 53 | Arc and Thread Safety | `arc-thread-safety` | MEDIUM | - | Pending |
| 54 | Cell and RefCell | `cell-refcell` | HARD | - | Pending |
| 55 | Cow (Copy-on-Write) | `cow-copy-on-write` | MEDIUM | - | Pending |

---

## Summary

- **Total Challenges**: 55
- **Completed**: 31
- **Remaining**: 24
- **Progress**: 56.4%

## Changelog

### 2025-01-26
- Created `from-into-traits` challenge (ID: 123)
  - Implemented 5 conversion type pairs: Celsius/Fahrenheit, Rgb/HexColor, Email from &str and String, Point2D to Point3D, generic Wrapper<T>
  - Added 47 tests covering temperature conversions (freezing/boiling/body temp, round-trips), RGB to hex (colors, leading zeros), Email from various sources, Point2D to Point3D, and generic Wrapper with various types (integers, strings, custom structs, Option, Result)
  - All tests passing including 6 doc tests
  - Begins Module 5: Conversion Traits (1/6 challenges)

### 2025-01-25
- Created `custom-iterators` challenge (ID: 122)
  - Implemented 6 custom iterator types: `Fibonacci`, `StepRange`, `CycleN<T>`, `Collatz`, `Windows<T>`, `Unfold<T, F>`
  - Added 63 tests covering Fibonacci sequence (first N, skip/take, sum), StepRange (ascending/descending, edge cases), CycleN (repeat cycles, empty/single element), Collatz (various starting points, sequence length), Windows (overlapping slices, edge cases), Unfold (state-based generation, early termination), and integration tests (filter, chain, enumerate, zip, map)
  - All tests passing including 6 doc tests
  - Completes Module 4: Advanced Iterators (6/6 challenges)

- Created `iterator-flattening` challenge (ID: 121)
  - Implemented 8 functions: `flatten_nested`, `flatten_options`, `flatten_results`, `chars_from_words`, `expand_ranges`, `flatten_to_depth_one`, `words_from_lines`, `flatten_and_filter`
  - Added 68 tests covering nested vector flattening, Option/Result flattening, character extraction, range expansion, partial flattening, word extraction, filter after flatten, and integration tests (pipelines, text processing, combined operations)
  - All tests passing including 8 doc tests
  - Continues Module 4: Advanced Iterators (5/6 challenges)

- Created `iterator-filtering` challenge (ID: 120)
  - Implemented 8 functions: `filter_even`, `filter_by_predicate`, `parse_valid_numbers`, `filter_map_with`, `take_while_positive`, `skip_while_negative`, `filter_in_range`, `first_matching`
  - Added 69 tests covering filter operations, filter_map, take_while/skip_while, range filtering, first matching element, and integration tests (chained filtering, parse then filter, filter_map chain, complex pipeline, custom structs)
  - All tests passing including 8 doc tests
  - Continues Module 4: Advanced Iterators (4/6 challenges)

- Created `fold-and-scan` challenge (ID: 119)
  - Implemented 8 functions: `sum_with_fold`, `product_with_fold`, `concat_strings`, `running_sum`, `running_max`, `take_while_sum_under`, `count_occurrences`, `running_average`
  - Added 62 tests covering fold-based sum/product/concat/count, scan-based running sum/max/average, early termination with take_while_sum_under, and integration tests (fold-scan equivalence, monotonic running max, complex pipelines, custom types)
  - All tests passing including 8 doc tests
  - Continues Module 4: Advanced Iterators (3/6 challenges)

- Created `iterator-inspection` challenge (ID: 118)
  - Implemented 7 functions: `indexed_elements`, `find_index`, `elements_with_next`, `group_consecutive_duplicates`, `find_first_repeated`, `collect_with_trace`, `sum_with_running_total`
  - Added 55 tests covering enumerate-based indexing, find with index, peekable lookahead, consecutive duplicate grouping, first repeated element detection, inspect-based tracing, running totals, and integration tests (indexed find pattern, differences, longest run, trace pipeline, combined inspection, running total with trace)
  - All tests passing including 7 doc tests
  - Continues Module 4: Advanced Iterators (2/6 challenges)

- Created `iterator-combinators` challenge (ID: 117)
  - Implemented 7 functions: `chain_sequences`, `zip_pairs`, `take_first`, `skip_first`, `reverse_sequence`, `interleave`, `sliding_pairs`
  - Added 55 tests covering chain operations, zip pairing, take/skip operations, reversal, interleaving with unequal lengths, sliding pairs, and integration tests (chain then take/skip, reverse then take, interleave then reverse, complex pipelines)
  - All tests passing including 7 doc tests
  - Begins Module 4: Advanced Iterators (1/6 challenges)

- Created `tempfile-operations` challenge (ID: 116)
  - Implemented 6 functions/types: `get_temp_dir`, `create_temp_file`, `create_temp_file_with_content`, `TempFile` struct (with new/path/write/read/Drop), `create_temp_dir`, `cleanup_temp_files`
  - Added 38 tests covering temp directory access, unique file creation, content writing, TempFile RAII pattern (automatic cleanup on drop), temp directory creation, batch cleanup with prefix matching, and integration tests (workflow, RAII, processing pipeline, batch creation, nested dirs, concurrent files)
  - All tests passing including 10 doc tests
  - Completes Module 3: File I/O and Paths (7/7 challenges)

- Created `stdio-operations` challenge (ID: 115)
  - Implemented 6 functions: `read_line_from_reader`, `read_all_lines_from_reader`, `write_to_writer`, `writeln_to_writer`, `write_and_flush`, `write_error_to_writer`
  - Added 38 tests covering reading single/multiple lines, writing without/with newlines, flushing buffers, error formatting, Windows/Unix newlines, Unicode content, and integration tests (read-echo, prompt pattern, read-process-write, error handling, mixed operations, large input)
  - All tests passing including 6 doc tests
  - Continues Module 3: File I/O and Paths (6/7 challenges)

- Created `file-metadata` challenge (ID: 114)
  - Implemented 7 functions: `get_file_size`, `get_file_type`, `is_readonly`, `get_modified_time`, `was_modified_within`, `is_executable`, `compare_modified_times`
  - Added 35 tests covering file size (empty, basic, large, binary), file type (file, directory, symlink), read-only permissions, modification times, time comparisons, executable detection (Unix permissions), and integration tests (file lifecycle, directory structure, multiple file comparison, permissions workflow)
  - All tests passing including 7 doc tests
  - Continues Module 3: File I/O and Paths (5/7 challenges)

- Created `directory-traversal` challenge (ID: 113)
  - Implemented 7 functions: `list_files`, `list_directories`, `list_all_recursive`, `find_by_extension`, `find_by_name`, `calculate_dir_size`, `count_files_recursive`
  - Added 38 tests covering listing files/directories, recursive traversal, finding by extension/name, calculating directory sizes, counting files, and integration tests (complex structure, find all matches, size vs count)
  - All tests passing including 7 doc tests
  - Continues Module 3: File I/O and Paths (4/7 challenges)

- Created `path-operations` challenge (ID: 112)
  - Implemented 8 functions: `join_paths`, `get_extension`, `get_file_name`, `get_file_stem`, `get_parent`, `change_extension`, `is_absolute`, `normalize_path`
  - Added 53 tests covering path joining, extension extraction, file name/stem retrieval, parent directory, extension modification, absolute path detection, normalization, and integration tests
  - All tests passing including 8 doc tests
  - Continues Module 3: File I/O and Paths (3/7 challenges)

- Created `writing-files` challenge (ID: 111)
  - Implemented 5 functions: `write_string`, `write_bytes`, `append_string`, `write_lines`, `write_with_buffer`
  - Added 42 tests covering writing strings, bytes, appending, writing multiple lines, buffered writing, unicode content, overwriting files, error handling for invalid paths, and integration tests (write and append, string and bytes, lines and buffer, log file pattern, large buffered write)
  - All tests passing including 5 doc tests
  - Continues Module 3: File I/O and Paths (2/7 challenges)

- Created `reading-files` challenge (ID: 110)
  - Implemented 5 functions: `read_entire_file`, `count_lines`, `count_words`, `read_lines`, `first_n_lines`
  - Added 34 tests covering reading entire files, counting lines/words, reading lines into Vec, first N lines, empty files, unicode content, and error handling for non-existent files
  - All tests passing including 5 doc tests
  - Begins Module 3: File I/O and Paths (1/7 challenges)

- Created `hashmap-advanced` challenge (ID: 109)
  - Implemented 8 functions: `create_with_capacity`, `reserve_additional`, `shrink_map`, `bulk_insert`, `get_capacity_stats`, `clear_and_shrink`, `group_by_key`, `merge_with_capacity`
  - Added 44 tests covering capacity creation, reservation, shrinking, bulk insertion, capacity stats, clear and shrink, grouping by key, merging with capacity, and integration tests (capacity workflow, bulk operations, grouping/merging, memory-efficient processing)
  - All tests passing including 8 doc tests
  - Completes Module 2: Collections (9/9 challenges)

- Created `linkedlist-basics` challenge (ID: 108)
  - Implemented 9 functions: `create_list`, `add_front`, `add_back`, `remove_front`, `remove_back`, `peek_front`, `peek_back`, `move_to_front`, `concat_lists`
  - Added 48 tests covering list creation, add/remove from both ends, peeking, move-to-front (LRU pattern), list concatenation, and integration tests (LRU cache simulation, queue/stack/deque operations)
  - All tests passing including 9 doc tests
  - Continues Module 2: Collections (8/9 challenges)

- Created `binary-heap-priority` challenge (ID: 107)
  - Implemented 8 functions: `create_max_heap`, `create_min_heap`, `pop_max`, `peek_max`, `top_k_largest`, `top_k_smallest`, `merge_heaps`, `heap_sort_descending`
  - Added 52 tests covering max-heap creation, min-heap creation, pop/peek operations, top-K largest/smallest, heap merging, heap sort, and integration tests (task scheduler, median finding, streaming top-K)
  - All tests passing including 8 doc tests
  - Continues Module 2: Collections (7/9 challenges)

- Created `entry-api` challenge (ID: 106)
  - Implemented 7 functions: `count_words`, `group_by_length`, `get_or_compute`, `increment_or_init`, `merge_maps`, `first_occurrence`, `update_or_default`
  - Added 46 tests covering word counting, grouping by length, caching patterns, increment-or-init, map merging, first occurrence tracking, update-or-default, and integration tests
  - All tests passing including 7 doc tests
  - Continues Module 2: Collections (6/9 challenges)

- Created `btreeset-ranges` challenge (ID: 105)
  - Implemented 8 functions: `create_number_set`, `get_range`, `get_range_inclusive`, `get_elements_before`, `get_elements_from`, `count_in_range`, `find_closest_less_than`, `find_closest_greater_than`
  - Added 50 tests covering set creation, half-open ranges, inclusive ranges, unbounded ranges, counting, finding closest elements, edge cases with negative numbers and empty sets, and integration tests
  - All tests passing including 8 doc tests
  - Continues Module 2: Collections (5/9 challenges)

- Created `collection-conversions` challenge (ID: 104)
  - Implemented 8 functions: `vec_to_hashset`, `vec_to_btreeset`, `hashset_to_sorted_vec`, `pairs_to_hashmap`, `pairs_to_btreemap`, `hashmap_to_pairs`, `merge_vecs`, `chain_and_collect`
  - Added 49 tests covering all conversion functions, empty/single/multiple elements, duplicate handling, strings, and integration tests
  - All tests passing including 8 doc tests
  - Continues Module 2: Collections (4/9 challenges)

- Created `vecdeque-queue` challenge (ID: 103)
  - Implemented 7 functions: `create_queue`, `enqueue`, `dequeue`, `peek_front`, `peek_back`, `rotate_left`, `rotate_right`
  - Added 45 tests covering queue creation, FIFO operations, peeking, rotation (left/right), edge cases, and integration tests
  - All tests passing including 7 doc tests
  - Continues Module 2: Collections (3/9 challenges)

- Created `btreemap-basics` challenge (ID: 102)
  - Implemented 7 functions: `create_sorted_map`, `get_value`, `get_keys_in_order`, `get_values_in_key_order`, `get_range`, `get_first`, `get_last`
  - Added 36 tests covering map creation, key/value retrieval, sorted iteration, range queries, and first/last element access
  - All tests passing including 7 doc tests
  - Continues Module 2: Collections (2/9 challenges)

- Created `hashset-operations` challenge (ID: 101)
  - Implemented 7 functions: `unique_elements`, `count_unique`, `find_common`, `find_all`, `find_difference`, `find_symmetric_difference`, `is_subset`
  - Added 43 tests covering unique element extraction, set intersection, union, difference, symmetric difference, and subset checks
  - All tests passing including 7 doc tests
  - Begins Module 2: Collections (1/9 challenges)

- Created `osstring-basics` challenge (ID: 100)
  - Implemented 6 functions: `to_os_string`, `os_str_to_str`, `os_string_to_string_lossy`, `get_file_extension`, `join_path_components`, `is_valid_utf8`
  - Added 49 tests covering OsString/OsStr conversions, path operations, extension extraction, and UTF-8 validation
  - All tests passing including 6 doc tests
  - Completes Module 1: String Operations (8/8 challenges)

- Created `string-patterns` challenge (ID: 99)
  - Implemented 7 functions: `has_prefix`, `has_suffix`, `find_first`, `find_last`, `count_occurrences`, `find_all_indices`, `extract_between`
  - Added 43 tests covering prefix/suffix checks, pattern finding, counting occurrences, finding all indices, and extracting text between markers
  - All tests passing including 7 doc tests

- Created `string-iterators` challenge (ID: 98)
  - Implemented 6 functions: `chars_to_vec`, `words_to_vec`, `lines_to_vec`, `count_words`, `reverse_words`, `capitalize_words`
  - Added 37 tests covering character iteration, word splitting, line splitting, word counting, word reversal, and word capitalization
  - All tests passing including 6 doc tests

- Created `unicode-operations` challenge (ID: 97)
  - Implemented 5 functions: `char_count`, `byte_count`, `safe_substring`, `char_at`, `is_single_char`
  - Added 32 tests covering ASCII, Cyrillic, Chinese, emojis, and edge cases (combining characters, ZWJ sequences)
  - All tests passing including 5 doc tests

- Created `string-building` challenge (ID: 96)
  - Implemented 5 components: `build_greeting`, `build_list`, `Person` with `Display`, `build_table`, `concat_with_separator`
  - Added 32 tests covering greeting formatting, numbered lists, Display trait, table building, and string concatenation
  - All tests passing including 5 doc tests

- Created `string-parsing` challenge (ID: 95)
  - Implemented 5 components: `parse_int`, `parse_bool`, `parse_key_value`, `Color` with `FromStr`, `parse_list<T>`
  - Added 43 tests covering integer parsing, boolean parsing, key-value parsing, Color struct parsing, and generic list parsing
  - All tests passing including 5 doc tests

- Created `string-manipulation` challenge (ID: 94)
  - Implemented 5 functions: `clean_string`, `contains_word`, `replace_word`, `split_and_trim`, `normalize_whitespace`
  - Added 28 tests covering trimming, case conversion, searching, replacing, and splitting
  - All tests passing including 5 doc tests

- Created `string-basics` challenge (ID: 93)
  - Added RUST_STD track to Track enum in `challenges/lib.rs`
  - Implemented 5 functions: `to_owned_string`, `count_chars`, `count_bytes`, `is_ascii_only`, `first_char`
  - Added 17 tests covering ASCII, Unicode, empty strings, and edge cases
  - All tests passing including doc tests
