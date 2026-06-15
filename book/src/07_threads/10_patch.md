# Update operations
# 更新操作

So far we've implemented only insertion and retrieval operations.\
到目前为止我们只实现了插入和检索操作。\
Let's see how we can expand the system to provide an update operation.
让我们看看如何扩展系统以提供更新操作。

## Legacy updates
## 旧版更新

In the non-threaded version of the system, updates were fairly straightforward: `TicketStore` exposed a
`get_mut` method that allowed the caller to obtain a mutable reference to a ticket, and then modify it.
在系统的非线程版本中，更新相当直接：`TicketStore` 暴露了一个 `get_mut` 方法，允许调用者获取对票证的可变引用，然后修改它。

## Multithreaded updates
## 多线程更新

The same strategy won't work in the current multithreaded version. The borrow checker would
stop us: `SyncSender<&mut Ticket>` isn't `'static` because `&mut Ticket` doesn't satisfy the `'static` lifetime, therefore
they can't be captured by the closure that gets passed to `std::thread::spawn`.
相同的策略在当前的多线程版本中行不通。借用检查器会阻止我们：`SyncSender<&mut Ticket>` 不是 `'static` 的，因为 `&mut Ticket` 不满足 `'static` 生命周期，因此它们不能被传递给 `std::thread::spawn` 的闭包捕获。

There are a few ways to work around this limitation. We'll explore a few of them in the following exercises.
有几种方法可以绕过这个限制。我们将在接下来的练习中探索其中的一些。

### Patching
### 补丁

We can't send a `&mut Ticket` over a channel, therefore we can't mutate on the client-side.\
我们不能通过通道发送 `&mut Ticket`，因此我们无法在客户端进行修改。\
Can we mutate on the server-side?
我们能在服务器端进行修改吗？

We can, if we tell the server what needs to be changed. In other words, if we send a **patch** to the server:
我们可以，如果我们告诉服务器需要改变什么。换句话说，如果我们向服务器发送一个**补丁**：

```rust
struct TicketPatch {
    id: TicketId,
    title: Option<TicketTitle>,
    description: Option<TicketDescription>,
    status: Option<TicketStatus>,
}
```

The `id` field is mandatory, since it's required to identify the ticket that needs to be updated.\
`id` 字段是必需的，因为需要它来标识需要更新的票证。\
All other fields are optional:
所有其他字段都是可选的：

- If a field is `None`, it means that the field should not be changed.
- 如果一个字段是 `None`，意味着该字段不应该被改变。
- If a field is `Some(value)`, it means that the field should be changed to `value`.
- 如果一个字段是 `Some(value)`，意味着该字段应该被改为 `value`。
