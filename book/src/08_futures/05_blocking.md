# Don't block the runtime
# 不要阻塞运行时

Let's circle back to yield points.\
让我们回到收益点。\
Unlike threads, **Rust tasks cannot be preempted**.
与线程不同，**Rust 任务不能被抢占**。

`tokio` cannot, on its own, decide to pause a task and run another one in its place.
`tokio` 不能自行决定暂停一个任务并运行另一个任务来代替它。
The control goes back to the executor **exclusively** when the task yields—i.e.
when `Future::poll` returns `Poll::Pending` or, in the case of `async fn`, when
you `.await` a future.
控制权**仅**在任务收益时回到执行器——即当 `Future::poll` 返回 `Poll::Pending` 时，或者对于 `async fn`，当你 `.await` 一个 future 时。

This exposes the runtime to a risk: if a task never yields, the runtime will never
be able to run another task. This is called **blocking the runtime**.
这使运行时面临一个风险：如果一个任务从不收益，运行时将永远无法运行另一个任务。这被称为**阻塞运行时**。

## What is blocking?
## 什么是阻塞？

How long is too long? How much time can a task spend without yielding before it
becomes a problem?
多久算太长？一个任务在不收益的情况下可以花费多少时间才会成为问题？

It depends on the runtime, the application, the number of in-flight tasks, and
many other factors. But, as a general rule of thumb, try to spend less than 100
microseconds between yield points.
这取决于运行时、应用程序、正在运行的任务数量以及许多其他因素。但是，根据一般的经验法则，尽量在收益点之间花费少于 100 微秒的时间。

## Consequences
## 后果

Blocking the runtime can lead to:
阻塞运行时可能导致：

- **Deadlocks**: if the task that's not yielding is waiting for another task to
  complete, and that task is waiting for the first one to yield, you have a deadlock.
  No progress can be made, unless the runtime is able to schedule the other task on
  a different thread.
- **死锁**：如果正在收益的任务正在等待另一个任务完成，而那个任务又在等待第一个任务收益，就发生了死锁。除非运行时能够在不同的线程上调度另一个任务，否则无法取得任何进展。
- **Starvation**: other tasks might not be able to run, or might run after a long
  delay, which can lead to poor performances (e.g. high tail latencies).
- **饥饿**：其他任务可能无法运行，或者可能在长时间延迟后才能运行，这可能导致性能不佳（例如高尾部延迟）。

## Blocking is not always obvious
## 阻塞并不总是显而易见的

Some types of operations should generally be avoided in async code, like:
某些类型的操作通常应避免在异步代码中使用，例如：

- Synchronous I/O. You can't predict how long it will take, and it's likely to be
  longer than 100 microseconds.
- 同步 I/O。你无法预测需要多长时间，而且很可能超过 100 微秒。
- Expensive CPU-bound computations.
- 昂贵的 CPU 密集型计算。

The latter category is not always obvious though. For example, sorting a vector with
a few elements is not a problem; that evaluation changes if the vector has billions
of entries.
不过后一类并不总是显而易见的。例如，对只有少量元素的 vector 进行排序不是问题；但如果 vector 有数十亿个条目，这个评估就会改变。

## How to avoid blocking
## 如何避免阻塞

OK, so how do you avoid blocking the runtime assuming you _must_ perform an operation
that qualifies or risks qualifying as blocking?\
好的，那么假设你_必须_执行一个符合或有可能符合阻塞条件的操作，你如何避免阻塞运行时？\
You need to move the work to a different thread. You don't want to use the so-called
runtime threads, the ones used by `tokio` to run tasks.
你需要将工作移动到不同的线程。你不希望使用所谓的运行时线程，即 `tokio` 用来运行任务的线程。

`tokio` provides a dedicated threadpool for this purpose, called the **blocking pool**.
`tokio` 为此提供了一个专用的线程池，称为**阻塞池**。
You can spawn a synchronous operation on the blocking pool using the
`tokio::task::spawn_blocking` function. `spawn_blocking` returns a future that resolves
to the result of the operation when it completes.
你可以使用 `tokio::task::spawn_blocking` 函数在阻塞池上生成一个同步操作。`spawn_blocking` 返回一个 future，在操作完成时解析为操作的结果。

```rust
use tokio::task;

fn expensive_computation() -> u64 {
    // [...]
}

async fn run() {
    let handle = task::spawn_blocking(expensive_computation);
    // Do other stuff in the meantime
    let result = handle.await.unwrap();
}
```

The blocking pool is long-lived. `spawn_blocking` should be faster
than creating a new thread directly via `std::thread::spawn`
because the cost of thread initialization is amortized over multiple calls.
阻塞池是长期存在的。`spawn_blocking` 应该比通过 `std::thread::spawn` 直接创建新线程更快，因为线程初始化的成本在多次调用中被分摊。

## Further reading
## 延伸阅读

- Check out [Alice Ryhl's blog post](https://ryhl.io/blog/async-what-is-blocking/)
  on the topic.
- 查看 [Alice Ryhl 的博客文章](https://ryhl.io/blog/async-what-is-blocking/) 关于这个主题。
