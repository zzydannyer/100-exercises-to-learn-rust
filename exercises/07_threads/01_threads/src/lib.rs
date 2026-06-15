// TODO: implement a multi-threaded version of the `sum` function
// TODO: 实现 `sum` 函数的多线程版本
//  using `spawn` and `join`.
//  使用 `spawn` 和 `join`。
//  Given a vector of integers, split the vector into two halves and
//  给定一个整数向量，将其分成两半，并在
//  sum each half in a separate thread.
//  独立的线程中对每一半求和。

// Caveat: We can't test *how* the function is implemented,
// 注意：我们无法测试函数的*实现方式*，
// we can only verify that it produces the correct result.
// 我们只能验证它是否产生正确的结果。
// You _could_ pass this test by just returning `v.iter().sum()`,
// 你*可以*通过仅返回 `v.iter().sum()` 来通过这个测试，
// but that would defeat the purpose of the exercise.
// 但这会违背练习的目的。
//
// Hint: you won't be able to get the spawned threads to _borrow_
// 提示：你无法让生成的线程直接_借用_
// slices of the vector directly. You'll need to allocate new
// 向量的切片。你需要为原始向量的每一半分配新的
// vectors for each half of the original vector. We'll see why
// 向量。我们将在下一个练习中了解
// this is necessary in the next exercise.
// 这样做的必要性。
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
