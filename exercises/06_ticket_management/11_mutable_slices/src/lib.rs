// TODO: Define a function named `squared` that raises all `i32`s within a slice to the power of 2.
// TODO: 定义一个名为 `squared` 的函数，将切片中所有 `i32` 值提升到 2 次幂。
//  The slice should be modified in place.
//  切片应在原地进行修改。

fn squared(slice: &mut [i32]) {
    for elem in slice {
        *elem = (*elem).pow(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut s = vec![];
        squared(&mut s);
        assert_eq!(s, vec![]);
    }

    #[test]
    fn one() {
        let mut s = [2];
        squared(&mut s);
        assert_eq!(s, [4]);
    }

    #[test]
    fn multiple() {
        let mut s = vec![2, 4];
        squared(&mut s);
        assert_eq!(s, vec![4, 16]);
    }
}
