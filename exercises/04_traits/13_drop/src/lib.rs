// TODO: implement a so-called "Drop bomb": a type that panics when dropped
// TODO: 实现一个所谓的"Drop bomb"：一个在 dropped 时触发 panic 的类型，
//  unless a certain operation has been performed on it.
//  除非对其执行了某个特定操作。
//  You can see the expected API in the tests below.
//  你可以在下面的测试中看到预期的 API。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
        // bomb 在 dropped 时应触发 panic
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // bomb 在 dropped 时不应触发 panic
        // since it has been defused
        // 因为它已经被拆除了
    }
}
