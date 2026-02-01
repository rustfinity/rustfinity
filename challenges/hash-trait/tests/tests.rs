use hash_trait::*;
use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};

// Helper function to compute hash for testing
fn compute_hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

// ============================================================================
// Point tests
// ============================================================================

#[test]
fn point_equal_points_same_hash() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    assert_eq!(p1, p2);
    assert_eq!(compute_hash(&p1), compute_hash(&p2));
}

#[test]
fn point_different_points_likely_different_hash() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    assert_ne!(p1, p2);
    // Different points should (usually) have different hashes
    // This is not guaranteed but highly likely for distinct values
    assert_ne!(compute_hash(&p1), compute_hash(&p2));
}

#[test]
fn point_in_hashset() {
    let mut set: HashSet<Point> = HashSet::new();
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 3, y: 4 };

    assert!(set.insert(p1));
    assert!(!set.insert(p2)); // Duplicate
    assert!(set.insert(p3));
    assert_eq!(set.len(), 2);
}

#[test]
fn point_in_hashmap() {
    let mut map: HashMap<Point, &str> = HashMap::new();
    map.insert(Point { x: 0, y: 0 }, "origin");
    map.insert(Point { x: 1, y: 0 }, "right");
    map.insert(Point { x: 0, y: 1 }, "up");

    assert_eq!(map.get(&Point { x: 0, y: 0 }), Some(&"origin"));
    assert_eq!(map.get(&Point { x: 1, y: 0 }), Some(&"right"));
    assert_eq!(map.get(&Point { x: 2, y: 2 }), None);
}

#[test]
fn point_negative_coordinates() {
    let p1 = Point { x: -1, y: -2 };
    let p2 = Point { x: -1, y: -2 };
    assert_eq!(p1, p2);
    assert_eq!(compute_hash(&p1), compute_hash(&p2));
}

#[test]
fn point_zero() {
    let p = Point { x: 0, y: 0 };
    let mut set: HashSet<Point> = HashSet::new();
    set.insert(p);
    assert!(set.contains(&Point { x: 0, y: 0 }));
}

// ============================================================================
// UserId tests
// ============================================================================

#[test]
fn userid_equal_ids_same_hash() {
    let id1 = UserId(42);
    let id2 = UserId(42);
    assert_eq!(id1, id2);
    assert_eq!(compute_hash(&id1), compute_hash(&id2));
}

#[test]
fn userid_different_ids_different_hash() {
    let id1 = UserId(1);
    let id2 = UserId(2);
    assert_ne!(id1, id2);
    assert_ne!(compute_hash(&id1), compute_hash(&id2));
}

#[test]
fn userid_in_hashset() {
    let mut set: HashSet<UserId> = HashSet::new();
    set.insert(UserId(1));
    set.insert(UserId(2));
    set.insert(UserId(1)); // Duplicate

    assert_eq!(set.len(), 2);
    assert!(set.contains(&UserId(1)));
    assert!(set.contains(&UserId(2)));
    assert!(!set.contains(&UserId(3)));
}

#[test]
fn userid_in_hashmap() {
    let mut scores: HashMap<UserId, u32> = HashMap::new();
    scores.insert(UserId(1), 100);
    scores.insert(UserId(2), 200);

    assert_eq!(scores.get(&UserId(1)), Some(&100));
    assert_eq!(scores.get(&UserId(2)), Some(&200));
    assert_eq!(scores.get(&UserId(3)), None);
}

#[test]
fn userid_zero() {
    let id = UserId(0);
    let mut set: HashSet<UserId> = HashSet::new();
    set.insert(id);
    assert!(set.contains(&UserId(0)));
}

#[test]
fn userid_max() {
    let id = UserId(u64::MAX);
    let mut set: HashSet<UserId> = HashSet::new();
    set.insert(id);
    assert!(set.contains(&UserId(u64::MAX)));
}

// ============================================================================
// CaseInsensitiveString tests
// ============================================================================

