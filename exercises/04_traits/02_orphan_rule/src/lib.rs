// TODO: this is an example of an orphan rule violation.
// TODO: 这是一个孤儿规则违规的示例。
//  We're implementing a foreign trait (`PartialEq`, from `std`) on
//  我们正在为外部类型（`u32`，来自 `std`）实现外部 trait（`PartialEq`，来自 `std`）。
//  a foreign type (`u32`, from `std`).
//  我们正在为外部类型（`u32`，来自 `std`）实现外部 trait（`PartialEq`，来自 `std`）。
//  Look at the compiler error to get familiar with what it looks like.
//  查看编译器错误以熟悉它的样子。
//  Then delete the code below and move on to the next exercise.
//  然后删除下面的代码，继续下一个练习。

impl PartialEq for u32 {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
