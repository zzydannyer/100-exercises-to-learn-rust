pub fn factorial(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 1..=n {
        // Use saturating multiplication to stop at the maximum value of u32
        // 使用饱和乘法，在达到 u32 最大值时停止，
        // rather than overflowing and wrapping around
        // 而不是溢出并回绕。
        // 饱和
        // 溢出后直接卡在边界，不循环。超最大就取最大值，低于最小就取最小值。
        // 例：u8=255 + 1 → 255
        result = result.saturating_mul(i);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        assert_eq!(factorial(20), u32::MAX);
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
