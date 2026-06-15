// TODO: you have something to do in each of the modules in this crate!
// TODO: 你在这个 crate 的每个模块中都有事情要做！
mod description;
mod status;
mod title;

// A common pattern in Rust is to split code into multiple (private) modules
// Rust 中一种常见的模式是将代码拆分成多个（私有）模块
// and then re-export the public parts of those modules at the root of the crate.
// 然后在 crate 的根目录重新导出这些模块的公有部分。
//
// This hides the internal structure of the crate from your users, while still
// 这向用户隐藏了 crate 的内部结构，同时仍然
// allowing you to organize your code however you like.
// 允许你按自己喜欢的方式组织代码。
pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

#[derive(Debug, PartialEq, Clone)]
// We no longer need to make the fields private!
// 我们不再需要将字段设为私有了！
// Since each field encapsulates its own validation logic, there is no risk of
// 由于每个字段都封装了自己的验证逻辑，因此不存在
// a user of `Ticket` modifying the fields in a way that would break the
// `Ticket` 的用户以破坏结构体不变式的方式修改字段的风险。
// invariants of the struct.
//
// Careful though: if you had any invariants that spanned multiple fields, you
// 但请注意：如果你有任何跨多个字段的不变式，你
// would need to ensure that those invariants are still maintained and go back
// 需要确保这些不变式仍然得到维护，并回到
// to making the fields private.
// 将字段设为私有的做法。
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
