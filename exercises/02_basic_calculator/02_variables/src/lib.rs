// 👇 The lines below, starting with `///`, are called **documentation comments**.
// 👇 下面以 `///` 开头的行被称为**文档注释**。
//    They attach documentation to the item that follows them. In this case, the `speed` function.
//    它们将文档附加到其后跟随的项上。在本例中，就是 `speed` 函数。
//    If you run `cargo doc --open` from this exercise's directory, Rust will generate
//    如果你从本练习的目录运行 `cargo doc --open`，Rust 将根据这些注释生成 HTML 文档并在浏览器中打开它。
//    HTML documentation from these comments and open it in your browser.
//    如果你从本练习的目录运行 `cargo doc --open`，Rust 将根据这些注释生成 HTML 文档并在浏览器中打开它。

/// Given the start and end points of a journey, and the time it took to complete it,
/// 给定一段行程的起点和终点，以及完成它所需的时间，
/// calculate the average speed.
/// 计算平均速度。
pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: define a variable named `distance` with the right value to get tests to pass
    // TODO: 定义一个名为 `distance` 的变量，使用正确的值以使测试通过。
    //  Do you need to annotate the type of `distance`? Why or why not?
    //  你需要为 `distance` 标注类型吗？为什么需要或不需要？
    let distance = end - start;
    // Don't change the line below
    // 不要修改下面这行
    distance / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(speed(10, 30, 10), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(speed(10, 31, 10), 2);
    }
}
