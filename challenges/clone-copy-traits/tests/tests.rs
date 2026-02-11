use clone_copy_traits::*;

// ============================================================================
// Color tests (Copy type)
// ============================================================================

#[test]
fn color_is_copy() {
    let color1 = Color {
        r: 255,
        g: 128,
        b: 64,
    };
    let color2 = color1; // Copy
    assert_eq!(color1.r, 255); // Original still valid
    assert_eq!(color2.r, 255);
}

#[test]
fn color_is_clone() {
    let color1 = Color { r: 10, g: 20, b: 30 };
    let color2 = color1.clone();
    assert_eq!(color1, color2);
}

#[test]
fn color_equality() {
    let c1 = Color { r: 0, g: 0, b: 0 };
    let c2 = Color { r: 0, g: 0, b: 0 };
    let c3 = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    assert_eq!(c1, c2);
    assert_ne!(c1, c3);
}

#[test]
fn color_debug() {
    let color = Color { r: 1, g: 2, b: 3 };
    let debug_str = format!("{:?}", color);
    assert!(debug_str.contains("Color"));
    assert!(debug_str.contains("1"));
    assert!(debug_str.contains("2"));
    assert!(debug_str.contains("3"));
}

#[test]
fn color_copy_in_function() {
    fn takes_color(c: Color) -> u8 {
        c.r
    }
    let color = Color {
        r: 100,
        g: 50,
        b: 25,
    };
    let r1 = takes_color(color);
    let r2 = takes_color(color); // Can call again because Color is Copy
    assert_eq!(r1, 100);
    assert_eq!(r2, 100);
}

// ============================================================================
// Dimensions tests (Copy type)
// ============================================================================

#[test]
fn dimensions_is_copy() {
    let dims1 = Dimensions {
        width: 100.0,
        height: 50.0,
    };
    let dims2 = dims1; // Copy
    assert_eq!(dims1.width, 100.0); // Original still valid
    assert_eq!(dims2.width, 100.0);
}

#[test]
fn dimensions_is_clone() {
    let dims1 = Dimensions {
        width: 10.5,
        height: 20.5,
    };
    let dims2 = dims1.clone();
    assert_eq!(dims1, dims2);
}

#[test]
fn dimensions_equality() {
    let d1 = Dimensions {
        width: 1.0,
        height: 2.0,
    };
    let d2 = Dimensions {
        width: 1.0,
        height: 2.0,
    };
    let d3 = Dimensions {
        width: 3.0,
        height: 4.0,
    };
    assert_eq!(d1, d2);
    assert_ne!(d1, d3);
}

#[test]
fn dimensions_debug() {
    let dims = Dimensions {
        width: 42.5,
        height: 99.9,
    };
    let debug_str = format!("{:?}", dims);
    assert!(debug_str.contains("Dimensions"));
    assert!(debug_str.contains("42.5"));
    assert!(debug_str.contains("99.9"));
}

#[test]
fn dimensions_copy_in_function() {
    fn area(d: Dimensions) -> f64 {
        d.width * d.height
    }
    let dims = Dimensions {
        width: 10.0,
        height: 5.0,
    };
    let a1 = area(dims);
    let a2 = area(dims); // Can call again because Dimensions is Copy
    assert_eq!(a1, 50.0);
    assert_eq!(a2, 50.0);
}

// ============================================================================
// Label tests (Clone-only type)
// ============================================================================

#[test]
fn label_is_clone() {
    let label1 = Label {
        text: String::from("Hello"),
    };
    let label2 = label1.clone();
    assert_eq!(label1.text, "Hello");
    assert_eq!(label2.text, "Hello");
}

#[test]
fn label_clone_is_independent() {
    let label1 = Label {
        text: String::from("Original"),
    };
    let mut label2 = label1.clone();
    label2.text.push_str(" Modified");
    assert_eq!(label1.text, "Original"); // Original unchanged
    assert_eq!(label2.text, "Original Modified");
}

#[test]
fn label_equality() {
    let l1 = Label {
        text: String::from("same"),
    };
    let l2 = Label {
        text: String::from("same"),
    };
    let l3 = Label {
        text: String::from("different"),
    };
    assert_eq!(l1, l2);
    assert_ne!(l1, l3);
}

#[test]
fn label_debug() {
    let label = Label {
        text: String::from("test"),
    };
    let debug_str = format!("{:?}", label);
    assert!(debug_str.contains("Label"));
    assert!(debug_str.contains("test"));
}

#[test]
fn label_empty_text() {
    let label = Label {
        text: String::new(),
    };
    let cloned = label.clone();
    assert_eq!(cloned.text, "");
}

