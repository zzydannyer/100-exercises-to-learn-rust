# The `Future` trait
# `Future` trait

## The local `Rc` problem
## 本地 `Rc` 问题

Let's go back to `tokio::spawn`'s signature:
让我们回到 `tokio::spawn` 的签名：

```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
{ /* */ }
```

What does it _actually_ mean for `F` to be `Send`?\
`F` 是 `Send` 的_实际上_意味着什么？\
It implies, as we saw in the previous section, that whatever value it captures from the
spawning environment has to be `Send`. But it goes further than that.
正如我们在上一节中看到的，它意味着从生成环境中捕获的任何值都必须是 `Send` 的。但这不止于此。

Any value that's _held across a .await point_ has to be `Send`.\
任何在 _.await 点之间持有_的值都必须是 `Send` 的。\
Let's look at an example:
让我们看一个例子：

```rust
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    // A value that's not `Send`,
    // created _inside_ the async function
    let non_send = Rc::new(1);
    
    // A `.await` point that does nothing
    yield_now().await;

    // The local non-`Send` value is still needed
    // after the `.await`
    println!("{}", non_send);
}
```

The compiler will reject this code:
编译器会拒绝这段代码：

```text
error: future cannot be sent between threads safely
    |
5   |     tokio::spawn(example());
    |                  ^^^^^^^^^ 
    | future returned by `example` is not `Send`
    |
note: future is not `Send` as this value is used across an await
    |
11  |     let non_send = Rc::new(1);
    |         -------- has type `Rc<i32>` which is not `Send`
12  |     // A `.await` point
13  |     yield_now().await;
    |                 ^^^^^ 
    |   await occurs here, with `non_send` maybe used later
note: required by a bound in `tokio::spawn`
    |
164 |     pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    |            ----- required by a bound in this function
165 |     where
166 |         F: Future + Send + 'static,
    |                     ^^^^ required by this bound in `spawn`
```

To understand why that's the case, we need to refine our understanding of
Rust's asynchronous model.
要理解为什么会这样，我们需要完善对 Rust 异步模型的理解。

## The `Future` trait
## `Future` trait

We stated early on that `async` functions return **futures**, types that implement
the `Future` trait. You can think of a future as a **state machine**.
我们在前面提到过 `async` 函数返回 **futures**，即实现了 `Future` trait 的类型。你可以将 future 视为一个**状态机**。
It's in one of two states:
它处于两种状态之一：

- **pending**: the computation has not finished yet.
- **pending**（挂起）：计算尚未完成。
- **ready**: the computation has finished, here's the output.
- **ready**（就绪）：计算已完成，这是输出。

This is encoded in the trait definition:
这编码在 trait 定义中：

```rust
trait Future {
    type Output;
    
    // Ignore `Pin` and `Context` for now
    fn poll(
      self: Pin<&mut Self>, 
      cx: &mut Context<'_>
    ) -> Poll<Self::Output>;
}
```

### `poll`
### `poll`

The `poll` method is the heart of the `Future` trait.\
`poll` 方法是 `Future` trait 的核心。\
A future on its own doesn't do anything. It needs to be **polled** to make progress.\
future 本身不会做任何事情。它需要被**轮询**才能取得进展。\
When you call `poll`, you're asking the future to do some work.
当你调用 `poll` 时，你是在要求 future 做一些工作。
`poll` tries to make progress, and then returns one of the following:
`poll` 尝试取得进展，然后返回以下之一：

- `Poll::Pending`: the future is not ready yet. You need to call `poll` again later.
- `Poll::Pending`：future 尚未就绪。你需要稍后再次调用 `poll`。
- `Poll::Ready(value)`: the future has finished. `value` is the result of the computation,
  of type `Self::Output`.
- `Poll::Ready(value)`：future 已完成。`value` 是计算的结果，类型为 `Self::Output`。

Once `Future::poll` returns `Poll::Ready`, it should not be polled again: the future has
completed, there's nothing left to do.
一旦 `Future::poll` 返回 `Poll::Ready`，就不应再轮询它：future 已完成，没有剩余的工作了。

### The role of the runtime
### 运行时的角色

You'll rarely, if ever, be calling poll directly.\
你很少（如果有的话）会直接调用 poll。\
That's the job of your async runtime: it has all the required information (the `Context`
in `poll`'s signature) to ensure that your futures are making progress whenever they can.
这是你的异步运行时的工作：它拥有所有必要的信息（`poll` 签名中的 `Context`）来确保你的 future 尽可能取得进展。

## `async fn` and futures
## `async fn` 和 futures

We've worked with the high-level interface, asynchronous functions.\
我们已经使用了高级接口，即异步函数。\
We've now looked at the low-level primitive, the `Future trait`.
我们现在已经查看了底层原语，即 `Future trait`。

How are they related?
它们之间有什么关系？

Every time you mark a function as asynchronous, that function will return a future.
每次你将函数标记为异步，该函数都会返回一个 future。
The compiler will transform the body of your asynchronous function into a **state machine**:
one state for each `.await` point.
编译器会将你的异步函数体转换为一个**状态机**：每个 `.await` 点对应一个状态。

Going back to our `Rc` example:
回到我们的 `Rc` 示例：

```rust
use std::rc::Rc;
use tokio::task::yield_now;

async fn example() {
    let non_send = Rc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
```

The compiler would transform it into an enum that looks somewhat like this:
编译器会将其转换为一个大致如下的枚举：

```rust
pub enum ExampleFuture {
    NotStarted,
    YieldNow(Rc<i32>),
    Terminated,
}
```

When `example` is called, it returns `ExampleFuture::NotStarted`. The future has never
been polled yet, so nothing has happened.\
当 `example` 被调用时，它返回 `ExampleFuture::NotStarted`。该 future 从未被轮询过，所以什么也没有发生。\
When the runtime polls it the first time, `ExampleFuture` will advance until the next
`.await` point: it'll stop at the `ExampleFuture::YieldNow(Rc<i32>)` stage of the state
machine, returning `Poll::Pending`.\
当运行时第一次轮询它时，`ExampleFuture` 会前进到下一个 `.await` 点：它会在状态机的 `ExampleFuture::YieldNow(Rc<i32>)` 阶段停止，返回 `Poll::Pending`。\
When it's polled again, it'll execute the remaining code (`println!`) and
return `Poll::Ready(())`.
当它再次被轮询时，它会执行剩余的代码（`println!`）并返回 `Poll::Ready(())`。

When you look at its state machine representation, `ExampleFuture`,
it is now clear why `example` is not `Send`: it holds an `Rc`, therefore
it cannot be `Send`.
当你查看其状态机表示 `ExampleFuture` 时，现在可以清楚地看到为什么 `example` 不是 `Send` 的：它持有一个 `Rc`，因此不能是 `Send` 的。

## Yield points
## 收益点

As you've just seen with `example`, every `.await` point creates a new intermediate
state in the lifecycle of a future.\
正如你刚从 `example` 中看到的，每个 `.await` 点都会在 future 的生命周期中创建一个新的中间状态。\
That's why `.await` points are also known as **yield points**: your future _yields control_
back to the runtime that was polling it, allowing the runtime to pause it and (if necessary)
schedule another task for execution, thus making progress on multiple fronts concurrently.
这就是为什么 `.await` 点也被称为**收益点**：你的 future 将控制权_交还_给正在轮询它的运行时，允许运行时暂停它并（如有必要）调度另一个任务执行，从而在多个方面同时取得进展。

We'll come back to the importance of yielding in a later section.
我们将在后面的部分回到收益的重要性上。
