# Spawning tasks
# 生成任务

Your solution to the previous exercise should look something like this:
你前一个练习的解决方案应该大致如下：

```rust
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        let (mut reader, mut writer) = socket.split();
        tokio::io::copy(&mut reader, &mut writer).await?;
    }
}
```

This is not bad!\
这还不错！\
If a long time passes between two incoming connections, the `echo` function will be idle
(since `TcpListener::accept` is an asynchronous function), thus allowing the executor
to run other tasks in the meantime.
如果在两个传入连接之间经过很长时间，`echo` 函数将处于空闲状态（因为 `TcpListener::accept` 是一个异步函数），从而允许执行器在此期间运行其他任务。

But how can we actually have multiple tasks running concurrently?\
但是我们如何才能真正让多个任务并发运行呢？\
If we always run our asynchronous functions until completion (by using `.await`), we'll never
have more than one task running at a time.
如果我们总是运行异步函数直到完成（通过使用 `.await`），我们永远不会有超过一个任务同时运行。

This is where the `tokio::spawn` function comes in.
这就是 `tokio::spawn` 函数的用武之地。

## `tokio::spawn`
## `tokio::spawn`

`tokio::spawn` allows you to hand off a task to the executor, **without waiting for it to complete**.\
`tokio::spawn` 允许你将任务交给执行器，**而不等待它完成**。\
Whenever you invoke `tokio::spawn`, you're telling `tokio` to continue running
the spawned task, in the background, **concurrently** with the task that spawned it.
每当你调用 `tokio::spawn` 时，你是在告诉 `tokio` 在后台**并发地**继续运行生成的 task。

Here's how you can use it to process multiple connections concurrently:
以下是你如何使用它来并发处理多个连接：

```rust
use tokio::net::TcpListener;

pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        // Spawn a background task to handle the connection
        // thus allowing the main task to immediately start 
        // accepting new connections
        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();
            tokio::io::copy(&mut reader, &mut writer).await?;
        });
    }
}
```

### Asynchronous blocks
### 异步块

In this example, we've passed an **asynchronous block** to `tokio::spawn`: `async move { /* */ }`
Asynchronous blocks are a quick way to mark a region of code as asynchronous without having
to define a separate async function.
在这个例子中，我们向 `tokio::spawn` 传递了一个**异步块**：`async move { /* */ }`。异步块是一种快速将代码区域标记为异步而无需定义单独异步函数的方式。

### `JoinHandle`
### `JoinHandle`

`tokio::spawn` returns a `JoinHandle`.\
`tokio::spawn` 返回一个 `JoinHandle`。\
You can use `JoinHandle` to `.await` the background task, in the same way
we used `join` for spawned threads.
你可以使用 `JoinHandle` 来 `.await` 后台任务，就像我们使用 `join` 处理生成的线程一样。

```rust
pub async fn run() {
    // Spawn a background task to ship telemetry data
    // to a remote server
    let handle = tokio::spawn(emit_telemetry());
    // In the meantime, do some other useful work
    do_work().await;
    // But don't return to the caller until 
    // the telemetry data has been successfully delivered
    handle.await;
}

pub async fn emit_telemetry() {
    // [...]
}

pub async fn do_work() {
    // [...]
}
```

### Panic boundary
### Panic 边界

If a task spawned with `tokio::spawn` panics, the panic will be caught by the executor.\
如果使用 `tokio::spawn` 生成的任务 panic，该 panic 会被执行器捕获。\
If you don't `.await` the corresponding `JoinHandle`, the panic won't be propagated to the spawner.
如果你不 `.await` 对应的 `JoinHandle`，panic 不会被传播到生成者。
Even if you do `.await` the `JoinHandle`, the panic won't be propagated automatically.
即使你 `.await` 了 `JoinHandle`，panic 也不会自动传播。
Awaiting a `JoinHandle` returns a `Result`, with [`JoinError`](https://docs.rs/tokio/latest/tokio/task/struct.JoinError.html)
as its error type. You can then check if the task panicked by calling `JoinError::is_panic` and
choose what to do with the panic—either log it, ignore it, or propagate it.
Await 一个 `JoinHandle` 会返回一个 `Result`，其错误类型为 [`JoinError`](https://docs.rs/tokio/latest/tokio/task/struct.JoinError.html)。然后你可以通过调用 `JoinError::is_panic` 来检查任务是否 panic，并选择如何处理 panic——记录它、忽略它或传播它。

```rust
use tokio::task::JoinError;

pub async fn run() {
    let handle = tokio::spawn(work());
    if let Err(e) = handle.await {
        if let Ok(reason) = e.try_into_panic() {
            // The task has panicked
            // We resume unwinding the panic,
            // thus propagating it to the current task
            panic::resume_unwind(reason);
        }
    }
}

pub async fn work() {
    // [...]
}
```

### `std::thread::spawn` vs `tokio::spawn`
### `std::thread::spawn` vs `tokio::spawn`

You can think of `tokio::spawn` as the asynchronous sibling of `std::thread::spawn`.
你可以将 `tokio::spawn` 视为 `std::thread::spawn` 的异步兄弟。

Notice a key difference: with `std::thread::spawn`, you're delegating control to the OS scheduler.
You're not in control of how threads are scheduled.
注意一个关键区别：使用 `std::thread::spawn`，你将控制权委托给操作系统调度器。你无法控制线程如何被调度。

With `tokio::spawn`, you're delegating to an async executor that runs entirely in
user space. The underlying OS scheduler is not involved in the decision of which task
to run next. We're in charge of that decision now, via the executor we chose to use.
使用 `tokio::spawn`，你将控制权委托给一个完全运行在用户空间的异步执行器。底层的操作系统调度器不参与决定下一步运行哪个任务。现在我们通过选择的执行器来掌控这个决定。
