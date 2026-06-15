# Modelling A Ticket, pt. 2
# 建模票据，第二部分

The `Ticket` struct we worked on in the previous chapters is a good start,
but it still screams "I'm a beginner Rustacean!".
我们在前几章中处理的 `Ticket` 结构体是一个好的开始，但它仍然在喊"我是一个 Rust 初学者！"。

We'll use this chapter to refine our Rust domain modelling skills.
我们将用本章来完善我们的 Rust 领域建模技能。
We'll need to introduce a few more concepts along the way:
在此过程中，我们需要引入一些新的概念：

- `enum`s, one of Rust's most powerful features for data modeling
- `enum`，Rust 数据建模最强大的特性之一
- The `Option` type, to model nullable values
- `Option` 类型，用于建模可空值
- The `Result` type, to model recoverable errors
- `Result` 类型，用于建模可恢复错误
- The `Debug` and `Display` traits, for printing
- `Debug` 和 `Display` 特征，用于打印
- The `Error` trait, to mark error types
- `Error` 特征，用于标记错误类型
- The `TryFrom` and `TryInto` traits, for fallible conversions
- `TryFrom` 和 `TryInto` 特征，用于可能失败的转换
- Rust's package system, explaining what's a library, what's a binary, how to use third-party crates
- Rust 的包系统，解释什么是库、什么是二进制文件、如何使用第三方 crate