// ============================================================================
// Document tests (Clone-only type)
// ============================================================================

#[test]
fn document_is_clone() {
    let doc = Document {
        title: String::from("My Book"),
        pages: vec![String::from("Page 1"), String::from("Page 2")],
    };
    let doc_clone = doc.clone();
    assert_eq!(doc.title, doc_clone.title);
    assert_eq!(doc.pages, doc_clone.pages);
}

#[test]
fn document_clone_is_independent() {
    let doc = Document {
        title: String::from("Original"),
        pages: vec![String::from("Chapter 1")],
    };
    let mut doc_clone = doc.clone();
    doc_clone.title = String::from("Modified");
    doc_clone.pages.push(String::from("Chapter 2"));

    assert_eq!(doc.title, "Original");
    assert_eq!(doc.pages.len(), 1);
    assert_eq!(doc_clone.title, "Modified");
    assert_eq!(doc_clone.pages.len(), 2);
}

#[test]
fn document_equality() {
    let d1 = Document {
        title: String::from("A"),
        pages: vec![String::from("1")],
    };
    let d2 = Document {
        title: String::from("A"),
        pages: vec![String::from("1")],
    };
    let d3 = Document {
        title: String::from("B"),
        pages: vec![String::from("1")],
    };
    assert_eq!(d1, d2);
    assert_ne!(d1, d3);
}

#[test]
fn document_debug() {
    let doc = Document {
        title: String::from("Test"),
        pages: vec![],
    };
    let debug_str = format!("{:?}", doc);
    assert!(debug_str.contains("Document"));
    assert!(debug_str.contains("Test"));
}

#[test]
fn document_empty() {
    let doc = Document {
        title: String::new(),
        pages: vec![],
    };
    let cloned = doc.clone();
    assert_eq!(cloned.title, "");
    assert!(cloned.pages.is_empty());
}

#[test]
fn document_many_pages() {
    let pages: Vec<String> = (0..100).map(|i| format!("Page {}", i)).collect();
    let doc = Document {
        title: String::from("Big Book"),
        pages,
    };
    let cloned = doc.clone();
    assert_eq!(cloned.pages.len(), 100);
    assert_eq!(cloned.pages[50], "Page 50");
}

// ============================================================================
// TaggedValue tests (Generic Clone)
// ============================================================================

#[test]
fn tagged_value_with_integer() {
    let tv = TaggedValue {
        tag: String::from("count"),
        value: 42,
    };
    let cloned = tv.clone();
    assert_eq!(cloned.tag, "count");
    assert_eq!(cloned.value, 42);
}

#[test]
fn tagged_value_with_string() {
    let tv = TaggedValue {
        tag: String::from("name"),
        value: String::from("Alice"),
    };
    let cloned = tv.clone();
    assert_eq!(cloned.tag, "name");
    assert_eq!(cloned.value, "Alice");
}

#[test]
fn tagged_value_with_vec() {
    let tv = TaggedValue {
        tag: String::from("numbers"),
        value: vec![1, 2, 3],
    };
    let cloned = tv.clone();
    assert_eq!(cloned.value, vec![1, 2, 3]);
}

#[test]
fn tagged_value_clone_is_independent() {
    let tv = TaggedValue {
        tag: String::from("data"),
        value: vec![1, 2, 3],
    };
    let mut cloned = tv.clone();
    cloned.value.push(4);
    assert_eq!(tv.value, vec![1, 2, 3]);
    assert_eq!(cloned.value, vec![1, 2, 3, 4]);
}

#[test]
fn tagged_value_equality() {
    let tv1 = TaggedValue {
        tag: String::from("x"),
        value: 10,
    };
    let tv2 = TaggedValue {
        tag: String::from("x"),
        value: 10,
    };
    let tv3 = TaggedValue {
        tag: String::from("y"),
        value: 10,
    };
    assert_eq!(tv1, tv2);
    assert_ne!(tv1, tv3);
}

#[test]
fn tagged_value_debug() {
    let tv = TaggedValue {
        tag: String::from("test"),
        value: 99,
    };
    let debug_str = format!("{:?}", tv);
    assert!(debug_str.contains("TaggedValue"));
    assert!(debug_str.contains("test"));
    assert!(debug_str.contains("99"));
}

#[test]
fn tagged_value_with_float() {
    let tv = TaggedValue {
        tag: String::from("pi"),
        value: 3.14159f64,
    };
    let cloned = tv.clone();
    assert!((cloned.value - 3.14159).abs() < 0.0001);
}

