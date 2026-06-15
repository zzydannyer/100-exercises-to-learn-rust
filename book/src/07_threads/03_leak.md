# Leaking data
# 内存泄漏

The main concern around passing references to spawned threads is use-after-free bugs:
accessing data using a pointer to a memory region that's already been freed/de-allocated.\
将引用传递给生成的线程时，主要担心的是 use-after-free 错误：使用指向已被释放/回收的内存区域的指针访问数据。\
If you're working with heap-allocated data, you can avoid the issue by
telling Rust that you'll never reclaim that memory: you choose to **leak memory**,
intentionally.
如果你正在处理堆分配的数据，你可以通过告诉 Rust 你永远不会回收那块内存来避免这个问题：你选择有意地**泄漏内存**。

This can be done, for example, using the `Box::leak` method from Rust's standard library:
例如，可以使用 Rust 标准库中的 `Box::leak` 方法来实现：

```rust
// Allocate a `u32` on the heap, by wrapping it in a `Box`.
let x = Box::new(41u32);
// Tell Rust that you'll never free that heap allocation
// using `Box::leak`. You can thus get back a 'static reference.
let static_ref: &'static mut u32 = Box::leak(x);
```

## Data leakage is process-scoped
## 内存泄漏是进程范围的

Leaking data is dangerous: if you keep leaking memory, you'll eventually
run out and crash with an out-of-memory error.
泄漏数据是危险的：如果持续泄漏内存，最终会耗尽内存并因内存不足错误而崩溃。

```rust
// If you leave this running for a while, 
// it'll eventually use all the available memory.
fn oom_trigger() {
    loop {
        let v: Vec<usize> = Vec::with_capacity(1024);
        v.leak();
    }
}
```

At the same time, memory leaked via `leak` method is not truly forgotten.\
同时，通过 `leak` 方法泄漏的内存并非真正被遗忘。\
The operating system can map each memory region to the process responsible for it.
操作系统可以将每个内存区域映射到负责它的进程。
When the process exits, the operating system will reclaim that memory.
当进程退出时，操作系统会回收该内存。

Keeping this in mind, it can be OK to leak memory when:
记住这一点，在以下情况下泄漏内存是可以接受的：

- The amount of memory you need to leak is bounded/known upfront, or
- 你需要泄漏的内存量是有上限/预先知道的，或者
- Your process is short-lived and you're confident you won't exhaust
  all the available memory before it exits
- 你的进程是短命的，并且你有信心在它退出之前不会耗尽所有可用内存

"Let the OS deal with it" is a perfectly valid memory management strategy
if your usecase allows for it.
"让操作系统来处理它"如果你的使用场景允许的话，这完全是一个有效的内存管理策略。
