// Customize the `dev` profile to wrap around on overflow.
// 自定义 `dev` profile，使其在溢出时回绕。
// Check Cargo's documentation to find out the right syntax:
// 查看 Cargo 的文档以找到正确的语法：
// https://doc.rust-lang.org/cargo/reference/profiles.html
//
// For reasons that we'll explain later, the customization needs to be done in the `Cargo.toml`
// 出于我们稍后会解释的原因，自定义需要在仓库根目录的 `Cargo.toml` 中进行，而不是练习的 `Cargo.toml`。
// at the root of the repository, not in the `Cargo.toml` of the exercise.
// 出于我们稍后会解释的原因，自定义需要在仓库根目录的 `Cargo.toml` 中进行，而不是练习的 `Cargo.toml`。

pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        // 20! is 2432902008176640000, which is too large to fit in a u32
        // 20! 是 2432902008176640000，它太大了，无法放入 u32
        // With the default dev profile, this will panic when you run `cargo test`
        // 使用默认的 dev profile，当你运行 `cargo test` 时会触发 panic
        // We want it to wrap around instead
        // 我们希望它改为回绕
        assert_eq!(factorial(20), 2_192_834_560);
        //                           ☝️
        // A large number literal using underscores to improve readability!
        // 一个使用下划线来提高可读性的大数字字面量！
    }

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
