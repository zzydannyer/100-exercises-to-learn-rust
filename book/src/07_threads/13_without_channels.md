# Design review
# 设计回顾

Let's take a moment to review the journey we've been through.
让我们花点时间来回顾一下我们走过的心路历程。

## Lockless with channel serialization
## 使用通道序列化的无锁设计

Our first implementation of a multithreaded ticket store used:
我们的第一个多线程票务存储实现使用了：

- a single long-lived thread (server), to hold the shared state
- 一个长期运行的线程（服务器），来持有共享状态
- multiple clients sending requests to it via channels from their own threads.
- 多个客户端从其自己的线程通过通道向它发送请求。

No locking of the state was necessary, since the server was the only one modifying the state. That's because
the "inbox" channel naturally **serialized** incoming requests: the server would process them one by one.\
不需要对状态进行锁定，因为服务器是唯一修改状态的线程。这是因为"收件箱"通道自然地**序列化**了传入的请求：服务器会逐个处理它们。\
We've already discussed the limitations of this approach when it comes to patching behaviour, but we didn't
discuss the performance implications of the original design: the server could only process one request at a time,
including reads.
我们已经讨论过这种方法在补丁行为方面的局限性，但我们没有讨论原始设计的性能影响：服务器一次只能处理一个请求，包括读取。

## Fine-grained locking
## 细粒度锁定

We then moved to a more sophisticated design, where each ticket was protected by its own lock and
clients could independently decide if they wanted to read or atomically modify a ticket, acquiring the appropriate lock.
然后我们转向了一个更复杂的设计，其中每个票证由自己的锁保护，客户端可以独立决定他们是想要读取还是原子地修改票证，并获取相应的锁。

This design allows for better parallelism (i.e. multiple clients can read tickets at the same time), but it is
still fundamentally **serial**: the server processes commands one by one. In particular, it hands out locks to clients
one by one.
这个设计允许更好的并行性（即多个客户端可以同时读取票证），但它本质上仍然是**串行**的：服务器逐个处理命令。具体来说，它逐个向客户端分发锁。

Could we remove the channels entirely and allow clients to directly access the `TicketStore`, relying exclusively on
locks to synchronize access?
我们能否完全移除通道，允许客户端直接访问 `TicketStore`，完全依靠锁来同步访问？

## Removing channels
## 移除通道

We have two problems to solve:
我们需要解决两个问题：

- Sharing `TicketStore` across threads
- 在线程间共享 `TicketStore`
- Synchronizing access to the store
- 同步对存储的访问

### Sharing `TicketStore` across threads
### 在线程间共享 `TicketStore`

We want all threads to refer to the same state, otherwise we don't really have a multithreaded system—we're just
running multiple single-threaded systems in parallel.\
我们希望所有线程引用相同的状态，否则我们并没有真正拥有一个多线程系统——我们只是在并行运行多个单线程系统。\
We've already encountered this problem when we tried to share a lock across threads: we can use an `Arc`.
我们在尝试在线程间共享锁时已经遇到过这个问题：我们可以使用 `Arc`。

### Synchronizing access to the store
### 同步对存储的访问

There is one interaction that's still lockless thanks to the serialization provided by the channels: inserting
(or removing) a ticket from the store.\
由于通道提供的序列化，有一种交互仍然是无锁的：向存储中插入（或从中移除）票证。\
If we remove the channels, we need to introduce (another) lock to synchronize access to the `TicketStore` itself.
如果我们移除通道，我们需要引入（另一个）锁来同步对 `TicketStore` 本身的访问。

If we use a `Mutex`, then it makes no sense to use an additional `RwLock` for each ticket: the `Mutex` will
already serialize access to the entire store, so we wouldn't be able to read tickets in parallel anyway.\
如果我们使用 `Mutex`，那么为每个票证使用额外的 `RwLock` 就没有意义了：`Mutex` 已经序列化了对整个存储的访问，所以我们无论如何也无法并行读取票证。\
If we use a `RwLock`, instead, we can read tickets in parallel. We just need to pause all reads while inserting
or removing a ticket.
相反，如果我们使用 `RwLock`，我们可以并行读取票证。我们只需要在插入或移除票证时暂停所有读取。

Let's go down this path and see where it leads us.
让我们沿着这条路走下去，看看它会通向何方。
