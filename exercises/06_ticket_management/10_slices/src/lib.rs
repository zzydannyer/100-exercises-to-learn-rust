// TODO: Define a function named `sum` that takes a reference to a slice of `u32` and returns the sum of all
// TODO: 定义一个名为 `sum` 的函数，该函数接受一个对 `u32` 切片的引用，并返回切片中所有元素的总和。
//  elements in the slice.
fn sum(slice: &[u32]) -> u32 {
    slice.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let v = vec![];
        assert_eq!(sum(&v), 0);
    }

    #[test]
    fn one_element() {
        let v = vec![1];
        assert_eq!(sum(&v), 1);
    }

    #[test]
    fn multiple_elements() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }

    #[test]
    fn array_slice() {
        let v = [1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }
}
