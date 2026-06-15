# Channels
# 通道

All our spawned threads have been fairly short-lived so far.\
到目前为止，我们所有生成的线程生命周期都非常短。\
Get some input, run a computation, return the result, shut down.
获取一些输入，运行计算，返回结果，然后关闭。

For our ticket management system, we want to do something different:
a client-server architecture.
对于我们的票务管理系统，我们想做一些不同的事情：一个客户端-服务器架构。

We will have **one long-running server thread**, responsible for managing
our state, the stored tickets.
我们将有一个**长期运行的服务器线程**，负责管理我们的状态，即存储的票证。

We will then have **multiple client threads**.\
然后我们将有**多个客户端线程**。\
Each client will be able to send **commands** and **queries** to
the stateful thread, in order to change its state (e.g. add a new ticket)
or retrieve information (e.g. get the status of a ticket).\
每个客户端将能够向有状态线程发送**命令**和**查询**，以改变其状态（例如添加新票证）或检索信息（例如获取票证状态）。\
Client threads will run concurrently.
客户端线程将并发运行。

## Communication
## 通信

So far we've only had very limited parent-child communication:
到目前为止，我们只有非常有限的父子通信：

- The spawned thread borrowed/consumed data from the parent context
- 生成的线程从父上下文借用/消费数据
- The spawned thread returned data to the parent when joined
- 生成的线程在 join 时将数据返回给父线程

This isn't enough for a client-server design.\
这对于客户端-服务器设计来说是不够的。\
Clients need to be able to send and receive data from the server thread
_after_ it has been launched.
客户端需要能够在服务器线程启动_之后_与其发送和接收数据。

We can solve the issue using **channels**.
我们可以使用**通道**来解决这个问题。

## Channels
## 通道

Rust's standard library provides **multi-producer, single-consumer** (mpsc) channels
in its `std::sync::mpsc` module.\
Rust 的标准库在其 `std::sync::mpsc` 模块中提供了**多生产者、单消费者**（mpsc）通道。\
There are two channel flavours: bounded and unbounded. We'll stick to the unbounded
version for now, but we'll discuss the pros and cons later on.
有两种通道风格：有界和无界。我们暂时使用无界版本，但稍后会讨论其优缺点。

Channel creation looks like this:
通道的创建如下所示：

```rust
use std::sync::mpsc::channel;

let (sender, receiver) = channel();
```

You get a sender and a receiver.\
你会得到一个发送者和一个接收者。\
You call `send` on the sender to push data into the channel.\
你在发送者上调用 `send` 来将数据推入通道。\
You call `recv` on the receiver to pull data from the channel.
你在接收者上调用 `recv` 来从通道中拉取数据。

### Multiple senders
### 多个发送者

`Sender` is clonable: we can create multiple senders (e.g. one for
each client thread) and they will all push data into the same channel.
`Sender` 是可克隆的：我们可以创建多个发送者（例如每个客户端线程一个），它们都会将数据推入同一个通道。

`Receiver`, instead, is not clonable: there can only be a single receiver
for a given channel.
`Receiver` 则不同，它是不可克隆的：一个给定的通道只能有一个接收者。

That's what **mpsc** (multi-producer single-consumer) stands for!
这就是 **mpsc**（多生产者单消费者）的含义！

### Message type
### 消息类型

Both `Sender` and `Receiver` are generic over a type parameter `T`.\
`Sender` 和 `Receiver` 都在类型参数 `T` 上是泛型的。\
That's the type of the _messages_ that can travel on our channel.
那就是可以在通道上传输的_消息_的类型。

It could be a `u64`, a struct, an enum, etc.
它可以是 `u64`、结构体、枚举等。

### Errors
### 错误

Both `send` and `recv` can fail.\
`send` 和 `recv` 都可能失败。\
`send` returns an error if the receiver has been dropped.\
如果接收者已被丢弃，`send` 会返回错误。\
`recv` returns an error if all senders have been dropped and the channel is empty.
如果所有发送者都已丢弃且通道为空，`recv` 会返回错误。

In other words, `send` and `recv` error when the channel is effectively closed.
换句话说，当通道实际上已关闭时，`send` 和 `recv` 会返回错误。
