# Cancellation
# 取消

What happens when a pending future is dropped?\
当一个 pending 的 future 被丢弃时会发生什么？\
The runtime will no longer poll it, therefore it won't make any further progress.
运行时将不再轮询它，因此它不会取得任何进一步进展。
In other words, its execution has been **cancelled**.
换句话说，它的执行已被**取消**。

In the wild, this often happens when working with timeouts.
在实际应用中，这通常发生在处理超时时。
For example:
例如：

```rust
use tokio::time::timeout;
use tokio::sync::oneshot;
use std::time::Duration;

async fn http_call() {
    // [...]
}

async fn run() {
    // Wrap the future with a `Timeout` set to expire in 10 milliseconds.
    let duration = Duration::from_millis(10);
    if let Err(_) = timeout(duration, http_call()).await {
        println!("Didn't receive a value within 10 ms");
    }
}
```

When the timeout expires, the future returned by `http_call` will be cancelled.
Let's imagine that this is `http_call`'s body:
当超时到期时，`http_call` 返回的 future 将被取消。让我们假设这是 `http_call` 的主体：

```rust
use std::net::TcpStream;

async fn http_call() {
    let (stream, _) = TcpStream::connect(/* */).await.unwrap();
    let request: Vec<u8> = /* */;
    stream.write_all(&request).await.unwrap();
}
```

Each yield point becomes a **cancellation point**.\
每个收益点都成为一个**取消点**。\
`http_call` can't be preempted by the runtime, so it can only be discarded after
it has yielded control back to the executor via `.await`.
`http_call` 不能被运行时抢占，所以它只能在通过 `.await` 将控制权交还给执行器之后才能被丢弃。
This applies recursively—e.g. `stream.write_all(&request)` is likely to have multiple
yield points in its implementation. It is perfectly possible to see `http_call` pushing
a _partial_ request before being cancelled, thus dropping the connection and never
finishing transmitting the body.
这递归地应用——例如，`stream.write_all(&request)` 在其实现中可能有多个收益点。完全有可能看到 `http_call` 在取消之前推送了_部分_请求，从而丢弃连接并且永远无法完成传输主体。

## Clean up
## 清理

Rust's cancellation mechanism is quite powerful—it allows the caller to cancel an ongoing task
without needing any form of cooperation from the task itself.\
Rust 的取消机制相当强大——它允许调用者取消正在进行的任务，而无需任务的任何形式的配合。\
At the same time, this can be quite dangerous. It may be desirable to perform a
**graceful cancellation**, to ensure that some clean-up tasks are performed
before aborting the operation.
同时，这可能相当危险。可能希望执行**优雅取消**，以确保在终止操作之前执行一些清理任务。

For example, consider this fictional API for a SQL transaction:
例如，考虑这个虚构的 SQL 事务 API：

```rust
async fn transfer_money(
    connection: SqlConnection,
    payer_id: u64,
    payee_id: u64,
    amount: u64
) -> Result<(), anyhow::Error> {
    let transaction = connection.begin_transaction().await?;
    update_balance(payer_id, amount, &transaction).await?;
    decrease_balance(payee_id, amount, &transaction).await?;
    transaction.commit().await?;
}
```

On cancellation, it'd be ideal to explicitly abort the pending transaction rather
than leaving it hanging.
在取消时，最好明确终止挂起的事务，而不是让它悬而未决。
Rust, unfortunately, doesn't provide a bullet-proof mechanism for this kind of
**asynchronous** clean up operations.
不幸的是，Rust 没有为这种**异步**清理操作提供防弹机制。

The most common strategy is to rely on the `Drop` trait to schedule the required
clean-up work. This can be by:
最常见的策略是依赖 `Drop` trait 来调度所需的清理工作。可以通过以下方式：

- Spawning a new task on the runtime
- 在运行时上生成一个新任务
- Enqueueing a message on a channel
- 在通道上入队一条消息
- Spawning a background thread
- 生成一个后台线程

The optimal choice is contextual.
最佳选择视上下文而定。

## Cancelling spawned tasks
## 取消已生成的任务

When you spawn a task using `tokio::spawn`, you can no longer drop it;
it belongs to the runtime.\
当你使用 `tokio::spawn` 生成一个任务时，你不能再丢弃它；它属于运行时。\
Nonetheless, you can use its `JoinHandle` to cancel it if needed:
尽管如此，如果有需要，你可以使用其 `JoinHandle` 来取消它：

```rust
async fn run() {
    let handle = tokio::spawn(/* some async task */);
    // Cancel the spawned task
    handle.abort();
}
```

## Further reading
## 延伸阅读

- Be extremely careful when using `tokio`'s `select!` macro to "race" two different futures.
  Retrying the same task in a loop is dangerous unless you can ensure **cancellation safety**.
  Check out [`select!`'s documentation](https://tokio.rs/tokio/tutorial/select) for more details.\
  If you need to interleave two asynchronous streams of data (e.g. a socket and a channel), prefer using
  [`StreamExt::merge`](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.merge) instead.
- 在使用 `tokio` 的 `select!` 宏来"竞赛"两个不同的 future 时要格外小心。在循环中重试同一个任务是危险的，除非你能确保**取消安全性**。查看 [`select!` 的文档](https://tokio.rs/tokio/tutorial/select)了解更多细节。如果你需要交错两个异步数据流（例如 socket 和 channel），优先使用 [`StreamExt::merge`](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.merge) 代替。
- A [`CancellationToken`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html) may be
  preferable to `JoinHandle::abort` in some cases.
- 在某些情况下，[`CancellationToken`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html) 可能比 `JoinHandle::abort` 更可取。
