# Traits
# 特征

In the previous chapter we covered the basics of Rust's type and ownership system.\
在上一章中，我们介绍了 Rust 类型和所有权系统的基础知识。\
It's time to dig deeper: we'll explore **traits**, Rust's take on interfaces.
是时候深入了：我们将探索 **特征**，Rust 对接口的实现方式。

Once you learn about traits, you'll start seeing their fingerprints all over the place.\
一旦你了解了特征，你就会发现它们无处不在。\
In fact, you've already seen traits in action throughout the previous chapter, e.g. `.into()` invocations as well
as operators like `==` and `+`.
事实上，你在上一章中已经看到了特征的实际应用，例如 `.into()` 的调用以及像 `==` 和 `+` 这样的运算符。

On top of traits as a concept, we'll also cover some of the key traits that are defined in Rust's standard library:
除了特征这个概念，我们还将涵盖 Rust 标准库中定义的一些关键特征：

- Operator traits (e.g. `Add`, `Sub`, `PartialEq`, etc.)
- 运算符特征（例如 `Add`、`Sub`、`PartialEq` 等）
- `From` and `Into`, for infallible conversions
- `From` 和 `Into`，用于无失败转换
- `Clone` and `Copy`, for copying values
- `Clone` 和 `Copy`，用于复制值
- `Deref` and deref coercion
- `Deref` 和解引用强制转换
- `Sized`, to mark types with a known size
- `Sized`，用于标记具有已知大小的类型
- `Drop`, for custom cleanup logic
- `Drop`，用于自定义清理逻辑

Since we'll be talking about conversions, we'll seize the opportunity to plug some of the "knowledge gaps"
from the previous chapter—e.g. what is `"A title"`, exactly? Time to learn more about slices too!
既然我们要讨论转换，我们将借此机会填补上一章中的一些"知识空白"——例如，`"A title"` 到底是什么？是时候了解更多关于切片的内容了！
