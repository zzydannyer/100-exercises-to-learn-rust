// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
// 给定一个数字 `n`，返回斐波那契数列中的第 `n+1` 个数字。
//
// The Fibonacci sequence is defined as follows:
// 斐波那契数列定义如下：
//
// - The first number of the sequence is 0.
// - 数列的第一个数字是 0。
// - The second number of the sequence is 1.
// - 数列的第二个数字是 1。
// - Every subsequent number is the sum of the two preceding numbers.
// - 后续每个数字都是前两个数字之和。
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
// 因此数列为：0, 1, 1, 2, 3, 5, 8, 13, 21，依此类推。
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// 我们期望 `fibonacci(0)` 返回 `0`，`fibonacci(1)` 返回 `1`，
// `fibonacci(2)` to return `1`, and so on.
// `fibonacci(2)` 返回 `1`，依此类推。
pub fn fibonacci(n: u32) -> u32 {
    // TODO: implement the `fibonacci` function
    // TODO: 实现 `fibonacci` 函数
    //
    // Hint: use a `Vec` to memoize the results you have already calculated
    // 提示：使用 `Vec` 来缓存你已经计算过的结果
    // so that you don't have to recalculate them several times.
    // 这样就不必多次重新计算它们了。
    let mut memo = vec![0, 1];
    let n_usize = n as usize;
    for i in 2..=n_usize {
        memo.push(memo[i - 1] + memo[i - 2]);
    }
    memo[n_usize]
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