#[test]
fn case_insensitive_same_case_equal() {
    let s1 = CaseInsensitiveString::new("hello");
    let s2 = CaseInsensitiveString::new("hello");
    assert_eq!(s1, s2);
    assert_eq!(compute_hash(&s1), compute_hash(&s2));
}

#[test]
fn case_insensitive_different_case_equal() {
    let s1 = CaseInsensitiveString::new("Hello");
    let s2 = CaseInsensitiveString::new("HELLO");
    let s3 = CaseInsensitiveString::new("hElLo");
    assert_eq!(s1, s2);
    assert_eq!(s2, s3);
    assert_eq!(compute_hash(&s1), compute_hash(&s2));
    assert_eq!(compute_hash(&s2), compute_hash(&s3));
}

#[test]
fn case_insensitive_different_strings_not_equal() {
    let s1 = CaseInsensitiveString::new("hello");
    let s2 = CaseInsensitiveString::new("world");
    assert_ne!(s1, s2);
    assert_ne!(compute_hash(&s1), compute_hash(&s2));
}

#[test]
fn case_insensitive_in_hashset() {
    let mut set: HashSet<CaseInsensitiveString> = HashSet::new();
    set.insert(CaseInsensitiveString::new("Hello"));
    set.insert(CaseInsensitiveString::new("HELLO")); // Duplicate
    set.insert(CaseInsensitiveString::new("World"));

    assert_eq!(set.len(), 2);
}

#[test]
fn case_insensitive_in_hashmap() {
    let mut counts: HashMap<CaseInsensitiveString, u32> = HashMap::new();
    *counts.entry(CaseInsensitiveString::new("Apple")).or_insert(0) += 1;
    *counts.entry(CaseInsensitiveString::new("APPLE")).or_insert(0) += 1;
    *counts.entry(CaseInsensitiveString::new("apple")).or_insert(0) += 1;

    assert_eq!(counts.len(), 1);
    assert_eq!(
        counts.get(&CaseInsensitiveString::new("ApPlE")),
        Some(&3)
    );
}

#[test]
fn case_insensitive_preserves_original() {
    let s = CaseInsensitiveString::new("HeLLo WoRLd");
    assert_eq!(s.value(), "HeLLo WoRLd");
}

#[test]
fn case_insensitive_empty_string() {
    let s1 = CaseInsensitiveString::new("");
    let s2 = CaseInsensitiveString::new("");
    assert_eq!(s1, s2);
    assert_eq!(compute_hash(&s1), compute_hash(&s2));
}

#[test]
fn case_insensitive_special_characters() {
    let s1 = CaseInsensitiveString::new("Hello, World!");
    let s2 = CaseInsensitiveString::new("HELLO, WORLD!");
    assert_eq!(s1, s2);
    assert_eq!(compute_hash(&s1), compute_hash(&s2));
}

#[test]
fn case_insensitive_unicode() {
    // Basic ASCII letters are case-insensitive
    let s1 = CaseInsensitiveString::new("Cafe");
    let s2 = CaseInsensitiveString::new("CAFE");
    assert_eq!(s1, s2);
}

// ============================================================================
// Document tests
// ============================================================================

#[test]
fn document_same_id_equal() {
    let doc1 = Document::new(1, "Draft", "Content v1");
    let doc2 = Document::new(1, "Final", "Content v2");
    assert_eq!(doc1, doc2);
    assert_eq!(compute_hash(&doc1), compute_hash(&doc2));
}

#[test]
fn document_different_id_not_equal() {
    let doc1 = Document::new(1, "Same Title", "Same Content");
    let doc2 = Document::new(2, "Same Title", "Same Content");
    assert_ne!(doc1, doc2);
    assert_ne!(compute_hash(&doc1), compute_hash(&doc2));
}

