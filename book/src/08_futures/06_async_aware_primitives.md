# Async-aware primitives
# 异步感知的原语

If you browse `tokio`'s documentation, you'll notice that it provides a lot of types
that "mirror" the ones in the standard library, but with an asynchronous twist:
locks, channels, timers, and more.
如果你浏览 `tokio` 的文档，你会注意到它提供了许多"镜像"标准库中的类型，但带有异步的变体：锁、通道、定时器等等。

When working in an asynchronous context, you should prefer these asynchronous alternatives
to their synchronous counterparts.
在异步上下文中工作时，你应该优先使用这些异步替代品而不是其同步对应物。

To understand why, let's take a look at `Mutex`, the mutually exclusive lock we explored
in the previous chapter.
为了理解原因，让我们看看 `Mutex`，即我们在上一章中探讨的互斥锁。

## Case study: `Mutex`
## 案例研究：`Mutex`

Let's look at a simple example:
让我们看一个简单的例子：

```rust
use std::sync::{Arc, Mutex};

async fn run(m: Arc<Mutex<Vec<u64>>>) {
    let guard = m.lock().unwrap();
    http_call(&guard).await;
    println!("Sent {:?} to the server", &guard);
    // `guard` is dropped here
}

/// Use `v` as the body of an HTTP call.
async fn http_call(v: &[u64]) {
  // [...]
}
```

### `std::sync::MutexGuard` and yield points
### `std::sync::MutexGuard` 和收益点

This code will compile, but it's dangerous.
这段代码可以编译，但它是危险的。

We try to acquire a lock over a `Mutex` from `std` in an asynchronous context.
We then hold on to the resulting `MutexGuard` across a yield point (the `.await` on
`http_call`).
我们尝试在异步上下文中获取来自 `std` 的 `Mutex` 上的锁。然后我们在收益点（`http_call` 上的 `.await`）之间持有生成的 `MutexGuard`。

Let's imagine that there are two tasks executing `run`, concurrently, on a single-threaded
runtime. We observe the following sequence of scheduling events:
假设在单线程运行时上有两个任务并发执行 `run`。我们观察到以下调度事件序列：

```text
     Task A          Task B
        | 
  Acquire lock
Yields to runtime
        | 
        +--------------+
                       |
             Tries to acquire lock
```

We have a deadlock. Task B will never manage to acquire the lock, because the lock
is currently held by task A, which has yielded to the runtime before releasing the
lock and won't be scheduled again because the runtime cannot preempt task B.
我们遇到了死锁。任务 B 永远无法获取锁，因为锁当前由任务 A 持有，而任务 A 在释放锁之前已经收益给运行时，并且不会再被调度，因为运行时不能抢占任务 B。

### `tokio::sync::Mutex`
### `tokio::sync::Mutex`

You can solve the issue by switching to `tokio::sync::Mutex`:
你可以通过切换到 `tokio::sync::Mutex` 来解决这个问题：

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

async fn run(m: Arc<Mutex<Vec<u64>>>) {
    let guard = m.lock().await;
    http_call(&guard).await;
    println!("Sent {:?} to the server", &guard);
    // `guard` is dropped here
}
```

Acquiring the lock is now an asynchronous operation, which yields back to the runtime
if it can't make progress.\
获取锁现在是一个异步操作，如果无法取得进展，它会收益回运行时。\
Going back to the previous scenario, the following would happen:
回到之前的场景，会发生以下情况：

```text
       Task A          Task B
          | 
  Acquires the lock
  Starts `http_call`
  Yields to runtime
          | 
          +--------------+
                         |
             Tries to acquire the lock
              Cannot acquire the lock
                 Yields to runtime
                         |
          +--------------+
          |
`http_call` completes      
  Releases the lock
   Yield to runtime
          |
          +--------------+
                         |
                 Acquires the lock
                       [...]
```

All good!
一切正常！

### Multithreaded won't save you
### 多线程不会救你

We've used a single-threaded runtime as the execution context in our
previous example, but the same risk persists even when using a multithreaded
runtime.\
我们在之前的示例中使用了单线程运行时作为执行上下文，但即使使用多线程运行时，同样的风险仍然存在。\
The only difference is in the number of concurrent tasks required to create the deadlock:
in a single-threaded runtime, 2 are enough; in a multithreaded runtime, we
would need `N+1` tasks, where `N` is the number of runtime threads.
唯一的区别在于创建死锁所需的并发任务数量：在单线程运行时中，2 个就足够了；在多线程运行时中，我们需要 `N+1` 个任务，其中 `N` 是运行时线程数。

### Downsides
### 缺点

Having an async-aware `Mutex` comes with a performance penalty.\
拥有一个异步感知的 `Mutex` 会带来性能损失。\
If you're confident that the lock isn't under significant contention
_and_ you're careful to never hold it across a yield point, you can
still use `std::sync::Mutex` in an asynchronous context.
如果你确信锁没有严重的争用，_并且_你小心地不要在收益点之间持有它，你仍然可以在异步上下文中使用 `std::sync::Mutex`。

But weigh the performance benefit against the liveness risk you
will incur.
但请权衡性能收益与你将面临的活动性风险。

## Other primitives
## 其他原语

We used `Mutex` as an example, but the same applies to `RwLock`, semaphores, etc.\
我们以 `Mutex` 为例，但同样的原则适用于 `RwLock`、信号量等。\
Prefer async-aware versions when working in an asynchronous context to minimise
the risk of issues.
在异步上下文中工作时，优先使用异步感知版本以最小化问题风险。
