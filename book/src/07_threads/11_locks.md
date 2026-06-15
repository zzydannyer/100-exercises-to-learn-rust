# Locks, `Send` and `Arc`
# 锁、`Send` 和 `Arc`

The patching strategy you just implemented has a major drawback: it's racy.\
你刚刚实现的补丁策略有一个主要缺点：它存在竞态条件。\
If two clients send patches for the same ticket roughly at same time, the server will apply them in an arbitrary order.
如果两个客户端大致同时为同一个票证发送补丁，服务器会以任意顺序应用它们。
Whoever enqueues their patch last will overwrite the changes made by the other client.
谁最后入队他们的补丁，谁就会覆盖另一个客户端所做的更改。

## Version numbers
## 版本号

We could try to fix this by using a **version number**.\
我们可以尝试通过使用**版本号**来修复这个问题。\
Each ticket gets assigned a version number upon creation, set to `0`.\
每个票证在创建时都会被分配一个版本号，设置为 `0`。\
Whenever a client sends a patch, they must include the current version number of the ticket alongside the
desired changes. The server will only apply the patch if the version number matches the one it has stored.
每当客户端发送补丁时，他们必须同时包含票证的当前版本号和所需的更改。只有当版本号与服务器存储的相匹配时，服务器才会应用该补丁。

In the scenario described above, the server would reject the second patch, because the version number would
have been incremented by the first patch and thus wouldn't match the one sent by the second client.
在上述场景中，服务器会拒绝第二个补丁，因为版本号已经被第一个补丁递增，因此与第二个客户端发送的版本号不匹配。

