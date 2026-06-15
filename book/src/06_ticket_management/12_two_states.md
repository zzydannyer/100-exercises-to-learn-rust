# Ticket ids
# 票据 ID

Let's think again about our ticket management system.\
让我们再次思考我们的票据管理系统。\
Our ticket model right now looks like this:
我们当前的票据模型看起来像这样：

```rust
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

One thing is missing here: an **identifier** to uniquely identify a ticket.\
这里缺少了一件事：一个唯一标识票据的**标识符**。\
That identifier should be unique for each ticket. That can be guaranteed by generating it automatically when
a new ticket is created.
该标识符对于每张票据应该是唯一的。这可以通过在创建新票据时自动生成来保证。

## Refining the model
## 精化模型

Where should the id be stored?\
ID 应该存储在哪里？\
We could add a new field to the `Ticket` struct:
我们可以向 `Ticket` 结构体添加一个新字段：

```rust
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

But we don't know the id before creating the ticket. So it can't be there from the get-go.\
但我们在创建票据之前不知道 ID。所以它不能从一开始就在那里。\
It'd have to be optional:
它必须是可选的：

```rust
pub struct Ticket {
    pub id: Option<TicketId>,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

That's also not ideal—we'd have to handle the `None` case every single time we retrieve a ticket from the store,
even though we know that the id should always be there once the ticket has been created.
这也不理想——每次我们从存储中检索票据时，我们都必须处理 `None` 情况，尽管我们知道一旦票据被创建，ID 应该总是在那里。

The best solution is to have two different ticket **states**, represented by two separate types:
a `TicketDraft` and a `Ticket`:
最好的解决方案是拥有两种不同的票据**状态**，由两种不同的类型表示：`TicketDraft` 和 `Ticket`：

```rust
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription
}

pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

A `TicketDraft` is a ticket that hasn't been created yet. It doesn't have an id, and it doesn't have a status.\
`TicketDraft` 是尚未创建的票据。它没有 ID，也没有状态。\
A `Ticket` is a ticket that has been created. It has an id and a status.\
`Ticket` 是已经创建的票据。它有 ID 和状态。\
Since each field in `TicketDraft` and `Ticket` embeds its own constraints, we don't have to duplicate logic
across the two types.
由于 `TicketDraft` 和 `Ticket` 中的每个字段都嵌入了自己的约束，我们不需要在这两种类型之间重复逻辑。
