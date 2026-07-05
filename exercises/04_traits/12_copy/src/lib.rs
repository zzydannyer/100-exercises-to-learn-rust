// TODO: implement the necessary traits to make the test compile and pass.
// TODO: 实现必要的 trait 以使测试编译通过。
//  You *can't* modify the test.
//  你*不能*修改测试。
use std::ops::Add;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl Add for WrappingU32 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let wrapped_val = self.value.wrapping_add(rhs.value);
        Self::new(wrapped_val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
