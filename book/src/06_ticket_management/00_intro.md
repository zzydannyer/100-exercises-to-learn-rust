# Intro
# 介绍

In the previous chapter we modelled `Ticket` in a vacuum: we defined its fields and their constraints, we learned
how to best represent them in Rust, but we didn't consider how `Ticket` fits into a larger system.
在上一章中，我们在真空中对 `Ticket` 进行建模：我们定义了它的字段及其约束，学习了如何在 Rust 中最好地表示它们，但我们没有考虑 `Ticket` 如何适应更大的系统。
We'll use this chapter to build a simple workflow around `Ticket`, introducing a (rudimentary) management system to
store and retrieve tickets.
我们将用本章来构建一个围绕 `Ticket` 的简单工作流，引入一个（基础的）管理系统来存储和检索票据。

The task will give us an opportunity to explore new Rust concepts, such as:
这个任务将给我们一个机会来探索新的 Rust 概念，例如：

- Stack-allocated arrays
- 栈分配数组
- `Vec`, a growable array type
- `Vec`，一个可增长的数组类型
- `Iterator` and `IntoIterator`, for iterating over collections
- `Iterator` 和 `IntoIterator`，用于迭代集合
- Slices (`&[T]`), to work with parts of a collection
- 切片（`&[T]`），用于处理集合的一部分
- Lifetimes, to describe how long references are valid
- 生命周期，用于描述引用有效的时间长度
- `HashMap` and `BTreeMap`, two key-value data structures
- `HashMap` 和 `BTreeMap`，两种键值数据结构
- `Eq` and `Hash`, to compare keys in a `HashMap`
- `Eq` 和 `Hash`，用于比较 `HashMap` 中的键
- `Ord` and `PartialOrd`, to work with a `BTreeMap`
- `Ord` 和 `PartialOrd`，用于处理 `BTreeMap`
- `Index` and `IndexMut`, to access elements in a collection
- `Index` 和 `IndexMut`，用于访问集合中的元素
