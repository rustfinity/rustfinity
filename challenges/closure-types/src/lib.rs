pub fn create_typed_closures() -> (
    impl Fn(f64, f64) -> f64,
    impl FnMut(&mut f64, f64),
     impl Fn(String) -> String,
) {
    let calculate_total = |price: f64, tax_rate: f64| price + price * tax_rate;

    let apply_discount = |total: &mut f64, discount: f64| {
        *total -= discount;
    };

    let checkout_cart = |cart_details: String| format!("Checkout complete: {}", cart_details);

    (calculate_total, apply_discount, checkout_cart)
}

// Example usage
pub fn main() {
    let (calculate_total, mut apply_discount, checkout_cart) = create_typed_closures();

    // Example tests
    assert_eq!(calculate_total(100.0, 0.2), 120.0);

    let mut total_price = 120.0;
    apply_discount(&mut total_price, 20.0);
    assert_eq!(total_price, 100.0);

    let cart_details = String::from("Items: Apple, Banana, Orange");
    assert_eq!(
        checkout_cart(cart_details),
        "Checkout complete: Items: Apple, Banana, Orange"
    );
}
