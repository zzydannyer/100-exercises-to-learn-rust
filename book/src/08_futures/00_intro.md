# Async Rust
# 异步 Rust

Threads are not the only way to write concurrent programs in Rust.\
线程并不是在 Rust 中编写并发程序的唯一方式。\
In this chapter we'll explore another approach: **asynchronous programming**.
在本章中，我们将探索另一种方法：**异步编程**。

In particular, you'll get an introduction to:
具体来说，你将了解到：

- The `async`/`.await` keywords, to write asynchronous code effortlessly
- `async`/`.await` 关键字，用于轻松编写异步代码
- The `Future` trait, to represent computations that may not be complete yet
- `Future` trait，用于表示可能尚未完成的计算
- `tokio`, the most popular runtime for running asynchronous code
- `tokio`，最流行的用于运行异步代码的运行时
- The cooperative nature of Rust asynchronous model, and how this affects your code
- Rust 异步模型的协作性质，以及它如何影响你的代码
