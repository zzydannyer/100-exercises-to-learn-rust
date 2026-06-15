# Readers and writers
# 读者与写者

Our new `TicketStore` works, but its read performance is not great: there can only be one client at a time
reading a specific ticket, because `Mutex<T>` doesn't distinguish between readers and writers.
我们新的 `TicketStore` 可以工作，但它的读取性能并不好：一次只能有一个客户端读取特定的票证，因为 `Mutex<T>` 不区分读者和写者。

We can solve the issue by using a different locking primitive: `RwLock<T>`.\
我们可以通过使用不同的锁原语来解决这个问题：`RwLock<T>`。\
`RwLock<T>` stands for **read-write lock**. It allows **multiple readers** to access the data simultaneously,
but only one writer at a time.
`RwLock<T>` 代表**读写锁**。它允许**多个读者**同时访问数据，但一次只能有一个写者。

`RwLock<T>` has two methods to acquire a lock: `read` and `write`.\
`RwLock<T>` 有两个获取锁的方法：`read` 和 `write`。\
`read` returns a guard that allows you to read the data, while `write` returns a guard that allows you to modify it.
`read` 返回一个允许你读取数据的 guard，而 `write` 返回一个允许你修改数据的 guard。

```rust
use std::sync::RwLock;

// An integer protected by a read-write lock
let lock = RwLock::new(0);

// Acquire a read lock on the RwLock
let guard1 = lock.read().unwrap();

// Acquire a **second** read lock
// while the first one is still active
let guard2 = lock.read().unwrap();
```

## Trade-offs
## 权衡

On the surface, `RwLock<T>` seems like a no-brainer: it provides a superset of the functionality of `Mutex<T>`.
Why would you ever use `Mutex<T>` if you can use `RwLock<T>` instead?
表面上，`RwLock<T>` 似乎是不需要动脑筋的选择：它提供了 `Mutex<T>` 功能的超集。如果你可以使用 `RwLock<T>`，为什么还要用 `Mutex<T>` 呢？

There are two key reasons:
有两个关键原因：

- Locking a `RwLock<T>` is more expensive than locking a `Mutex<T>`.\
  锁定 `RwLock<T>` 比锁定 `Mutex<T>` 更昂贵。\
  This is because `RwLock<T>` has to keep track of the number of active readers and writers, while `Mutex<T>`
  only has to keep track of whether the lock is held or not.
  This performance overhead is not an issue if there are more readers than writers, but if the workload
  is write-heavy `Mutex<T>` might be a better choice.
  这是因为 `RwLock<T>` 必须跟踪活跃读者和写者的数量，而 `Mutex<T>` 只需要跟踪锁是否被持有。如果读者多于写者，这种性能开销不是问题，但如果工作负载以写为主，`Mutex<T>` 可能是更好的选择。
- `RwLock<T>` can cause **writer starvation**.\
  `RwLock<T>` 可能导致**写者饥饿**。\
  If there are always readers waiting to acquire the lock, writers might never get a chance to run.\
  如果总有读者在等待获取锁，写者可能永远没有机会运行。\
  `RwLock<T>` doesn't provide any guarantees about the order in which readers and writers are granted access to the lock.
  It depends on the policy implemented by the underlying OS, which might not be fair to writers.
  `RwLock<T>` 不提供关于读者和写者被授予锁访问权限的顺序的任何保证。这取决于底层操作系统实现的策略，该策略可能对写者不公平。

In our case, we can expect the workload to be read-heavy (since most clients will be reading tickets, not modifying them),
so `RwLock<T>` is a good choice.
在我们的例子中，我们可以预期工作负载以读为主（因为大多数客户端将读取票证而不是修改它们），所以 `RwLock<T>` 是一个不错的选择。
