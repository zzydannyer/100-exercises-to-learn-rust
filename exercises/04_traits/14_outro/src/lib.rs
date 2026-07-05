// TODO: Define a new `SaturatingU16` type.
// TODO: 定义一个新的 `SaturatingU16` 类型。
//   It should hold a `u16` value.
//   它应该持有一个 `u16` 值。
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   它应该提供从 `u16`、`u8`、`&u16` 和 `&u8` 的转换。
//   It should support addition with a right-hand side of type
//   它应该支持与右侧类型为 `SaturatingU16`、`u16`、`&u16` 和 `&SaturatingU16` 的加法。
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   它应该支持与右侧类型为 `SaturatingU16`、`u16`、`&u16` 和 `&SaturatingU16` 的加法。
//   maximum value for `u16`.
//   加法应该在 `u16` 的最大值处饱和。
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   它应该可以与其他 `SaturatingU16` 或 `u16` 进行比较。
//   It should be possible to print its debug representation.
//   它应该可以打印其调试表示形式。
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
// 测试位于 `tests` 文件夹中——要注意你的类型和方法的可见性。
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SaturatingU16 {
    value: u16,
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 {
            value: value as u16,
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16 { value: *value }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 {
            value: *value as u16,
        }
    }
}

impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let value = self.value.saturating_add(rhs.value);
        SaturatingU16 { value }
    }
}
