// Rewrite the factorial function using a `while` loop.
// 使用 `while` 循环重写阶乘函数。
pub fn factorial(n: u32) -> u32 {
    // The `todo!()` macro is a placeholder that the compiler
    // `todo!()` 宏是一个占位符，编译器将其解释为"我稍后会回来处理"，
    // interprets as "I'll get back to this later", thus
    // `todo!()` 宏是一个占位符，编译器将其解释为"我稍后会回来处理"，
    // suppressing type errors.
    // 从而抑制类型错误。
    // It panics at runtime.
    // 它在运行时触发 panic。
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