#[test]
fn document_in_hashset() {
    let mut set: HashSet<Document> = HashSet::new();
    set.insert(Document::new(1, "Title 1", "Content 1"));
    set.insert(Document::new(1, "Title 2", "Content 2")); // Same id, duplicate
    set.insert(Document::new(2, "Title 3", "Content 3"));

    assert_eq!(set.len(), 2);
}

#[test]
fn document_in_hashmap() {
    let mut versions: HashMap<Document, u32> = HashMap::new();
    let doc = Document::new(1, "Doc", "Content");
    versions.insert(doc.clone(), 1);
    versions.insert(Document::new(1, "Updated Doc", "New Content"), 2);

    assert_eq!(versions.len(), 1); // Same id overwrites
}

#[test]
fn document_empty_strings() {
    let doc1 = Document::new(1, "", "");
    let doc2 = Document::new(1, "Has Title", "Has Content");
    assert_eq!(doc1, doc2); // Same id
    assert_eq!(compute_hash(&doc1), compute_hash(&doc2));
}

#[test]
fn document_zero_id() {
    let doc = Document::new(0, "Title", "Content");
    let mut set: HashSet<Document> = HashSet::new();
    set.insert(doc);
    assert!(set.contains(&Document::new(0, "Different", "Different")));
}

// ============================================================================
// Rgb tests
// ============================================================================

#[test]
fn rgb_equal_colors_same_hash() {
    let c1 = Rgb::new(255, 128, 0);
    let c2 = Rgb::new(255, 128, 0);
    assert_eq!(c1, c2);
    assert_eq!(compute_hash(&c1), compute_hash(&c2));
}

#[test]
fn rgb_different_colors_different_hash() {
    let c1 = Rgb::new(255, 0, 0);
    let c2 = Rgb::new(0, 255, 0);
    assert_ne!(c1, c2);
    assert_ne!(compute_hash(&c1), compute_hash(&c2));
}

#[test]
fn rgb_in_hashset() {
    let mut set: HashSet<Rgb> = HashSet::new();
    set.insert(Rgb::new(255, 0, 0));   // Red
    set.insert(Rgb::new(0, 255, 0));   // Green
    set.insert(Rgb::new(0, 0, 255));   // Blue
    set.insert(Rgb::new(255, 0, 0));   // Duplicate red

    assert_eq!(set.len(), 3);
}

#[test]
fn rgb_black_and_white() {
    let black = Rgb::new(0, 0, 0);
    let white = Rgb::new(255, 255, 255);
    let mut set: HashSet<Rgb> = HashSet::new();
    set.insert(black);
    set.insert(white);

    assert!(set.contains(&Rgb::new(0, 0, 0)));
    assert!(set.contains(&Rgb::new(255, 255, 255)));
}

#[test]
fn rgb_in_hashmap() {
    let mut color_names: HashMap<Rgb, &str> = HashMap::new();
    color_names.insert(Rgb::new(255, 0, 0), "red");
    color_names.insert(Rgb::new(0, 255, 0), "green");
    color_names.insert(Rgb::new(0, 0, 255), "blue");

    assert_eq!(color_names.get(&Rgb::new(255, 0, 0)), Some(&"red"));
    assert_eq!(color_names.get(&Rgb::new(128, 128, 128)), None);
}

// ============================================================================
// count_unique tests
// ============================================================================

#[test]
fn count_unique_basic() {
    assert_eq!(count_unique(&[1, 2, 2, 3, 3, 3]), 3);
}

#[test]
fn count_unique_empty() {
    assert_eq!(count_unique::<i32>(&[]), 0);
}

#[test]
fn count_unique_single() {
    assert_eq!(count_unique(&[42]), 1);
}

#[test]
fn count_unique_all_same() {
    assert_eq!(count_unique(&[1, 1, 1, 1, 1]), 1);
}

#[test]
fn count_unique_all_different() {
    assert_eq!(count_unique(&[1, 2, 3, 4, 5]), 5);
}

