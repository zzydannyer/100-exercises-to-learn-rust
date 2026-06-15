# The `Drop` trait
# `Drop` 特征

When we introduced [destructors](../03_ticket_v1/11_destructor.md),
we mentioned that the `drop` function:
当我们介绍[析构函数](../03_ticket_v1/11_destructor.md)时，我们提到 `drop` 函数：

1. reclaims the memory occupied by the type (i.e. `std::mem::size_of` bytes)
1. 回收该类型占用的内存（即 `std::mem::size_of` 字节）
2. cleans up any additional resources that the value might be managing (e.g. the heap buffer of a `String`)
2. 清理该值可能管理的任何额外资源（例如 `String` 的堆缓冲区）

Step 2. is where the `Drop` trait comes in.
第 2 步就是 `Drop` 特征的用武之地。

```rust
pub trait Drop {
    fn drop(&mut self);
}
```

The `Drop` trait is a mechanism for you to define _additional_ cleanup logic for your types,
beyond what the compiler does for you automatically.\
`Drop` 特征是一种机制，让你为类型定义_额外的_清理逻辑，超出编译器自动为你做的部分。\
Whatever you put in the `drop` method will be executed when the value goes out of scope.
无论你在 `drop` 方法中放什么，都将在该值超出作用域时执行。

## `Drop` and `Copy`
## `Drop` 和 `Copy`

When talking about the `Copy` trait, we said that a type can't implement `Copy` if it
manages additional resources beyond the `std::mem::size_of` bytes that it occupies in memory.
在谈论 `Copy` 特征时，我们说如果一个类型管理超出其在内存中占用的 `std::mem::size_of` 字节之外的额外资源，则不能实现 `Copy`。

You might wonder: how does the compiler know if a type manages additional resources?
That's right: `Drop` trait implementations!\
你可能想知道：编译器如何知道一个类型是否管理额外资源？没错：通过 `Drop` 特征的实现！\
If your type has an explicit `Drop` implementation, the compiler will assume
that your type has additional resources attached to it and won't allow you to implement `Copy`.
如果你的类型有显式的 `Drop` 实现，编译器会假设你的类型附加了额外资源，并且不会允许你实现 `Copy`。

```rust
// This is a unit struct, i.e. a struct with no fields.
// 这是一个单元结构体，即没有字段的结构体。
#[derive(Clone, Copy)]
struct MyType;

impl Drop for MyType {
    fn drop(&mut self) {
       // We don't need to do anything here,
       // it's enough to have an "empty" Drop implementation
    }
}
```

The compiler will complain with this error message:
编译器会报出这个错误信息：

```text
error[E0184]: the trait `Copy` cannot be implemented for this type; 
              the type has a destructor
 --> src/lib.rs:2:17
  |
2 | #[derive(Clone, Copy)]
  |                 ^^^^ `Copy` not allowed on types with destructors
```
