# Error trait
# 错误特征

## Error reporting
## 错误报告

In the previous exercise you had to destructure the `TitleError` variant to extract the error message and
pass it to the `panic!` macro.\
在上一个练习中，你必须解构 `TitleError` 变体来提取错误消息并将其传递给 `panic!` 宏。\
This is a (rudimentary) example of **error reporting**: transforming an error type into a representation that can be
shown to a user, a service operator, or a developer.
这是一个（基本的）**错误报告**示例：将错误类型转换为可以展示给用户、服务运营人员或开发者的表示形式。

It's not practical for each Rust developer to come up with their own error reporting strategy: it'd be a waste of time
and it wouldn't compose well across projects.
让每个 Rust 开发者都提出自己的错误报告策略是不切实际的：这会浪费时间，而且不能很好地跨项目组合。
That's why Rust provides the `std::error::Error` trait.
这就是为什么 Rust 提供了 `std::error::Error` 特征。

## The `Error` trait
## `Error` 特征

There are no constraints on the type of the `Err` variant in a `Result`, but it's a good practice to use a type
that implements the `Error` trait.
`Result` 中 `Err` 变体的类型没有约束，但最佳实践是使用实现了 `Error` 特征的类型。
`Error` is the cornerstone of Rust's error handling story:
`Error` 是 Rust 错误处理故事的基石：

```rust
// Slightly simplified definition of the `Error` trait
pub trait Error: Debug + Display {}
```

You might recall the `:` syntax from [the `From` trait](../04_traits/09_from.md#supertrait--subtrait)—it's used to specify **supertraits**.
你可能还记得[`From` 特征](../04_traits/09_from.md#supertrait--subtrait)中的 `:` 语法——它用于指定**超特征**。
For `Error`, there are two supertraits: `Debug` and `Display`. If a type wants to implement `Error`, it must also
implement `Debug` and `Display`.
对于 `Error`，有两个超特征：`Debug` 和 `Display`。如果一个类型想要实现 `Error`，它也必须实现 `Debug` 和 `Display`。

## `Display` and `Debug`
## `Display` 和 `Debug`

We've already encountered the `Debug` trait in [a previous exercise](../04_traits/04_derive.md)—it's the trait used by
`assert_eq!` to display the values of the variables it's comparing when the assertion fails.
我们已经在[上一个练习](../04_traits/04_derive.md)中遇到过 `Debug` 特征——它是 `assert_eq!` 在断言失败时用来显示正在比较的变量值的特征。

From a "mechanical" perspective, `Display` and `Debug` are identical—they encode how a type should be converted
into a string-like representation:
从"机制"角度来看，`Display` 和 `Debug` 是相同的——它们编码了类型应该如何转换为类似字符串的表示形式：

```rust
// `Debug`
pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}

// `Display`
pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

The difference is in their _purpose_: `Display` returns a representation that's meant for "end-users",
while `Debug` provides a low-level representation that's more suitable to developers and service operators.\
区别在于它们的_目的_：`Display` 返回面向"最终用户"的表示形式，而 `Debug` 提供更适合开发者和服务运营人员的低级表示形式。\
That's why `Debug` can be automatically implemented using the `#[derive(Debug)]` attribute, while `Display`
**requires** a manual implementation.
这就是为什么 `Debug` 可以通过 `#[derive(Debug)]` 属性自动实现，而 `Display` **需要**手动实现。
