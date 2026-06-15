// TODO: fix the function signature below to make the tests pass.
// TODO: 修复下面的函数签名以使测试通过。
//  Make sure to read the compiler error message—the Rust compiler is your pair programming
//  一定要阅读编译器错误信息——Rust 编译器是你在本课程中的结对编程伙伴，它会经常为你指明正确的方向！
//  partner in this course and it'll often guide you in the right direction!
//  一定要阅读编译器错误信息——Rust 编译器是你在本课程中的结对编程伙伴，它会经常为你指明正确的方向！
//
// The input parameters should have the same type of the return type.
// 输入参数应该与返回类型具有相同的类型。
fn compute(a, b) -> u32 {
    // Don't touch the function body.
    // 不要修改函数体。
    a + b * 2
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 5);
    }
}