#[test]
fn count_unique_strings() {
    assert_eq!(count_unique(&["a", "b", "a", "c", "b"]), 3);
}

#[test]
fn count_unique_points() {
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 1 },
        Point { x: 0, y: 0 },
    ];
    assert_eq!(count_unique(&points), 2);
}

#[test]
fn count_unique_case_insensitive() {
    let strings = vec![
        CaseInsensitiveString::new("Hello"),
        CaseInsensitiveString::new("HELLO"),
        CaseInsensitiveString::new("World"),
    ];
    assert_eq!(count_unique(&strings), 2);
}

// ============================================================================
// find_duplicates tests
// ============================================================================

#[test]
fn find_duplicates_basic() {
    let dups = find_duplicates(&[1, 2, 2, 3, 3, 3]);
    assert_eq!(dups.len(), 2);
    assert!(dups.contains(&2));
    assert!(dups.contains(&3));
}

#[test]
fn find_duplicates_empty() {
    let dups = find_duplicates::<i32>(&[]);
    assert!(dups.is_empty());
}

#[test]
fn find_duplicates_no_duplicates() {
    let dups = find_duplicates(&[1, 2, 3, 4, 5]);
    assert!(dups.is_empty());
}

#[test]
fn find_duplicates_all_same() {
    let dups = find_duplicates(&[7, 7, 7, 7]);
    assert_eq!(dups.len(), 1);
    assert!(dups.contains(&7));
}

#[test]
fn find_duplicates_single() {
    let dups = find_duplicates(&[42]);
    assert!(dups.is_empty());
}

#[test]
fn find_duplicates_strings() {
    let dups = find_duplicates(&["a", "b", "a", "c", "b"]);
    assert_eq!(dups.len(), 2);
    assert!(dups.contains(&"a"));
    assert!(dups.contains(&"b"));
}

#[test]
fn find_duplicates_points() {
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 1 },
        Point { x: 0, y: 0 },
        Point { x: 2, y: 2 },
    ];
    let dups = find_duplicates(&points);
    assert_eq!(dups.len(), 1);
    assert!(dups.contains(&Point { x: 0, y: 0 }));
}

#[test]
fn find_duplicates_userids() {
    let ids = vec![UserId(1), UserId(2), UserId(1), UserId(3)];
    let dups = find_duplicates(&ids);
    assert_eq!(dups.len(), 1);
    assert!(dups.contains(&UserId(1)));
}

// ============================================================================
// group_by_hash tests
// ============================================================================

#[test]
fn group_by_hash_basic() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let by_parity = group_by_hash(&numbers, |n| n % 2);

    assert_eq!(by_parity.get(&0).unwrap(), &vec![2, 4, 6]);
    assert_eq!(by_parity.get(&1).unwrap(), &vec![1, 3, 5]);
}

#[test]
fn group_by_hash_empty() {
    let numbers: Vec<i32> = vec![];
    let grouped = group_by_hash(&numbers, |n| n % 2);
    assert!(grouped.is_empty());
}

#[test]
fn group_by_hash_single_group() {
    let numbers = vec![2, 4, 6, 8];
    let by_parity = group_by_hash(&numbers, |n| n % 2);

    assert_eq!(by_parity.len(), 1);
    assert_eq!(by_parity.get(&0).unwrap(), &vec![2, 4, 6, 8]);
}

#[test]
fn group_by_hash_strings_by_first_char() {
    let words = vec!["apple", "banana", "apricot", "blueberry", "cherry"];
    let grouped = group_by_hash(&words, |w| w.chars().next().unwrap());

    assert_eq!(grouped.get(&'a').unwrap().len(), 2);
    assert_eq!(grouped.get(&'b').unwrap().len(), 2);
    assert_eq!(grouped.get(&'c').unwrap().len(), 1);
}

