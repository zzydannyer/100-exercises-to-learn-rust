# Asynchronous functions
# 异步函数

All the functions and methods you've written so far were eager.\
到目前为止你编写的所有函数和方法都是 eager（立即执行）的。\
Nothing happened until you invoked them. But once you did, they ran to
completion: they did **all** their work, and then returned their output.
在你调用它们之前什么都不会发生。但一旦你调用了它们，它们就会运行到完成：它们做**所有**的工作，然后返回输出。

Sometimes that's undesirable.\
有时这并不是我们想要的。\
For example, if you're writing an HTTP server, there might be a lot of
**waiting**: waiting for the request body to arrive, waiting for the
database to respond, waiting for a downstream service to reply, etc.
例如，如果你在编写一个 HTTP 服务器，可能会有很多**等待**：等待请求体到达、等待数据库响应、等待下游服务回复等等。

What if you could do something else while you're waiting?\
如果你能在等待的同时做点别的事情呢？\
What if you could choose to give up midway through a computation?\
如果你可以选择在计算中途放弃呢？\
What if you could choose to prioritise another task over the current one?
如果你可以选择优先处理另一个任务而不是当前任务呢？

That's where **asynchronous functions** come in.
这就是**异步函数**的用武之地。

## `async fn`
## `async fn`

You use the `async` keyword to define an asynchronous function:
你使用 `async` 关键字来定义一个异步函数：

```rust
use tokio::net::TcpListener;

// This function is asynchronous
async fn bind_random() -> TcpListener {
    // [...]
}
```

What happens if you call `bind_random` as you would a regular function?
如果你像调用普通函数那样调用 `bind_random` 会发生什么？

```rust
fn run() {
    // Invoke `bind_random`
    let listener = bind_random();
    // Now what?
}
```

Nothing happens!\
什么都不会发生！\
Rust doesn't start executing `bind_random` when you call it,
not even as a background task (as you might expect based on your experience
with other languages).
当你调用 `bind_random` 时，Rust 不会开始执行它，甚至不会作为后台任务执行（正如你可能基于其他语言的经验所期望的那样）。
Asynchronous functions in Rust are **lazy**: they don't do any work until you
explicitly ask them to.
Rust 中的异步函数是**惰性**的：在你明确要求它们之前，它们不会做任何工作。
Using Rust's terminology, we say that `bind_random` returns a **future**, a type
that represents a computation that may complete later. They're called futures
because they implement the `Future` trait, an interface that we'll examine in
detail later on in this chapter.
用 Rust 的术语来说，我们说 `bind_random` 返回一个 **future**，一种代表可能稍后完成的计算的类型。它们被称为 future，因为它们实现了 `Future` trait，这是一个我们将在本章后面详细检查的接口。

## `.await`
## `.await`

The most common way to ask an asynchronous function to do some work is to use
the `.await` keyword:
让异步函数做一些工作的最常见方式是使用 `.await` 关键字：

```rust
use tokio::net::TcpListener;

async fn bind_random() -> TcpListener {
    // [...]
}

async fn run() {
    // Invoke `bind_random` and wait for it to complete
    let listener = bind_random().await;
    // Now `listener` is ready
}
```

`.await` doesn't return control to the caller until the asynchronous function
has run to completion—e.g. until the `TcpListener` has been created in the example above.
`.await` 直到异步函数运行完成之前不会将控制权返回给调用者——例如，直到上面的示例中 `TcpListener` 被创建。

## Runtimes
## 运行时

If you're puzzled, you're right to be!\
如果你感到困惑，你是有道理的！\
We've just said that the perk of asynchronous functions
is that they don't do **all** their work at once. We then introduced `.await`, which
doesn't return until the asynchronous function has run to completion. Haven't we
just re-introduced the problem we were trying to solve? What's the point?
我们刚刚说异步函数的好处是它们不会一次做完**所有**工作。然后我们引入了 `.await`，它在异步函数运行完成之前不会返回。我们不是又重新引入了我们试图解决的问题吗？那意义何在？

Not quite! A lot happens behind the scenes when you call `.await`!\
并非如此！当你调用 `.await` 时，幕后发生了很多事情！\
You're yielding control to an **async runtime**, also known as an **async executor**.
你将控制权交给了一个**异步运行时**，也称为**异步执行器**。
Executors are where the magic happens: they are in charge of managing all your
ongoing asynchronous **tasks**. In particular, they balance two different goals:
执行器是奇迹发生的地方：它们负责管理所有正在进行的异步**任务**。特别地，它们平衡两个不同的目标：

- **Progress**: they make sure that tasks make progress whenever they can.
- **进度**：它们确保任务尽可能取得进展。
- **Efficiency**: if a task is waiting for something, they try to make sure that
  another task can run in the meantime, fully utilising the available resources.
- **效率**：如果一个任务在等待某事，它们会尝试确保另一个任务可以同时运行，充分利用可用资源。

### No default runtime
### 无默认运行时

Rust is fairly unique in its approach to asynchronous programing: there is
no default runtime. The standard library doesn't ship with one. You need to
bring your own!
Rust 在异步编程的方法上相当独特：没有默认的运行时。标准库没有自带运行时。你需要自带！

In most cases, you'll choose one of the options available in the ecosystem.
在大多数情况下，你会选择生态系统中可用的选项之一。
Some runtimes are designed to be broadly applicable, a solid option for most applications.
一些运行时设计为广泛适用，是大多数应用程序的可靠选择。
`tokio` and `async-std` belong to this category. Other runtimes are optimised for
specific use cases—e.g. `embassy` for embedded systems.
`tokio` 和 `async-std` 属于这一类。其他运行时针对特定用例进行了优化——例如，`embassy` 用于嵌入式系统。

Throughout this course we'll rely on `tokio`, the most popular runtime for general-purpose
asynchronous programming in Rust.
在整个课程中，我们将依赖 `tokio`，这是 Rust 中通用异步编程最流行的运行时。

### `#[tokio::main]`
### `#[tokio::main]`

The entrypoint of your executable, the `main` function, must be a synchronous function.
That's where you're supposed to set up and launch your chosen async runtime.
你的可执行文件的入口点，即 `main` 函数，必须是一个同步函数。你应该在那里设置和启动你选择的异步运行时。

Most runtimes provide a macro to make this easier. For `tokio`, it's `tokio::main`:
大多数运行时提供一个宏来简化这个过程。对于 `tokio`，它是 `tokio::main`：

```rust
#[tokio::main]
async fn main() {
    // Your async code goes here
}
```

which expands to:
它会展开为：

```rust
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(
        // Your async function goes here
        // [...]
    );
}
```

### `#[tokio::test]`
### `#[tokio::test]`

The same goes for tests: they must be synchronous functions.\
测试也是一样：它们必须是同步函数。\
Each test function is run in its own thread, and you're responsible for
setting up and launching an async runtime if you need to run async code
in your tests.\
每个测试函数在自己的线程中运行，如果你需要在测试中运行异步代码，你负责设置和启动异步运行时。\
`tokio` provides a `#[tokio::test]` macro to make this easier:
`tokio` 提供了一个 `#[tokio::test]` 宏来简化这个过程：

```rust
#[tokio::test]
async fn my_test() {
    // Your async test code goes here
}
```
