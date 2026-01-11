// 1. Create the Renderable trait

// 2. Create the Circle and Rectangle structs

// 3. Implement the trait for Circle and Rectangle

// 4. Create the Canvas struct

// 5. Implement the Canvas struct

// Example usage
pub fn main() {
    let mut canvas = Canvas::new();
    canvas.add_shape(Box::new(Circle { radius: 5.0 }));
    canvas.add_shape(Box::new(Rectangle {
        width: 3.0,
        height: 4.0,
    }));
    let rendered_shapes = canvas.render_all();
    for shape in rendered_shapes {
        println!("{}", shape);
    }
}
