// TODO: Add the necessary trait bounds to `min` so that it compiles successfully.
// TODO: 为 `min` 添加必要的 trait 约束，使其成功编译。
//   Refer to the documentation of the `std::cmp` module for more information on the traits you might need.
//   请参考 `std::cmp` 模块的文档，了解你可能需要的 trait 的更多信息。
//
// Note: there are different trait bounds that'll make the compiler happy, but they come with
// 注意：有多种不同的 trait 约束都能让编译器满意，但它们具有不同的**语义**。
// different _semantics_. We'll cover those differences later in the course when we talk about ordered
// 注意：有多种不同的 trait 约束都能让编译器满意，但它们具有不同的**语义**。
// collections (e.g. BTreeMap).
// 我们将在课程的后面讨论有序集合（例如 BTreeMap）时介绍这些差异。

/// Return the minimum of two values.
/// 返回两个值中的最小值。
pub fn min<T: std::cmp::PartialOrd>(left: T, right: T) -> T {
    if left <= right { left } else { right }
}
