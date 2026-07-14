// TODO: Given a vector of integers, split it in two halves
// TODO: 给定一个整数向量，将其分成两半，
//  and compute the sum of each half in a separate thread.
//  并在独立的线程中计算每一半的和。
//  Don't perform any heap allocation. Don't leak any memory.
//  不要进行任何堆分配。不要泄漏任何内存。

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    let (left, right) = v.split_at(mid);

    std::thread::scope(|s| {
        let left_sum = s.spawn(|| left.iter().sum::<i32>());
        let right_sum = s.spawn(|| right.iter().sum::<i32>());
        left_sum.join().unwrap() + right_sum.join().unwrap()
    })
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
