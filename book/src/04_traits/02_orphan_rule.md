# Implementing traits
# 实现特征

When a type is defined in another crate (e.g. `u32`, from Rust's standard library), you
can't directly define new methods for it. If you try:
当一个类型在另一个 crate 中定义时（例如 Rust 标准库中的 `u32`），你不能直接为其定义新方法。如果你尝试：

```rust
impl u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
```

the compiler will complain:
编译器会报错：

```text
error[E0390]: cannot define inherent `impl` for primitive types
  |
1 | impl u32 {
  | ^^^^^^^^
  |
  = help: consider using an extension trait instead
```

## Extension trait
## 扩展特征

An **extension trait** is a trait whose primary purpose is to attach new methods
to foreign types, such as `u32`.
**扩展特征**是一个其主要目的是将新方法附加到外部类型（如 `u32`）上的特征。
That's exactly the pattern you deployed in the previous exercise, by defining
the `IsEven` trait and then implementing it for `i32` and `u32`. You are then
free to call `is_even` on those types as long as `IsEven` is in scope.
这正是你在上一个练习中使用的模式，通过定义 `IsEven` 特征，然后为 `i32` 和 `u32` 实现它。只要 `IsEven` 在作用域内，你就可以在这些类型上自由调用 `is_even`。

```rust
// Bring the trait in scope
use my_library::IsEven;

fn main() {
    // Invoke its method on a type that implements it
    if 4.is_even() {
        // [...]
    }
}
```

## One implementation
## 单一实现

There are limitations to the trait implementations you can write.\
你能编写的特征实现有一些限制。\
The simplest and most straight-forward one: you can't implement the same trait twice,
in a crate, for the same type.
最简单直接的一点是：你不能在一个 crate 中为同一个类型实现同一个特征两次。

For example:
例如：

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        true
    }
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        false
    }
}
```

The compiler will reject it:
编译器会拒绝它：

```text
error[E0119]: conflicting implementations of trait `IsEven` for type `u32`
   |
5  | impl IsEven for u32 {
   | ------------------- first implementation here
...
11 | impl IsEven for u32 {
   | ^^^^^^^^^^^^^^^^^^^ conflicting implementation for `u32`
```

There can be no ambiguity as to what trait implementation should be used when `IsEven::is_even`
is invoked on a `u32` value, therefore there can only be one.
当对 `u32` 值调用 `IsEven::is_even` 时，使用哪个特征实现不能有歧义，因此只能有一个。

## Orphan rule
## 孤儿规则

Things get more nuanced when multiple crates are involved.
当涉及多个 crate 时，情况变得更加微妙。
In particular, at least one of the following must be true:
具体来说，以下条件中至少有一个必须成立：

- The trait is defined in the current crate
- 该特征在当前 crate 中定义
- The implementor type is defined in the current crate
- 实现者类型在当前 crate 中定义

This is known as Rust's **orphan rule**. Its goal is to make the method resolution
process unambiguous.
这被称为 Rust 的**孤儿规则**。其目标是使方法解析过程无歧义。

Imagine the following situation:
想象以下情况：

- Crate `A` defines the `IsEven` trait
- Crate `A` 定义了 `IsEven` 特征
- Crate `B` implements `IsEven` for `u32`
- Crate `B` 为 `u32` 实现了 `IsEven`
- Crate `C` provides a (different) implementation of the `IsEven` trait for `u32`
- Crate `C` 为 `u32` 提供了（不同的）`IsEven` 特征实现
- Crate `D` depends on both `B` and `C` and calls `1.is_even()`
- Crate `D` 同时依赖于 `B` 和 `C`，并调用 `1.is_even()`

Which implementation should be used? The one defined in `B`? Or the one defined in `C`?\
应该使用哪个实现？`B` 中定义的？还是 `C` 中定义的？\
There's no good answer, therefore the orphan rule was defined to prevent this scenario.
没有好的答案，因此定义了孤儿规则来防止这种情况。
Thanks to the orphan rule, neither crate `B` nor crate `C` would compile.
多亏了孤儿规则，crate `B` 和 crate `C` 都无法编译。

## Further reading
## 延伸阅读

- There are some caveats and exceptions to the orphan rule as stated above.
  Check out [the reference](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence)
  if you want to get familiar with its nuances.
- 上面所述的孤儿规则有一些注意事项和例外。
  如果你想了解其细微差别，请查看[参考文档](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence)。