#[test]
fn group_by_hash_strings_by_length() {
    let words = vec!["a", "bb", "ccc", "dd", "e"];
    let by_length = group_by_hash(&words, |w| w.len());

    assert_eq!(by_length.get(&1).unwrap(), &vec!["a", "e"]);
    assert_eq!(by_length.get(&2).unwrap(), &vec!["bb", "dd"]);
    assert_eq!(by_length.get(&3).unwrap(), &vec!["ccc"]);
}

#[test]
fn group_by_hash_points_by_quadrant() {
    let points = vec![
        Point { x: 1, y: 1 },
        Point { x: -1, y: 1 },
        Point { x: 2, y: 2 },
        Point { x: -2, y: -2 },
    ];
    let by_quadrant = group_by_hash(&points, |p| {
        match (p.x >= 0, p.y >= 0) {
            (true, true) => 1,
            (false, true) => 2,
            (false, false) => 3,
            (true, false) => 4,
        }
    });

    assert_eq!(by_quadrant.get(&1).unwrap().len(), 2); // Q1
    assert_eq!(by_quadrant.get(&2).unwrap().len(), 1); // Q2
    assert_eq!(by_quadrant.get(&3).unwrap().len(), 1); // Q3
}

#[test]
fn group_by_hash_rgb_by_channel() {
    let colors = vec![
        Rgb::new(255, 0, 0),
        Rgb::new(200, 0, 0),
        Rgb::new(0, 255, 0),
        Rgb::new(0, 0, 255),
    ];
    // Group by dominant channel
    let by_channel = group_by_hash(&colors, |c| {
        if c.r >= c.g && c.r >= c.b {
            'r'
        } else if c.g >= c.b {
            'g'
        } else {
            'b'
        }
    });

    assert_eq!(by_channel.get(&'r').unwrap().len(), 2);
    assert_eq!(by_channel.get(&'g').unwrap().len(), 1);
    assert_eq!(by_channel.get(&'b').unwrap().len(), 1);
}

// ============================================================================
// Integration tests
// ============================================================================

#[test]
fn integration_hashset_of_documents() {
    let mut documents: HashSet<Document> = HashSet::new();

    // Add documents with various versions
    documents.insert(Document::new(1, "Spec v1", "Initial spec"));
    documents.insert(Document::new(2, "Design v1", "Initial design"));
    documents.insert(Document::new(1, "Spec v2", "Updated spec")); // Same id as first

    // Only 2 unique documents (by id)
    assert_eq!(documents.len(), 2);
    assert!(documents.contains(&Document::new(1, "Any", "Any")));
    assert!(documents.contains(&Document::new(2, "Any", "Any")));
}

#[test]
fn integration_case_insensitive_word_count() {
    let words = vec!["The", "the", "THE", "quick", "Quick", "fox"];
    let mut counts: HashMap<CaseInsensitiveString, u32> = HashMap::new();

    for word in words {
        *counts.entry(CaseInsensitiveString::new(word)).or_insert(0) += 1;
    }

    assert_eq!(counts.get(&CaseInsensitiveString::new("THE")), Some(&3));
    assert_eq!(counts.get(&CaseInsensitiveString::new("QUICK")), Some(&2));
    assert_eq!(counts.get(&CaseInsensitiveString::new("FOX")), Some(&1));
}

#[test]
fn integration_unique_and_duplicates() {
    let items = vec![1, 2, 2, 3, 3, 3, 4];

    let unique_count = count_unique(&items);
    let duplicates = find_duplicates(&items);

    assert_eq!(unique_count, 4);
    assert_eq!(duplicates.len(), 2);
    assert!(duplicates.contains(&2));
    assert!(duplicates.contains(&3));
}

#[test]
fn integration_group_and_count() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let grouped = group_by_hash(&numbers, |n| n % 3);

    // Count unique values in each group
    assert_eq!(count_unique(grouped.get(&0).unwrap()), 3); // 3, 6, 9
    assert_eq!(count_unique(grouped.get(&1).unwrap()), 4); // 1, 4, 7, 10
    assert_eq!(count_unique(grouped.get(&2).unwrap()), 3); // 2, 5, 8
}

