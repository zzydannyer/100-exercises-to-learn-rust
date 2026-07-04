// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.
// TODO: 为 `WrappingU32` 类型实现 `From` trait，以使 `example` 编译通过。

pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(val: u32) -> Self {
        Self { value: val }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
