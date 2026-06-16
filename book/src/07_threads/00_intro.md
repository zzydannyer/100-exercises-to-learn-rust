# Threads
# 线程

One of Rust's big promises is _fearless concurrency_: making it easier to write safe, concurrent programs.
Rust 的一大承诺是_无畏并发_：使编写安全、并发的程序更加容易。
We haven't seen much of that yet. All the work we've done so far has been single-threaded.
我们还没有看到很多这方面内容。到目前为止，我们完成的所有工作都是单线程的。
Time to change that!
是时候改变这一点了！

In this chapter we'll make our ticket store multithreaded.\
在本章中，我们将使我们的票据存储变为多线程。\
We'll have the opportunity to touch most of Rust's core concurrency features, including:
我们将有机会接触 Rust 大部分核心并发特性，包括：

- Threads, using the `std::thread` module
- 线程，使用 `std::thread` 模块
- Message passing, using channels
- 消息传递，使用通道
- Shared state, using `Arc`, `Mutex` and `RwLock`
- 共享状态，使用 `Arc`、`Mutex` 和 `RwLock`
- `Send` and `Sync`, the traits that encode Rust's concurrency guarantees
- `Send` 和 `Sync`，编码 Rust 并发保证的特征

We'll also discuss various design patterns for multithreaded systems and some of their trade-offs.
我们还将讨论多线程系统的各种设计模式及其一些权衡。
