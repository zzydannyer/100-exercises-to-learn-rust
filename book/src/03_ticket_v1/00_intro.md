# Modelling A Ticket
# 建模工单

The first chapter should have given you a good grasp over some of Rust's primitive types, operators and
basic control flow constructs.\
第一章应该已经让你对 Rust 的一些原始类型、运算符和基本控制流结构有了很好的掌握。\
In this chapter we'll go one step further and cover what makes Rust truly unique: **ownership**.\
在本章中，我们将更进一步，讲解 Rust 真正独特的地方：**所有权**。\
Ownership is what enables Rust to be both memory-safe and performant, with no garbage collector.
所有权使 Rust 既能在内存安全方面出色，又能提供高性能，且无需垃圾回收器。

As our running example, we'll use a (JIRA-like) ticket, the kind you'd use to track bugs, features, or tasks in
a software project.\
作为我们的贯穿示例，我们将使用一个（类似 JIRA 的）工单，就是在软件项目中用来跟踪错误、功能或任务的那种。\
We'll take a stab at modeling it in Rust. It'll be the first iteration—it won't be perfect nor very idiomatic
by the end of the chapter. It'll be enough of a challenge though!\
我们将尝试在 Rust 中建模它。这将是第一次迭代——到本章结束时它不会完美，也不会非常地道。但它将是一个足够的挑战！\
To move forward you'll have to pick up several new Rust concepts, such as:
要继续前进，你需要掌握几个新的 Rust 概念，例如：

- `struct`s, one of Rust's ways to define custom types
- `struct`（结构体），Rust 定义自定义类型的方式之一
- Ownership, references and borrowing
- 所有权、引用和借用
- Memory management: stack, heap, pointers, data layout, destructors
- 内存管理：栈、堆、指针、数据布局、析构函数
- Modules and visibility
- 模块和可见性
- Strings
- 字符串
