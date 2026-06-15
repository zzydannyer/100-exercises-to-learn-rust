# `thiserror`
# `thiserror`

That was a bit of detour, wasn't it? But a necessary one!\
这有点绕路了，不是吗？但这是必要的！\
Let's get back on track now: custom error types and `thiserror`.
现在让我们回到正轨：自定义错误类型和 `thiserror`。

## Custom error types
## 自定义错误类型

We've seen how to implement the `Error` trait "manually" for a custom error type.\
我们已经看到了如何为自定义错误类型"手动"实现 `Error` 特征。\
Imagine that you have to do this for most error types in your codebase. That's a lot of boilerplate, isn't it?
想象一下，你必须为代码库中的大多数错误类型这样做。那是大量的样板代码，不是吗？

We can remove some of the boilerplate by using [`thiserror`](https://docs.rs/thiserror/latest/thiserror/),
a Rust crate that provides a **procedural macro** to simplify the creation of custom error types.
我们可以通过使用 [`thiserror`](https://docs.rs/thiserror/latest/thiserror/) 来减少一些样板代码，这是一个 Rust crate，它提供了一个**过程宏**来简化自定义错误类型的创建。

```rust
#[derive(thiserror::Error, Debug)]
enum TicketNewError {
    #[error("{0}")]
    TitleError(String),
    #[error("{0}")]
    DescriptionError(String),
}
```

## You can write your own macros
## 你可以编写自己的宏

All the `derive` macros we've seen so far were provided by the Rust standard library.\
到目前为止我们看到的所有 `derive` 宏都是由 Rust 标准库提供的。\
`thiserror::Error` is the first example of a **third-party** `derive` macro.
`thiserror::Error` 是**第三方** `derive` 宏的第一个例子。

`derive` macros are a subset of **procedural macros**, a way to generate Rust code at compile time.
We won't get into the details of how to write a procedural macro in this course, but it's important
to know that you can write your own!\
`derive` 宏是**过程宏**的一个子集，是一种在编译时生成 Rust 代码的方式。我们不会在本课程中深入介绍如何编写过程宏的细节，但知道你可以自己编写这一点很重要！\
A topic to approach in a more advanced Rust course.
这是一个在更高级的 Rust 课程中再探讨的话题。

## Custom syntax
## 自定义语法

Each procedural macro can define its own syntax, which is usually explained in the crate's documentation.
In the case of `thiserror`, we have:
每个过程宏都可以定义自己的语法，这通常会在 crate 的文档中解释。对于 `thiserror`，我们有：

- `#[derive(thiserror::Error)]`: this is the syntax to derive the `Error` trait for a custom error type, helped by `thiserror`.
- `#[derive(thiserror::Error)]`：这是借助 `thiserror` 为自定义错误类型派生 `Error` 特征的语法。
- `#[error("{0}")]`: this is the syntax to define a `Display` implementation for each variant of the custom error type.
  `{0}` is replaced by the zero-th field of the variant (`String`, in this case) when the error is displayed.
- `#[error("{0}")]`：这是为自定义错误类型的每个变体定义 `Display` 实现的语法。当显示错误时，`{0}` 会被替换为变体的第零个字段（在这种情况下是 `String`）。
