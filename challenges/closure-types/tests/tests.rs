use closure_types::*;

#[test]
fn test_calculate_total() {
    let (calculate_total, _, _) = create_typed_closures();
    assert_eq!(calculate_total(100.0, 0.2), 120.0);
    assert_eq!(calculate_total(50.0, 0.1), 55.0);
    assert_eq!(calculate_total(0.0, 0.3), 0.0);
    assert_eq!(calculate_total(200.0, 0.15), 230.0);
}

#[test]
fn test_apply_discount() {
    let (_, mut apply_discount, _) = create_typed_closures();
    let mut total_price = 120.0;

    // Apply a discount and check the result
    apply_discount(&mut total_price, 20.0);
    assert_eq!(total_price, 100.0);

    // Apply a larger discount
    apply_discount(&mut total_price, 30.0);
    assert_eq!(total_price, 70.0);

    // Apply a zero discount
    apply_discount(&mut total_price, 0.0);
    assert_eq!(total_price, 70.0);

    // Ensure that applying a discount larger than the total works correctly
    apply_discount(&mut total_price, 100.0);
    assert_eq!(total_price, -30.0);
}

#[test]
fn test_checkout_cart() {
    let (_, _, checkout_cart) = create_typed_closures();

    // Standard cart checkout
    let cart_details = String::from("Items: Apple, Banana, Orange");

    let checkout_result = checkout_cart(cart_details);
    for item in ["Apple", "Banana", "Orange"].iter() {
        assert!(
            checkout_result.contains(item),
            "Receipt should contain item: {}",
            item
        );
    }
}

#[test]
fn test_combined_behavior() {
    let (calculate_total, mut apply_discount, checkout_cart) = create_typed_closures();

    // Simulate a real-world use case
    let item_price = 150.0;
    let tax_rate = 0.2;
    let mut total_price = calculate_total(item_price, tax_rate);

    assert_eq!(total_price, 180.0); // Price after tax

    apply_discount(&mut total_price, 30.0); // Apply a discount
    assert_eq!(total_price, 150.0);

    let cart_details = String::from("Items: Laptop, Mouse");
    let receipt = checkout_cart(cart_details);

    for item in ["Laptop", "Mouse"].iter() {
        assert!(
            receipt.contains(item),
            "Receipt should contain item: {}",
            item
        );
    }
}


#[test]
fn test_checkout_cart_is_fn_not_fn_once() {
    let (_, _, checkout_cart) = create_typed_closures();

    // Fn closures can be called multiple times with different inputs
    let result1 = checkout_cart(String::from("Items: Apple"));
    let result2 = checkout_cart(String::from("Items: Banana"));

    assert!(result1.contains("Apple"));
    assert!(result2.contains("Banana"));
}