/// Given the start and end points of a journey, and the time it took to complete the journey,
/// 给定一段行程的起点和终点，以及完成行程所需的时间，
/// calculate the average speed of the journey.
/// 计算行程的平均速度。
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: Panic with a custom message if `time_elapsed` is 0
    // TODO: 如果 `time_elapsed` 为 0，以自定义消息触发 panic。
    if time_elapsed == 0 {
        panic!("The journey took no time at all. That's impossible!");
    }

    (end - start) / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    // 👇 With the `#[should_panic]` annotation we can assert that we expect the code
    // 👇 使用 `#[should_panic]` 注解，我们可以断言期望代码会触发 panic。
    //    under test to panic. We can also check the panic message by using `expected`.
    //    我们还可以通过 `expected` 来检查 panic 消息。
    //    This is all part of Rust's built-in test framework!
    //    这些都是 Rust 内置测试框架的一部分！
    #[should_panic(expected = "The journey took no time at all. That's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
