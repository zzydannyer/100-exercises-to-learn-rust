// Define a function named `factorial` that, given a non-negative integer `n`,
// 定义一个名为 `factorial` 的函数，给定一个非负整数 `n`，
// returns `n!`, the factorial of `n`.
// 返回 `n!`，即 `n` 的阶乘。
//
// The factorial of `n` is defined as the product of all positive integers up to `n`.
// `n` 的阶乘定义为从 1 到 `n` 的所有正整数的乘积。
// For example, `5!` (read "five factorial") is `5 * 4 * 3 * 2 * 1`, which is `120`.
// 例如，`5!`（读作 "五的阶乘"）等于 `5 * 4 * 3 * 2 * 1`，即 `120`。
// `0!` is defined to be `1`.
// `0!` 被定义为 `1`。
//
// We expect `factorial(0)` to return `1`, `factorial(1)` to return `1`,
// 我们期望 `factorial(0)` 返回 `1`，`factorial(1)` 返回 `1`，
// `factorial(2)` to return `2`, and so on.
// `factorial(2)` 返回 `2`，依此类推。
//
// Use only what you learned! No loops yet, so you'll have to use recursion!
// 只使用你学过的知识！还没有循环，所以你需要使用递归！

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
