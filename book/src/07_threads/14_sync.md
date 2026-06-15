# `Sync`
# `Sync`

Before we wrap up this chapter, let's talk about another key trait in Rust's standard library: `Sync`.
在我们结束本章之前，让我们谈谈 Rust 标准库中的另一个关键 trait：`Sync`。

`Sync` is an auto trait, just like `Send`.\
`Sync` 是一个自动 trait，就像 `Send` 一样。\
It is automatically implemented by all types that can be safely **shared** between threads.
它由所有可以安全地在线程间**共享**的类型自动实现。

In other words: `T` is Sync if `&T` is `Send`.
换句话说：`T` 是 Sync 的，如果 `&T` 是 `Send` 的。

## `T: Sync` doesn't imply `T: Send`
## `T: Sync` 并不意味着 `T: Send`

It's important to note that `T` can be `Sync` without being `Send`.\
值得注意的是，`T` 可以是 `Sync` 的而不一定是 `Send` 的。\
For example: `MutexGuard` is not `Send`, but it is `Sync`.
例如：`MutexGuard` 不是 `Send` 的，但它是 `Sync` 的。

It isn't `Send` because the lock must be released on the same thread that acquired it, therefore we don't
want `MutexGuard` to be dropped on a different thread.\
它不是 `Send` 的，因为锁必须在获取它的同一个线程上释放，因此我们不希望 `MutexGuard` 在不同的线程上被丢弃。\
But it is `Sync`, because giving a `&MutexGuard` to another thread has no impact on where the lock is released.
但它是 `Sync` 的，因为将 `&MutexGuard` 提供给另一个线程对锁在哪里释放没有影响。

## `T: Send` doesn't imply `T: Sync`
## `T: Send` 并不意味着 `T: Sync`

The opposite is also true: `T` can be `Send` without being `Sync`.\
反过来也成立：`T` 可以是 `Send` 的而不一定是 `Sync` 的。\
For example: `RefCell<T>` is `Send` (if `T` is `Send`), but it is not `Sync`.
例如：`RefCell<T>` 是 `Send` 的（如果 `T` 是 `Send` 的话），但它不是 `Sync` 的。

`RefCell<T>` performs runtime borrow checking, but the counters it uses to track borrows are not thread-safe.
Therefore, having multiple threads holding a `&RefCell` would lead to a data race, with potentially
multiple threads obtaining mutable references to the same data. Hence `RefCell` is not `Sync`.\
`RefCell<T>` 执行运行时借用检查，但它用于跟踪借用的计数器不是线程安全的。因此，让多个线程持有 `&RefCell` 会导致数据竞争，多个线程可能获得对同一数据的可变引用。因此 `RefCell` 不是 `Sync` 的。\
`Send` is fine, instead, because when we send a `RefCell` to another thread we're not
leaving behind any references to the data it contains, hence no risk of concurrent mutable access.
而 `Send` 则是可以的，因为当我们把 `RefCell` 发送到另一个线程时，我们不会留下任何对其包含数据的引用，因此没有并发可变访问的风险。