#[test]
fn integration_color_palette() {
    let mut palette: HashSet<Rgb> = HashSet::new();

    // Add primary colors
    palette.insert(Rgb::new(255, 0, 0));
    palette.insert(Rgb::new(0, 255, 0));
    palette.insert(Rgb::new(0, 0, 255));

    // Try to add duplicates
    palette.insert(Rgb::new(255, 0, 0));
    palette.insert(Rgb::new(0, 255, 0));

    assert_eq!(palette.len(), 3);

    // Find duplicates in a list
    let color_list = vec![
        Rgb::new(255, 0, 0),
        Rgb::new(0, 255, 0),
        Rgb::new(255, 0, 0),
    ];
    let dups = find_duplicates(&color_list);
    assert_eq!(dups.len(), 1);
    assert!(dups.contains(&Rgb::new(255, 0, 0)));
}

#[test]
fn integration_userid_lookup() {
    let mut user_data: HashMap<UserId, (&str, u32)> = HashMap::new();
    user_data.insert(UserId(1), ("Alice", 25));
    user_data.insert(UserId(2), ("Bob", 30));
    user_data.insert(UserId(3), ("Charlie", 35));

    // Lookup by id
    assert_eq!(user_data.get(&UserId(1)), Some(&("Alice", 25)));
    assert_eq!(user_data.get(&UserId(4)), None);

    // Count unique users
    let user_ids = vec![UserId(1), UserId(2), UserId(1), UserId(3), UserId(2)];
    assert_eq!(count_unique(&user_ids), 3);

    // Find duplicate user references
    let dups = find_duplicates(&user_ids);
    assert_eq!(dups.len(), 2);
}

#[test]
fn integration_hash_consistency() {
    // Verify that hash is consistent across multiple calls
    let point = Point { x: 42, y: 24 };
    let hash1 = compute_hash(&point);
    let hash2 = compute_hash(&point);
    let hash3 = compute_hash(&point);

    assert_eq!(hash1, hash2);
    assert_eq!(hash2, hash3);
}

#[test]
fn integration_equal_implies_same_hash() {
    // Test the critical invariant: a == b implies hash(a) == hash(b)

    // Point
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5, y: 10 };
    assert_eq!(p1, p2);
    assert_eq!(compute_hash(&p1), compute_hash(&p2));

    // CaseInsensitiveString
    let s1 = CaseInsensitiveString::new("TeSt");
    let s2 = CaseInsensitiveString::new("TEST");
    assert_eq!(s1, s2);
    assert_eq!(compute_hash(&s1), compute_hash(&s2));

    // Document
    let d1 = Document::new(100, "Title A", "Content A");
    let d2 = Document::new(100, "Title B", "Content B");
    assert_eq!(d1, d2);
    assert_eq!(compute_hash(&d1), compute_hash(&d2));

    // Rgb
    let c1 = Rgb::new(128, 128, 128);
    let c2 = Rgb::new(128, 128, 128);
    assert_eq!(c1, c2);
    assert_eq!(compute_hash(&c1), compute_hash(&c2));
}

#[test]
fn integration_complex_grouping() {
    #[derive(Clone)]
    struct Item {
        category: String,
        value: i32,
    }

    let items = vec![
        Item { category: "A".to_string(), value: 1 },
        Item { category: "B".to_string(), value: 2 },
        Item { category: "A".to_string(), value: 3 },
        Item { category: "C".to_string(), value: 4 },
        Item { category: "B".to_string(), value: 5 },
    ];

    let grouped = group_by_hash(&items, |item| item.category.clone());

    assert_eq!(grouped.len(), 3);
    assert_eq!(grouped.get("A").unwrap().len(), 2);
    assert_eq!(grouped.get("B").unwrap().len(), 2);
    assert_eq!(grouped.get("C").unwrap().len(), 1);
}
