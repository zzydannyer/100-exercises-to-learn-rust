// TODO: Given a vector of integers, leak its heap allocation.
// TODO: 给定一个整数向量，泄漏其堆分配。
//  Then split the resulting static slice into two halves and
//  然后将生成的静态切片分成两半，并在
//  sum each half in a separate thread.
//  独立的线程中对每一半求和。
//  Hint: check out `Vec::leak`.
//  提示：查看 `Vec::leak`。

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let v = v.leak();
    let mid = v.len() / 2;
    let (v1, v2) = v.split_at(mid);
    let handle1 = thread::spawn(|| v1.into_iter().sum::<i32>());
    let handle2 = thread::spawn(|| v2.into_iter().sum::<i32>());
    handle1.join().unwrap() + handle2.join().unwrap()
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
