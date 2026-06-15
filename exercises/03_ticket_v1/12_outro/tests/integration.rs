use outro_02::Order;

// Files inside the `tests` directory are only compiled when you run tests.
// `tests` 目录中的文件仅在运行测试时才会被编译。
// As a consequence, we don't need the `#[cfg(test)]` attribute for conditional compilation—it's
// 因此，我们不需要 `#[cfg(test)]` 属性进行条件编译——这是隐式默认的。
// implied.
// 因此，我们不需要 `#[cfg(test)]` 属性进行条件编译——这是隐式默认的。

#[test]
fn test_order() {
    let mut order = Order::new("Rusty Book".to_string(), 3, 2999);

    assert_eq!(order.product_name(), "Rusty Book");
    assert_eq!(order.quantity(), &3);
    assert_eq!(order.unit_price(), &2999);
    assert_eq!(order.total(), 8997);

    order.set_product_name("Rust Book".to_string());
    order.set_quantity(2);
    order.set_unit_price(3999);

    assert_eq!(order.product_name(), "Rust Book");
    assert_eq!(order.quantity(), &2);
    assert_eq!(order.unit_price(), &3999);
    assert_eq!(order.total(), 7998);
}

// Validation tests
// 验证测试
#[test]
#[should_panic]
fn test_empty_product_name() {
    Order::new("".to_string(), 3, 2999);
}

#[test]
#[should_panic]
fn test_long_product_name() {
    Order::new("a".repeat(301), 3, 2999);
}

#[test]
#[should_panic]
fn test_zero_quantity() {
    Order::new("Rust Book".to_string(), 0, 2999);
}

#[test]
#[should_panic]
fn test_zero_unit_price() {
    Order::new("Rust Book".to_string(), 3, 0);
}