#[test]
fn tagged_value_with_bool() {
    let tv = TaggedValue {
        tag: String::from("flag"),
        value: true,
    };
    let cloned = tv.clone();
    assert!(cloned.value);
}

// ============================================================================
// duplicate_copy tests
// ============================================================================

#[test]
fn duplicate_copy_color() {
    let color = Color {
        r: 100,
        g: 150,
        b: 200,
    };
    let (c1, c2) = duplicate_copy(color);
    assert_eq!(c1, c2);
    assert_eq!(c1.r, 100);
    assert_eq!(color.r, 100); // Original still valid
}

#[test]
fn duplicate_copy_dimensions() {
    let dims = Dimensions {
        width: 5.5,
        height: 10.5,
    };
    let (d1, d2) = duplicate_copy(dims);
    assert_eq!(d1, d2);
    assert_eq!(d1.width, 5.5);
}

#[test]
fn duplicate_copy_integer() {
    let (a, b) = duplicate_copy(42i32);
    assert_eq!(a, 42);
    assert_eq!(b, 42);
}

#[test]
fn duplicate_copy_float() {
    let (a, b) = duplicate_copy(3.14f64);
    assert!((a - 3.14).abs() < 0.001);
    assert!((b - 3.14).abs() < 0.001);
}

#[test]
fn duplicate_copy_bool() {
    let (a, b) = duplicate_copy(true);
    assert!(a);
    assert!(b);
}

#[test]
fn duplicate_copy_char() {
    let (a, b) = duplicate_copy('Z');
    assert_eq!(a, 'Z');
    assert_eq!(b, 'Z');
}

#[test]
fn duplicate_copy_tuple() {
    let (a, b) = duplicate_copy((1, 2, 3));
    assert_eq!(a, (1, 2, 3));
    assert_eq!(b, (1, 2, 3));
}

// ============================================================================
// duplicate_clone tests
// ============================================================================

#[test]
fn duplicate_clone_label() {
    let label = Label {
        text: String::from("test"),
    };
    let cloned = duplicate_clone(&label);
    assert_eq!(cloned.text, "test");
    assert_eq!(label.text, "test"); // Original still valid
}

#[test]
fn duplicate_clone_document() {
    let doc = Document {
        title: String::from("Title"),
        pages: vec![String::from("Page")],
    };
    let cloned = duplicate_clone(&doc);
    assert_eq!(cloned.title, "Title");
    assert_eq!(cloned.pages, vec!["Page"]);
}

#[test]
fn duplicate_clone_string() {
    let s = String::from("hello");
    let cloned = duplicate_clone(&s);
    assert_eq!(cloned, "hello");
}

#[test]
fn duplicate_clone_vec() {
    let v = vec![1, 2, 3, 4, 5];
    let cloned = duplicate_clone(&v);
    assert_eq!(cloned, vec![1, 2, 3, 4, 5]);
}

#[test]
fn duplicate_clone_color() {
    // Copy types also implement Clone
    let color = Color { r: 1, g: 2, b: 3 };
    let cloned = duplicate_clone(&color);
    assert_eq!(cloned, color);
}

#[test]
fn duplicate_clone_tagged_value() {
    let tv = TaggedValue {
        tag: String::from("tag"),
        value: 100,
    };
    let cloned = duplicate_clone(&tv);
    assert_eq!(cloned.tag, "tag");
    assert_eq!(cloned.value, 100);
}

// ============================================================================
// clone_vec tests
// ============================================================================

#[test]
fn clone_vec_integers() {
    let v = vec![1, 2, 3, 4, 5];
    let cloned = clone_vec(&v);
    assert_eq!(cloned, vec![1, 2, 3, 4, 5]);
}

#[test]
fn clone_vec_strings() {
    let v = vec![String::from("a"), String::from("b"), String::from("c")];
    let cloned = clone_vec(&v);
    assert_eq!(cloned, v);
}

#[test]
fn clone_vec_empty() {
    let v: Vec<i32> = vec![];
    let cloned = clone_vec(&v);
    assert!(cloned.is_empty());
}

#[test]
fn clone_vec_single() {
    let v = vec![42];
    let cloned = clone_vec(&v);
    assert_eq!(cloned, vec![42]);
}

#[test]
fn clone_vec_labels() {
    let labels = vec![
        Label {
            text: String::from("one"),
        },
        Label {
            text: String::from("two"),
        },
    ];
    let cloned = clone_vec(&labels);
    assert_eq!(cloned.len(), 2);
    assert_eq!(cloned[0].text, "one");
    assert_eq!(cloned[1].text, "two");
}

