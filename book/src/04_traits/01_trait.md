# Traits
# 特征

Let's look again at our `Ticket` type:
让我们再看看我们的 `Ticket` 类型：

```rust
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

All our tests, so far, have been making assertions using `Ticket`'s fields.
到目前为止，我们所有的测试都在使用 `Ticket` 的字段进行断言。

```rust
assert_eq!(ticket.title(), "A new title");
```

What if we wanted to compare two `Ticket` instances directly?
如果我们想直接比较两个 `Ticket` 实例呢？

```rust
let ticket1 = Ticket::new(/* ... */);
let ticket2 = Ticket::new(/* ... */);
ticket1 == ticket2
```

The compiler will stop us:
编译器会阻止我们：

```text
error[E0369]: binary operation `==` cannot be applied to type `Ticket`
  --> src/main.rs:18:13
   |
18 |     ticket1 == ticket2
   |     ------- ^^ ------- Ticket
   |     |
   |     Ticket
   |
note: an implementation of `PartialEq` might be missing for `Ticket`
```

`Ticket` is a new type. Out of the box, there is **no behavior attached to it**.\
`Ticket` 是一个新类型。开箱即用，**没有任何行为附加到它上面**。\
Rust doesn't magically infer how to compare two `Ticket` instances just because they contain `String`s.
Rust 不会仅仅因为两个 `Ticket` 实例包含 `String` 就神奇地推断出如何比较它们。

The Rust compiler is nudging us in the right direction though: it's suggesting that we might be missing an implementation
of `PartialEq`. `PartialEq` is a **trait**!
不过，Rust 编译器正在引导我们朝着正确的方向前进：它暗示我们可能缺少 `PartialEq` 的实现。`PartialEq` 是一个**特征**！

## What are traits?
## 什么是特征？

Traits are Rust's way of defining **interfaces**.\
特征是 Rust 定义**接口**的方式。\
A trait defines a set of methods that a type must implement to satisfy the trait's contract.
特征定义了一组方法，类型必须实现这些方法以满足特征的约定。

### Defining a trait
### 定义一个特征

The syntax for a trait definition goes like this:
特征定义的语法如下：

```rust
trait <TraitName> {
    fn <method_name>(<parameters>) -> <return_type>;
}
```

We might, for example, define a trait named `MaybeZero` that requires its implementors to define an `is_zero` method:
例如，我们可以定义一个名为 `MaybeZero` 的特征，要求其实现者定义一个 `is_zero` 方法：

```rust
trait MaybeZero {
    fn is_zero(self) -> bool;
}
```

### Implementing a trait
### 实现一个特征

To implement a trait for a type we use the `impl` keyword, just like we do for regular[^inherent] methods,
but the syntax is a bit different:
要为一个类型实现一个特征，我们使用 `impl` 关键字，就像我们为常规[^inherent]方法所做的那样，但语法略有不同：

```rust
impl <TraitName> for <TypeName> {
    fn <method_name>(<parameters>) -> <return_type> {
        // Method body
    }
}
```

For example, to implement the `MaybeZero` trait for a custom number type, `WrappingU32`:
例如，为自定义数字类型 `WrappingU32` 实现 `MaybeZero` 特征：

```rust
pub struct WrappingU32 {
    inner: u32,
}

impl MaybeZero for WrappingU32 {
    fn is_zero(self) -> bool {
        self.inner == 0
    }
}
```

### Invoking a trait method
### 调用特征方法

To invoke a trait method, we use the `.` operator, just like we do with regular methods:
要调用特征方法，我们使用 `.` 运算符，就像我们对常规方法所做的那样：

```rust
let x = WrappingU32 { inner: 5 };
assert!(!x.is_zero());
```

To invoke a trait method, two things must be true:
要调用特征方法，必须满足两个条件：

- The type must implement the trait.
- 该类型必须实现该特征。
- The trait must be in scope.
- 该特征必须在作用域内。

To satisfy the latter, you may have to add a `use` statement for the trait:
为了满足后者，你可能需要为该特征添加 `use` 语句：

```rust
use crate::MaybeZero;
```

This is not necessary if:
以下情况则不需要：

- The trait is defined in the same module where the invocation occurs.
- 该特征定义在调用发生的同一模块中。
- The trait is defined in the standard library's **prelude**.
  The prelude is a set of traits and types that are automatically imported into every Rust program.
  It's as if `use std::prelude::*;` was added at the beginning of every Rust module.
- 该特征定义在标准库的 **prelude** 中。
  prelude 是一组自动导入到每个 Rust 程序中的特征和类型。
  就好像在每个 Rust 模块的开头都添加了 `use std::prelude::*;`。

You can find the list of traits and types in the prelude in the
[Rust documentation](https://doc.rust-lang.org/std/prelude/index.html).
你可以在 [Rust 文档](https://doc.rust-lang.org/std/prelude/index.html) 中找到 prelude 中的特征和类型列表。

[^inherent]: A method defined directly on a type, without using a trait, is also known as an **inherent method**.
[^inherent]: 直接在类型上定义的方法，而不使用特征，也称为**固有方法**。
