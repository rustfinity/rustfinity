// A color represented by RGB components.
// TODO: Add the appropriate derive macros to make this type Copy
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// Dimensions represented by width and height.
// TODO: Add the appropriate derive macros to make this type Copy
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
}

// A text label.
// TODO: Add the appropriate derive macros (Clone only, NOT Copy - String is not Copy)
pub struct Label {
    pub text: String,
}

// A document with a title and pages.
// TODO: Add the appropriate derive macros (Clone only, NOT Copy)
pub struct Document {
    pub title: String,
    pub pages: Vec<String>,
}

// A generic tagged value.
// TODO: Add the appropriate derive macros with trait bounds
pub struct TaggedValue<T> {
    pub tag: String,
    pub value: T,
}

/// Duplicates a `Copy` type by returning two copies.
///
/// Since `T` implements `Copy`, the value is copied rather than moved,
/// allowing us to return two independent copies.
pub fn duplicate_copy<T: Copy>(value: T) -> (T, T) {
    // TODO: Return a tuple with two copies of the value
    // Hint: Since T is Copy, you can use the value multiple times
    todo!()
}

/// Creates a clone of a referenced value.
///
/// Takes a reference to avoid moving the original value and returns
/// a cloned copy.
pub fn duplicate_clone<T: Clone>(value: &T) -> T {
    // TODO: Clone the value and return it
    todo!()
}

/// Clones all items in a slice into a new Vec.
pub fn clone_vec<T: Clone>(items: &[T]) -> Vec<T> {
    // TODO: Clone all items from the slice into a new Vec
    // Hint: You can use .to_vec() or .iter().cloned().collect()
    todo!()
}

// Example usage
pub fn main() {
    // Copy types - original still usable after assignment
    let color1 = Color { r: 255, g: 128, b: 0 };
    let color2 = color1; // Copy happens here
    println!("Color1: r={}", color1.r); // color1 still valid!
    println!("Color2: r={}", color2.r);

    let dims = Dimensions {
        width: 10.0,
        height: 20.0,
    };
    let (d1, d2) = duplicate_copy(dims);
    println!("Dimensions: {}x{} and {}x{}", d1.width, d1.height, d2.width, d2.height);

    // Clone types - must explicitly clone
    let label1 = Label {
        text: String::from("Hello"),
    };
    let label2 = label1.clone(); // Explicit clone needed
    println!("Label1: {}", label1.text);
    println!("Label2: {}", label2.text);

    let doc = Document {
        title: String::from("My Doc"),
        pages: vec![String::from("Page 1"), String::from("Page 2")],
    };
    let doc_copy = duplicate_clone(&doc);
    println!("Original: {}", doc.title);
    println!("Clone: {}", doc_copy.title);

    // Generic tagged value
    let tagged = TaggedValue {
        tag: String::from("number"),
        value: 42,
    };
    let tagged_clone = tagged.clone();
    println!("Tagged: {} = {}", tagged.tag, tagged.value);
    println!("Clone: {} = {}", tagged_clone.tag, tagged_clone.value);

    // Clone a vec
    let numbers = vec![1, 2, 3];
    let cloned = clone_vec(&numbers);
    println!("Original: {:?}", numbers);
    println!("Cloned: {:?}", cloned);
}
