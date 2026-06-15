# Runtime architecture
# 运行时架构

So far we've been talking about async runtimes as an abstract concept.
到目前为止，我们一直将异步运行时作为一个抽象概念来讨论。
Let's dig a bit deeper into the way they are implemented—as you'll see soon enough,
it has an impact on our code.
让我们更深入地探讨它们的实现方式——正如你很快会看到的，它对我们的代码有影响。

## Flavors
## 风格

`tokio` ships two different runtime _flavors_.
`tokio` 提供了两种不同的运行时_风格_。

You can configure your runtime via `tokio::runtime::Builder`:
你可以通过 `tokio::runtime::Builder` 来配置你的运行时：

- `Builder::new_multi_thread` gives you a **multithreaded `tokio` runtime**
- `Builder::new_multi_thread` 给你一个**多线程 `tokio` 运行时**
- `Builder::new_current_thread` will instead rely on the **current thread** for execution.
- `Builder::new_current_thread` 则会依赖**当前线程**来执行。

`#[tokio::main]` returns a multithreaded runtime by default, while
`#[tokio::test]` uses a current thread runtime out of the box.
`#[tokio::main]` 默认返回一个多线程运行时，而 `#[tokio::test]` 直接使用当前线程运行时。

### Current thread runtime
### 当前线程运行时

The current-thread runtime, as the name implies, relies exclusively on the OS thread
it was launched on to schedule and execute tasks.\
当前线程运行时，顾名思义，完全依赖于启动它的操作系统线程来调度和执行任务。\
When using the current-thread runtime, you have **concurrency** but no **parallelism**:
asynchronous tasks will be interleaved, but there will always be at most one task running
at any given time.
使用当前线程运行时，你拥有**并发**但没有**并行**：异步任务会交错执行，但任何时候最多只有一个任务在运行。

### Multithreaded runtime
### 多线程运行时

When using the multithreaded runtime, instead, there can be up to `N` tasks running
_in parallel_ at any given time, where `N` is the number of threads used by the
runtime. By default, `N` matches the number of available CPU cores.
使用多线程运行时，任何时候最多可以有 `N` 个任务_并行_运行，其中 `N` 是运行时使用的线程数。默认情况下，`N` 与可用的 CPU 核心数匹配。

There's more: `tokio` performs **work-stealing**.\
还有更多：`tokio` 执行**工作窃取**。\
If a thread is idle, it won't wait around: it'll try to find a new task that's ready for
execution, either from a global queue or by stealing it from the local queue of another
thread.\
如果一个线程空闲，它不会闲着：它会尝试找到一个新的准备就绪的任务来执行，要么从全局队列中获取，要么从另一个线程的本地队列中窃取。\
Work-stealing can have significant performance benefits, especially on tail latencies,
whenever your application is dealing with workloads that are not perfectly balanced
across threads.
工作窃取可以带来显著的性能优势，特别是在尾部延迟方面，当你的应用程序处理的工作负载在线程之间不完全平衡时尤为如此。

## Implications
## 影响

`tokio::spawn` is flavor-agnostic: it'll work no matter if you're running on the multithreaded
or current-thread runtime. The downside is that the signature assumes the worst case
(i.e. multithreaded) and is constrained accordingly:
`tokio::spawn` 是风格无关的：无论你是在多线程运行时还是在当前线程运行时上运行，它都能工作。缺点是其签名假设了最坏情况（即多线程），并相应地进行约束：

```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{ /* */ }
```

Let's ignore the `Future` trait for now to focus on the rest.\
我们先忽略 `Future` trait，专注于其余部分。\
`spawn` is asking all its inputs to be `Send` and have a `'static` lifetime.
`spawn` 要求其所有输入都是 `Send` 的并且具有 `'static` 生命周期。

The `'static` constraint follows the same rationale of the `'static` constraint
on `std::thread::spawn`: the spawned task may outlive the context it was spawned
from, therefore it shouldn't depend on any local data that may be de-allocated
after the spawning context is destroyed.
`'static` 约束遵循与 `std::thread::spawn` 上 `'static` 约束相同的逻辑：生成的任务可能比生成它的上下文存活得更久，因此它不应依赖于任何在生成上下文销毁后可能被释放的本地数据。

```rust
fn spawner() {
    let v = vec![1, 2, 3];
    // This won't work, since `&v` doesn't
    // live long enough.
    tokio::spawn(async { 
        for x in &v {
            println!("{x}")
        }
    })
}
```

`Send`, on the other hand, is a direct consequence of `tokio`'s work-stealing strategy:
a task that was spawned on thread `A` may end up being moved to thread `B` if that's idle,
thus requiring a `Send` bound since we're crossing thread boundaries.
另一方面，`Send` 是 `tokio` 工作窃取策略的直接后果：在线程 `A` 上生成的任务最终可能会被移动到线程 `B`（如果线程 `B` 空闲），因此需要 `Send` 约束，因为我们正在跨越线程边界。

```rust
fn spawner(input: Rc<u64>) {
    // This won't work either, because
    // `Rc` isn't `Send`.
    tokio::spawn(async move {
        println!("{}", input);
    })
}
```
