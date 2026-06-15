# Mutable indexing
# 可变索引

`Index` allows read-only access. It doesn't let you mutate the value you
retrieved.
`Index` 允许只读访问。它不允许你修改检索到的值。

## `IndexMut`
## `IndexMut`

If you want to allow mutability, you need to implement the `IndexMut` trait.
如果你想允许可变性，你需要实现 `IndexMut` 特征。

```rust
// Slightly simplified
pub trait IndexMut<Idx>: Index<Idx>
{
    // Required method
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}
```

`IndexMut` can only be implemented if the type already implements `Index`,
since it unlocks an _additional_ capability.
`IndexMut` 只能在类型已经实现了 `Index` 的前提下实现，因为它解锁了一个_额外的_能力。
