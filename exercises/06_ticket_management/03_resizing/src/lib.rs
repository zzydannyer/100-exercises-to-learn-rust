#[cfg(test)]
mod tests {
    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(2);
        v.push(1);
        v.push(2); // max capacity reached
        v.push(2); // 已达到最大容量 满了就翻倍
        assert_eq!(v.capacity(), 4);

        v.push(3); // beyond capacity, needs to resize
        v.push(3); // 超出容量，需要调整大小

        // Can you guess what the new capacity will be?
        // 你能猜到新的容量是多少吗？
        // Beware that the standard library makes no guarantees about the
        // 请注意，标准库不保证用于调整向量大小的算法，
        // algorithm used to resize the vector, so this may change in the future.
        // 因此这在未来可能会发生变化。
        assert_eq!(v.capacity(), 8);
    }
}