#[test]
fn clone_vec_colors() {
    let colors = vec![
        Color { r: 0, g: 0, b: 0 },
        Color {
            r: 255,
            g: 255,
            b: 255,
        },
    ];
    let cloned = clone_vec(&colors);
    assert_eq!(cloned, colors);
}

#[test]
fn clone_vec_is_independent() {
    let v = vec![String::from("a"), String::from("b")];
    let mut cloned = clone_vec(&v);
    cloned.push(String::from("c"));
    assert_eq!(v.len(), 2);
    assert_eq!(cloned.len(), 3);
}

#[test]
fn clone_vec_from_slice() {
    let arr = [1, 2, 3, 4, 5];
    let cloned = clone_vec(&arr);
    assert_eq!(cloned, vec![1, 2, 3, 4, 5]);
}

#[test]
fn clone_vec_nested() {
    let v = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let cloned = clone_vec(&v);
    assert_eq!(cloned, vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
}

// ============================================================================
// Integration tests
// ============================================================================

#[test]
fn integration_copy_vs_clone_behavior() {
    // Copy type: can use after move
    let color = Color {
        r: 100,
        g: 100,
        b: 100,
    };
    let _moved = color;
    assert_eq!(color.r, 100); // Still works!

    // Clone type: must explicitly clone to keep using
    let label = Label {
        text: String::from("test"),
    };
    let cloned = label.clone();
    // Both are now valid
    assert_eq!(label.text, "test");
    assert_eq!(cloned.text, "test");
}

#[test]
fn integration_duplicate_functions() {
    let dims = Dimensions {
        width: 10.0,
        height: 20.0,
    };
    let (d1, d2) = duplicate_copy(dims);
    let d3 = duplicate_clone(&dims);
    assert_eq!(d1, d2);
    assert_eq!(d1, d3);
}

#[test]
fn integration_clone_vec_with_tagged_values() {
    let values = vec![
        TaggedValue {
            tag: String::from("a"),
            value: 1,
        },
        TaggedValue {
            tag: String::from("b"),
            value: 2,
        },
    ];
    let cloned = clone_vec(&values);
    assert_eq!(cloned.len(), 2);
    assert_eq!(cloned[0].value, 1);
    assert_eq!(cloned[1].value, 2);
}

#[test]
fn integration_complex_document_workflow() {
    // Create a document
    let mut doc = Document {
        title: String::from("Draft"),
        pages: vec![],
    };

    // Add some pages
    doc.pages.push(String::from("Introduction"));
    doc.pages.push(String::from("Chapter 1"));

    // Clone for review
    let review_copy = duplicate_clone(&doc);

    // Modify original
    doc.title = String::from("Final");
    doc.pages.push(String::from("Conclusion"));

    // Review copy unchanged
    assert_eq!(review_copy.title, "Draft");
    assert_eq!(review_copy.pages.len(), 2);

    // Original modified
    assert_eq!(doc.title, "Final");
    assert_eq!(doc.pages.len(), 3);
}

#[test]
fn integration_generic_tagged_value_various_types() {
    // With different types
    let int_val = TaggedValue {
        tag: String::from("int"),
        value: 42,
    };
    let str_val = TaggedValue {
        tag: String::from("str"),
        value: String::from("hello"),
    };
    let vec_val = TaggedValue {
        tag: String::from("vec"),
        value: vec![1, 2, 3],
    };

    let cloned_int = int_val.clone();
    let cloned_str = str_val.clone();
    let cloned_vec = vec_val.clone();

    assert_eq!(cloned_int.value, 42);
    assert_eq!(cloned_str.value, "hello");
    assert_eq!(cloned_vec.value, vec![1, 2, 3]);
}

#[test]
fn integration_copy_types_in_collection() {
    let colors = vec![
        Color { r: 255, g: 0, b: 0 },
        Color { r: 0, g: 255, b: 0 },
        Color { r: 0, g: 0, b: 255 },
    ];

    // Can iterate and copy
    let doubled: Vec<(Color, Color)> = colors.iter().map(|&c| duplicate_copy(c)).collect();

    assert_eq!(doubled.len(), 3);
    assert_eq!(doubled[0].0, doubled[0].1);
}

#[test]
fn integration_nested_clone() {
    let doc = Document {
        title: String::from("Nested"),
        pages: vec![String::from("Page 1")],
    };
    let tagged = TaggedValue {
        tag: String::from("document"),
        value: doc,
    };
    let cloned = tagged.clone();
    assert_eq!(cloned.value.title, "Nested");
    assert_eq!(cloned.value.pages[0], "Page 1");
}
