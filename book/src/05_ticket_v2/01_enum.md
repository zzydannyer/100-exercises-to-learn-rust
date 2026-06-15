# Enumerations
# 枚举

Based on the validation logic you wrote [in a previous chapter](../03_ticket_v1/02_validation.md),
there are only a few valid statuses for a ticket: `To-Do`, `InProgress` and `Done`.\
根据你在[前一章](../03_ticket_v1/02_validation.md)中编写的验证逻辑，票据只有几个有效状态：`To-Do`、`InProgress` 和 `Done`。\
This is not obvious if we look at the `status` field in the `Ticket` struct or at the type of the `status`
parameter in the `new` method:
如果我们查看 `Ticket` 结构体中的 `status` 字段或 `new` 方法中 `status` 参数的类型，这一点并不明显：

```rust
#[derive(Debug, PartialEq)]
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: String
    ) -> Self {
        // [...]
    }
}
```

In both cases we're using `String` to represent the `status` field.
在这两种情况下，我们都在使用 `String` 来表示 `status` 字段。
`String` is a very general type—it doesn't immediately convey the information that the `status` field
has a limited set of possible values. Even worse, the caller of `Ticket::new` will only find out **at runtime**
if the status they provided is valid or not.
`String` 是一个非常通用的类型——它不会立即传达出 `status` 字段只有一组有限可能值的信息。更糟糕的是，`Ticket::new` 的调用者只有在**运行时**才会发现他们提供的状态是否有效。

We can do better than that with **enumerations**.
我们可以用**枚举**做得更好。

## `enum`
## `enum`

An enumeration is a type that can have a fixed set of values, called **variants**.\
枚举是一种可以有固定一组值的类型，这些值称为**变体**。\
In Rust, you define an enumeration using the `enum` keyword:
在 Rust 中，你使用 `enum` 关键字定义一个枚举：

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

`enum`, just like `struct`, defines **a new Rust type**.
`enum`，就像 `struct` 一样，定义了**一个新的 Rust 类型**。
