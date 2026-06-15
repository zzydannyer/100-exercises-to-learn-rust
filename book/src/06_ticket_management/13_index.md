# Indexing
# 索引

`TicketStore::get` returns an `Option<&Ticket>` for a given `TicketId`.\
`TicketStore::get` 为给定的 `TicketId` 返回一个 `Option<&Ticket>`。\
We've seen before how to access elements of arrays and vectors using Rust's
indexing syntax:
我们之前已经看过如何使用 Rust 的索引语法访问数组和向量的元素：

```rust
let v = vec![0, 1, 2];
assert_eq!(v[0], 0);
```

How can we provide the same experience for `TicketStore`?\
我们如何为 `TicketStore` 提供相同的体验？\
You guessed right: we need to implement a trait, `Index`!
你猜对了：我们需要实现一个特征 `Index`！

## `Index`
## `Index`

The `Index` trait is defined in Rust's standard library:
`Index` 特征定义在 Rust 的标准库中：

```rust
// Slightly simplified
pub trait Index<Idx>
{
    type Output;

    // Required method
    fn index(&self, index: Idx) -> &Self::Output;
}
```

It has:
它有：

- One generic parameter, `Idx`, to represent the index type
- 一个泛型参数 `Idx`，表示索引类型
- One associated type, `Output`, to represent the type we retrieved using the index
- 一个关联类型 `Output`，表示我们使用索引检索到的类型

Notice how the `index` method doesn't return an `Option`. The assumption is that
`index` will panic if you try to access an element that's not there, as it happens
for array and vec indexing.
注意 `index` 方法不返回 `Option`。这里的假设是，如果你尝试访问一个不存在的元素，`index` 会恐慌，就像数组和 vec 索引一样。
