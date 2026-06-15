# Panics
# 恐慌

Let's go back to the `speed` function you wrote for the ["Variables" section](02_variables.md).
让我们回到你在["变量"部分](02_variables.md)中编写的 `speed` 函数。
It probably looked something like this:
它可能看起来像这样：

```rust
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    let distance = end - start;
    distance / time_elapsed
}
```

If you have a keen eye, you might have spotted one issue[^one]: what happens if `time_elapsed` is zero?
如果你眼力敏锐，你可能已经发现了一个问题[^one]：如果 `time_elapsed` 为零会发生什么？

You can try it
out [on the Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=36e5ddbe3b3f741dfa9f74c956622bac)!\
你可以在 [Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=36e5ddbe3b3f741dfa9f74c956622bac) 上试试看！\
The program will exit with the following error message:
程序将退出并显示以下错误消息：

```text
thread 'main' panicked at src/main.rs:3:5:
attempt to divide by zero
```

This is known as a **panic**.\
这被称为**恐慌(panic)**。\
A panic is Rust's way to signal that something went so wrong that
the program can't continue executing, it's an **unrecoverable error**[^catching]. Division by zero classifies as such an
error.
恐慌是 Rust 用来表示某些问题严重到程序无法继续执行的方式，这是一种**不可恢复的错误**[^catching]。除以零被归类为这样的错误。

## The panic! macro
## panic! 宏

You can intentionally trigger a panic by calling the `panic!` macro[^macro]:
你可以通过调用 `panic!` 宏[^macro]有意触发恐慌：

```rust
fn main() {
    panic!("This is a panic!");
    // The line below will never be executed
    let x = 1 + 2;
}
```

There are other mechanisms to work with recoverable errors in Rust, which [we'll cover later](../05_ticket_v2/06_fallibility.md).
Rust 中还有处理可恢复错误的其他机制，我们[将在后面讲解](../05_ticket_v2/06_fallibility.md)。
For the time being we'll stick with panics as a brutal but simple stopgap solution.
目前，我们将坚持使用恐慌作为一种粗暴但简单的临时解决方案。

## Further reading
## 扩展阅读

- [The panic! macro documentation](https://doc.rust-lang.org/std/macro.panic.html)
- [panic! 宏文档](https://doc.rust-lang.org/std/macro.panic.html)

[^one]: There's another issue with `speed` that we'll address soon enough. Can you spot it?
[^one]: `speed` 还有另一个问题，我们很快会解决。你能发现它吗？

[^catching]: You can try to catch a panic, but it should be a last resort attempt reserved for very specific
circumstances.
[^catching]: 你可以尝试捕获恐慌，但这应该是保留给非常特定情况下的最后手段。

[^macro]: If it's followed by a `!`, it's a macro invocation. Think of macros as spicy functions for now. We'll
cover them in more detail later in the course.
[^macro]: 如果后跟 `!`，那就是宏调用。目前可以把宏看作加强版的函数。我们将在课程后面更详细地介绍它们。