This approach is fairly common in distributed systems (e.g. when client and servers don't share memory),
and it is known as **optimistic concurrency control**.\
这种方法在分布式系统中相当常见（例如，当客户端和服务器不共享内存时），被称为**乐观并发控制**。\
The idea is that most of the time, conflicts won't happen, so we can optimize for the common case.
其理念是大多数情况下不会发生冲突，所以我们可以针对常见情况进行优化。
You know enough about Rust by now to implement this strategy on your own as a bonus exercise, if you want to.
你现在对 Rust 的了解已经足以在作为额外练习时自己实现这个策略了，如果你想的话。

## Locking
## 锁

We can also fix the race condition by introducing a **lock**.\
我们也可以通过引入**锁**来修复竞态条件。\
Whenever a client wants to update a ticket, they must first acquire a lock on it. While the lock is active,
no other client can modify the ticket.
每当客户端想要更新票证时，必须首先获取其上的锁。当锁处于激活状态时，没有其他客户端可以修改该票证。

Rust's standard library provides two different locking primitives: `Mutex<T>` and `RwLock<T>`.\
Rust 的标准库提供了两种不同的锁原语：`Mutex<T>` 和 `RwLock<T>`。\
Let's start with `Mutex<T>`. It stands for **mut**ual **ex**clusion, and it's the simplest kind of lock:
it allows only one thread to access the data, no matter if it's for reading or writing.
让我们从 `Mutex<T>` 开始。它代表**互斥**，是最简单的一种锁：它只允许一个线程访问数据，无论是读取还是写入。

`Mutex<T>` wraps the data it protects, and it's therefore generic over the type of the data.\
`Mutex<T>` 包装了它保护的数据，因此在数据的类型上是泛型的。\
You can't access the data directly: the type system forces you to acquire a lock first using either `Mutex::lock` or
`Mutex::try_lock`. The former blocks until the lock is acquired, the latter returns immediately with an error if the lock
can't be acquired.\
你不能直接访问数据：类型系统强制你先使用 `Mutex::lock` 或 `Mutex::try_lock` 获取锁。前者会阻塞直到获取到锁，如果无法获取锁，后者会立即返回错误。\
Both methods return a guard object that dereferences to the data, allowing you to modify it. The lock is released when
the guard is dropped.
两种方法都返回一个解引用到数据的 guard 对象，允许你修改它。当 guard 被丢弃时，锁会被释放。

```rust
use std::sync::Mutex;

// An integer protected by a mutex lock
let lock = Mutex::new(0);

// Acquire a lock on the mutex
let mut guard = lock.lock().unwrap();

// Modify the data through the guard,
// leveraging its `Deref` implementation
*guard += 1;

// The lock is released when `data` goes out of scope
// This can be done explicitly by dropping the guard
// or happen implicitly when the guard goes out of scope
drop(guard)
```

## Locking granularity
## 锁的粒度

What should our `Mutex` wrap?\
我们的 `Mutex` 应该包装什么？\
The simplest option would be to wrap the entire `TicketStore` in a single `Mutex`.\
最简单的选项是将整个 `TicketStore` 包装在一个 `Mutex` 中。\
This would work, but it would severely limit the system's performance: you wouldn't be able to read tickets in parallel,
because every read would have to wait for the lock to be released.\
这可行，但会严重限制系统的性能：你将无法并行读取票证，因为每次读取都必须等待锁被释放。\
This is known as **coarse-grained locking**.
这被称为**粗粒度锁**。

It would be better to use **fine-grained locking**, where each ticket is protected by its own lock.
最好使用**细粒度锁**，其中每个票证由自己的锁保护。
This way, clients can keep working with tickets in parallel, as long as they aren't trying to access the same ticket.
这样，只要客户端不试图访问同一个票证，它们就可以继续并行处理票证。

```rust
// The new structure, with a lock for each ticket
struct TicketStore {
    tickets: BTreeMap<TicketId, Mutex<Ticket>>,
}
```

This approach is more efficient, but it has a downside: `TicketStore` has to become **aware** of the multithreaded
nature of the system; up until now, `TicketStore` has been blissfully ignoring the existence of threads.\
这种方法更高效，但它有一个缺点：`TicketStore` 必须变得**意识到**系统的多线程特性；到目前为止，`TicketStore` 一直幸福地忽略线程的存在。\
Let's go for it anyway.
不过我们还是选择这样做。

## Who holds the lock?
## 谁持有锁？

For the whole scheme to work, the lock must be passed to the client that wants to modify the ticket.\
为了让整个方案工作，锁必须被传递给想要修改票证的客户端。\
The client can then directly modify the ticket (as if they had a `&mut Ticket`) and release the lock when they're done.
然后客户端可以直接修改票证（就像他们有一个 `&mut Ticket` 一样）并在完成后释放锁。

This is a bit tricky.\
这有点棘手。\
We can't send a `Mutex<Ticket>` over a channel, because `Mutex` is not `Clone` and
we can't move it out of the `TicketStore`. Could we send the `MutexGuard` instead?
我们不能通过通道发送 `Mutex<Ticket>`，因为 `Mutex` 不是 `Clone`，而且我们不能把它移出 `TicketStore`。我们可以发送 `MutexGuard` 代替吗？

Let's test the idea with a small example:
让我们用一个小例子来测试这个想法：

```rust
use std::thread::spawn;
use std::sync::Mutex;
use std::sync::mpsc::sync_channel;

fn main() {
    let lock = Mutex::new(0);
    let (sender, receiver) = sync_channel(1);
    let guard = lock.lock().unwrap();

    spawn(move || {
        receiver.recv().unwrap();
    });

    // Try to send the guard over the channel
    // to another thread
    sender.send(guard);
}
```

The compiler is not happy with this code:
编译器对这个代码不满意：

```text
error[E0277]: `MutexGuard<'_, i32>` cannot be sent between 
              threads safely
   --> src/main.rs:10:7
    |
10  |   spawn(move || {
    |  _-----_^
    | | |
    | | required by a bound introduced by this call
11  | |     receiver.recv().unwrap();
12  | | });
    | |_^ `MutexGuard<'_, i32>` cannot be sent between threads safely
    |
    = help: the trait `Send` is not implemented for 
            `MutexGuard<'_, i32>`, which is required by 
            `{closure@src/main.rs:10:7: 10:14}: Send`
    = note: required for `Receiver<MutexGuard<'_, i32>>` 
            to implement `Send`
note: required because it's used within this closure
```

`MutexGuard<'_, i32>` is not `Send`: what does it mean?
`MutexGuard<'_, i32>` 不是 `Send` 的：这意味着什么？

## `Send`
## `Send`

`Send` is a marker trait that indicates that a type can be safely transferred from one thread to another.\
`Send` 是一个标记 trait，表示一个类型可以安全地在线程之间传递。\
`Send` is also an auto-trait, just like `Sized`; it's automatically implemented (or not implemented) for your type
by the compiler, based on its definition.\
`Send` 也是一个自动 trait，就像 `Sized` 一样；编译器会根据其定义自动为你的类型实现（或不实现）。\
You can also implement `Send` manually for your types, but it requires `unsafe` since you have to guarantee that the
type is indeed safe to send between threads for reasons that the compiler can't automatically verify.
你也可以为你的类型手动实现 `Send`，但这需要 `unsafe`，因为你必须保证该类型在编译器无法自动验证的原因下确实可以安全地在线程之间发送。

### Channel requirements
### 通道要求

`Sender<T>`, `SyncSender<T>` and `Receiver<T>` are `Send` if and only if `T` is `Send`.\
`Sender<T>`、`SyncSender<T>` 和 `Receiver<T>` 是 `Send` 的当且仅当 `T` 是 `Send` 的。\
That's because they are used to send values between threads, and if the value itself is not `Send`, it would be
unsafe to send it between threads.
这是因为它们被用于在线程之间发送值，如果值本身不是 `Send` 的，那么在线程之间发送它就不安全。

### `MutexGuard`
### `MutexGuard`

`MutexGuard` is not `Send` because the underlying operating system primitives that `Mutex` uses to implement
the lock require (on some platforms) that the lock must be released by the same thread that acquired it.\
`MutexGuard` 不是 `Send` 的，因为 `Mutex` 用来实现锁的底层操作系统原语要求（在某些平台上）锁必须由获取它的同一线程释放。\
If we were to send a `MutexGuard` to another thread, the lock would be released by a different thread, which would
lead to undefined behavior.
如果我们把 `MutexGuard` 发送到另一个线程，锁将由一个不同的线程释放，这将导致未定义行为。

## Our challenges
## 我们的挑战

Summing it up:
总结一下：

- We can't send a `MutexGuard` over a channel. So we can't lock on the server-side and then modify the ticket on the
  client-side.
- 我们不能通过通道发送 `MutexGuard`。所以我们不能在服务器端加锁，然后在客户端修改票证。
- We can send a `Mutex` over a channel because it's `Send` as long as the data it protects is `Send`, which is the
  case for `Ticket`.
  At the same time, we can't move the `Mutex` out of the `TicketStore` nor clone it.
- 我们可以通过通道发送 `Mutex`，因为只要它保护的数据是 `Send` 的，它就是 `Send` 的，而 `Ticket` 就是这样的。同时，我们不能将 `Mutex` 移出 `TicketStore`，也不能克隆它。

How can we solve this conundrum?\
我们如何解决这个难题？\
We need to look at the problem from a different angle.
我们需要从一个不同的角度来看待这个问题。
To lock a `Mutex`, we don't need an owned value. A shared reference is enough, since `Mutex` uses internal mutability:
为了锁定 `Mutex`，我们不需要一个拥有的值。共享引用就足够了，因为 `Mutex` 使用内部可变性：

```rust
impl<T> Mutex<T> {
    // `&self`, not `self`!
    pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
        // Implementation details
    }
}
```

It is therefore enough to send a shared reference to the client.\
因此，向客户端发送一个共享引用就足够了。\
We can't do that directly, though, because the reference would have to be `'static` and that's not the case.\
但我们不能直接这样做，因为引用必须是 `'static` 的，而事实并非如此。\
In a way, we need an "owned shared reference". It turns out that Rust has a type that fits the bill: `Arc`.
从某种意义上说，我们需要一个"拥有的共享引用"。事实证明 Rust 有一个符合要求的类型：`Arc`。

## `Arc` to the rescue
## `Arc` 来救援

`Arc` stands for **atomic reference counting**.\
`Arc` 代表**原子引用计数**。\
`Arc` wraps around a value and keeps track of how many references to the value exist.
`Arc` 包装一个值并跟踪对该值的引用数量。
When the last reference is dropped, the value is deallocated.\
当最后一个引用被丢弃时，该值会被释放。\
The value wrapped in an `Arc` is immutable: you can only get shared references to it.
包装在 `Arc` 中的值是不可变的：你只能获得对其的共享引用。

```rust
use std::sync::Arc;

let data: Arc<u32> = Arc::new(0);
let data_clone = Arc::clone(&data);

// `Arc<T>` implements `Deref<T>`, so can convert 
// a `&Arc<T>` to a `&T` using deref coercion
let data_ref: &u32 = &data;
```

If you're having a déjà vu moment, you're right: `Arc` sounds very similar to `Rc`, the reference-counted pointer we
introduced when talking about interior mutability. The difference is thread-safety: `Rc` is not `Send`, while `Arc` is.
It boils down to the way the reference count is implemented: `Rc` uses a "normal" integer, while `Arc` uses an
**atomic** integer, which can be safely shared and modified across threads.
如果你有一种既视感，你是对的：`Arc` 听起来与我们在讨论内部可变性时介绍的引用计数指针 `Rc` 非常相似。区别在于线程安全：`Rc` 不是 `Send` 的，而 `Arc` 是。这归结于引用计数的实现方式：`Rc` 使用"普通"整数，而 `Arc` 使用**原子**整数，可以安全地在线程间共享和修改。

## `Arc<Mutex<T>>`
## `Arc<Mutex<T>>`

If we pair `Arc` with `Mutex`, we finally get a type that:
如果我们把 `Arc` 和 `Mutex` 配对使用，我们最终得到一个类型，它：

- Can be sent between threads, because:
- 可以在线程之间发送，因为：
  - `Arc` is `Send` if `T` is `Send`, and
  - 如果 `T` 是 `Send` 的，`Arc` 就是 `Send` 的，而
  - `Mutex` is `Send` if `T` is `Send`.
  - 如果 `T` 是 `Send` 的，`Mutex` 就是 `Send` 的。
  - `T` is `Ticket`, which is `Send`.
  - `T` 是 `Ticket`，它是 `Send` 的。
- Can be cloned, because `Arc` is `Clone` no matter what `T` is.
  Cloning an `Arc` increments the reference count, the data is not copied.
- 可以克隆，因为无论 `T` 是什么，`Arc` 都是 `Clone` 的。克隆 `Arc` 会增加引用计数，数据不会被复制。
- Can be used to modify the data it wraps, because `Arc` lets you get a shared
  reference to `Mutex<T>` which can in turn be used to acquire a lock.
- 可以用来修改它包装的数据，因为 `Arc` 让你获得对 `Mutex<T>` 的共享引用，进而可以用来获取锁。

We have all the pieces we need to implement the locking strategy for our ticket store.
我们有了为票务存储实现锁定策略所需的所有组件。

## Further reading
## 延伸阅读

- We won't be covering the details of atomic operations in this course, but you can find more information
  [in the `std` documentation](https://doc.rust-lang.org/std/sync/atomic/index.html) as well as in the
  ["Rust atomics and locks" book](https://marabos.nl/atomics/).
- 我们不会在本课程中涵盖原子操作的细节，但你可以在 [`std` 文档](https://doc.rust-lang.org/std/sync/atomic/index.html)以及 ["Rust atomics and locks" 书籍](https://marabos.nl/atomics/)中找到更多信息。
