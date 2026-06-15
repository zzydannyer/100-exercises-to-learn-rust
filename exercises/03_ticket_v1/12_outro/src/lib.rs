// TODO: Define a new `Order` type.
// TODO: 定义一个新的 `Order` 类型。
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   它应该跟踪三部分信息：`product_name`、`quantity` 和 `unit_price`。
//   The product name can't be empty and it can't be longer than 300 bytes.
//   产品名称不能为空，且不能超过 300 字节。
//   The quantity must be strictly greater than zero.
//   数量必须严格大于零。
//   The unit price is in cents and must be strictly greater than zero.
//   单价以分为单位，且必须严格大于零。
//   Order must include a method named `total` that returns the total price of the order.
//   Order 必须包含一个名为 `total` 的方法，返回订单的总价。
//   Order must provide setters and getters for each field.
//   Order 必须为每个字段提供 setter 和 getter 方法。
//
// Tests are located in a different place this time—in the `tests` folder.
// 这次测试放在不同的位置——在 `tests` 文件夹中。
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// `tests` 文件夹是 `cargo` 的一个特殊位置。它是 cargo 寻找**集成测试**的地方。
// Integration here has a very specific meaning: they test **the public API** of your project.
// 这里的集成有一个非常具体的含义：它们测试你项目的**公有 API**。
// You'll need to pay attention to the visibility of your types and methods; integration
// 你需要注意你的类型和方法的可见性；集成测试无法访问私有或 `pub(crate)` 的项。
// tests can't access private or `pub(crate)` items.
// 你需要注意你的类型和方法的可见性；集成测试无法访问私有或 `pub(crate)` 的项。
