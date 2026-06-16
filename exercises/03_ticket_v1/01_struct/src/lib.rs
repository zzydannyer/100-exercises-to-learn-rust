// Define a struct named `Order` with the following fields:
// 定义一个名为 `Order` 的结构体，包含以下字段：
// - `price`, an unsigned integer
// - `price`，一个无符号整数
// - `quantity`, an unsigned integer
// - `quantity`，一个无符号整数
//
// It should also have a method named `is_available` that returns a `true` if the quantity is
// 它还应该有一个名为 `is_available` 的方法，如果数量大于 0 则返回 `true`，否则返回 `false`。
// greater than 0, otherwise `false`.
// 它还应该有一个名为 `is_available` 的方法，如果数量大于 0 则返回 `true`，否则返回 `false`。

struct Order {
    price: u32,
    quantity: u32,
}

impl Order {
    fn is_available(&self) -> bool {
        self.quantity > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}
