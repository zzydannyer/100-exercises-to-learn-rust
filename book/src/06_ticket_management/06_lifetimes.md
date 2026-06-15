# Lifetimes
# 生命周期

Let's try to complete the previous exercise by adding an implementation of `IntoIterator` for `&TicketStore`, for
maximum convenience in `for` loops.
让我们尝试通过为 `&TicketStore` 添加 `IntoIterator` 实现来完成上一个练习，以便在 `for` 循环中获得最大的便利。

Let's start by filling in the most "obvious" parts of the implementation:
让我们从填充实现中最"明显"的部分开始：

```rust
impl IntoIterator for &TicketStore {
    type Item = &Ticket;
    type IntoIter = // What goes here?

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.iter()
    }
}
```

What should `type IntoIter` be set to?\
`type IntoIter` 应该设置为什么？\
Intuitively, it should be the type returned by `self.tickets.iter()`, i.e. the type returned by `Vec::iter()`.\
直觉上，它应该是 `self.tickets.iter()` 返回的类型，即 `Vec::iter()` 返回的类型。\
If you check the standard library documentation, you'll find that `Vec::iter()` returns an `std::slice::Iter`.
The definition of `Iter` is:
如果你查看标准库文档，你会发现 `Vec::iter()` 返回一个 `std::slice::Iter`。`Iter` 的定义是：

```rust
pub struct Iter<'a, T> { /* fields omitted */ }
```

`'a` is a **lifetime parameter**.
`'a` 是一个**生命周期参数**。

## Lifetime parameters
## 生命周期参数

Lifetimes are **labels** used by the Rust compiler to keep track of how long a reference (either mutable or
immutable) is valid.\
生命周期是 Rust 编译器用来跟踪引用（无论是可变的还是不可变的）有效时间有多长的**标签**。\
The lifetime of a reference is constrained by the scope of the value it refers to. Rust always makes sure, at compile-time,
that references are not used after the value they refer to has been dropped, to avoid dangling pointers and use-after-free bugs.
引用的生命周期受其引用的值的作用域约束。Rust 总是确保在编译时，引用不会在其引用的值被丢弃后被使用，以避免悬垂指针和释放后使用错误。

This should sound familiar: we've already seen these concepts in action when we discussed ownership and borrowing.
Lifetimes are just a way to **name** how long a specific reference is valid.
这听起来应该很熟悉：我们在讨论所有权和借用时已经看到过这些概念。生命周期只是**命名**特定引用有效多长时间的一种方式。

Naming becomes important when you have multiple references and you need to clarify how they **relate to each other**.
当你拥有多个引用并且需要阐明它们之间的**关系**时，命名就变得很重要。
Let's look at the signature of `Vec::iter()`:
让我们看看 `Vec::iter()` 的签名：

```rust
impl <T> Vec<T> {
    // Slightly simplified
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // [...]
    }
}
```

`Vec::iter()` is generic over a lifetime parameter, named `'a`.\
`Vec::iter()` 对一个名为 `'a` 的生命周期参数是泛型的。\
`'a` is used to **tie together** the lifetime of the `Vec` and the lifetime of the `Iter` returned by `iter()`.
In plain English: the `Iter` returned by `iter()` cannot outlive the `Vec` reference (`&self`) it was created from.
`'a` 用于**关联** `Vec` 的生命周期和 `iter()` 返回的 `Iter` 的生命周期。用通俗的话说：`iter()` 返回的 `Iter` 不能比创建它的 `Vec` 引用（`&self`）存活得更久。

This is important because `Vec::iter`, as we discussed, returns an iterator over **references** to the `Vec`'s elements.
If the `Vec` is dropped, the references returned by the iterator would be invalid. Rust must make sure this doesn't happen,
and lifetimes are the tool it uses to enforce this rule.
这很重要，因为正如我们讨论过的，`Vec::iter` 返回一个对 `Vec` 元素的**引用**的迭代器。如果 `Vec` 被丢弃，迭代器返回的引用将无效。Rust 必须确保这不会发生，而生命周期就是它用来强制执行这一规则的工具。

## Lifetime elision
## 生命周期省略

Rust has a set of rules, called **lifetime elision rules**, that allow you to omit explicit lifetime annotations in many cases.
Rust 有一套规则，称为**生命周期省略规则**，它们允许你在许多情况下省略显式的生命周期注解。
For example, `Vec::iter`'s definition looks like this in `std`'s source code:
例如，`Vec::iter` 的定义在 `std` 的源代码中看起来像这样：

```rust
impl <T> Vec<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        // [...]
    }
}
```

No explicit lifetime parameter is present in the signature of `Vec::iter()`.
`Vec::iter()` 的签名中没有显式的生命周期参数。
Elision rules imply that the lifetime of the `Iter` returned by `iter()` is tied to the lifetime of the `&self` reference.
You can think of `'_` as a **placeholder** for the lifetime of the `&self` reference.
省略规则暗示 `iter()` 返回的 `Iter` 的生命周期与 `&self` 引用的生命周期相关联。你可以将 `'_` 视为 `&self` 引用的生命周期的**占位符**。

See the [References](#references) section for a link to the official documentation on lifetime elision.\
查看[参考资料](#references)部分，了解关于生命周期省略的官方文档链接。\
In most cases, you can rely on the compiler telling you when you need to add explicit lifetime annotations.
在大多数情况下，你可以依赖编译器告诉你何时需要添加显式的生命周期注解。

## References
## 参考资料

- [std::vec::Vec::iter](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter)
- [std::slice::Iter](https://doc.rust-lang.org/std/slice/struct.Iter.html)
- [Lifetime elision rules](https://doc.rust-lang.org/reference/lifetime-elision.html)
- [生命周期省略规则](https://doc.rust-lang.org/reference/lifetime-elision.html)
