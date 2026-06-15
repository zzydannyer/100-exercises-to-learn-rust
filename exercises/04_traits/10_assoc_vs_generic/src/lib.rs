// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
// TODO: 定义一个新的 trait `Power`，它有一个方法 `power`，将 `self` 提升到 `n` 次方。
//  to the power of `n`.
// TODO: 定义一个新的 trait `Power`，它有一个方法 `power`，将 `self` 提升到 `n` 次方。
//  The trait definition and its implementations should be enough to get
//  trait 定义及其实现应该足以让测试编译通过。
//  the tests to compile and pass.
//  trait 定义及其实现应该足以让测试编译通过。
//
// Recommendation: you may be tempted to write a generic implementation to handle
// 建议：你可能倾向于编写一个泛型实现来一次性处理所有情况。
// all cases at once. However, this is fairly complicated and requires the use of
// 建议：你可能倾向于编写一个泛型实现来一次性处理所有情况。
// additional crates (i.e. `num-traits`).
// 然而，这相当复杂，并且需要使用额外的 crate（例如 `num-traits`）。
// Even then, it might be preferable to use a simple macro instead to avoid
// 即使如此，使用简单的宏可能更可取，以避免高度泛型实现的复杂性。
// the complexity of a highly generic implementation. Check out the
// 即使如此，使用简单的宏可能更可取，以避免高度泛型实现的复杂性。
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// 如果你有兴趣了解更多，请查看"Little book of Rust macros"（https://veykril.github.io/tlborm/）。
// interested in learning more about it.
// 如果你有兴趣了解更多，请查看"Little book of Rust macros"（https://veykril.github.io/tlborm/）。
// You don't have to though: it's perfectly okay to write three separate
// 不过你不必这样做：手动编写三个独立的实现也是完全可以的。
// implementations manually. Venture further only if you're curious.
// 不过你不必这样做：手动编写三个独立的实现也是完全可以的。

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
