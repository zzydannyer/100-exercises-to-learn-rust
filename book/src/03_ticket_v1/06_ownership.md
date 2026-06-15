# Ownership
# 所有权

If you solved the previous exercise using what this course has taught you so far,
your accessor methods probably look like this:
如果你用本课程迄今为止所教的内容解决了上一个练习，你的访问器方法可能如下所示：

```rust
impl Ticket {
    pub fn title(self) -> String {
        self.title
    }

    pub fn description(self) -> String {
        self.description
    }

    pub fn status(self) -> String {
        self.status
    }
}
```

Those methods compile and are enough to get tests to pass, but in a real-world scenario they won't get you very far.
这些方法可以编译，并且足够让测试通过，但在现实场景中它们不会让你走多远。
Consider this snippet:
考虑这段代码片段：

```rust
if ticket.status() == "To-Do" {
    // We haven't covered the `println!` macro yet,
    // but for now it's enough to know that it prints 
    // a (templated) message to the console
    println!("Your next task is: {}", ticket.title());
}
```

If you try to compile it, you'll get an error:
如果你尝试编译它，你会得到一个错误：

```text
error[E0382]: use of moved value: `ticket`
  --> src/main.rs:30:43
   |
25 |     let ticket = Ticket::new(/* */);
   |         ------ move occurs because `ticket` has type `Ticket`, 
   |                which does not implement the `Copy` trait
26 |     if ticket.status() == "To-Do" {
   |               -------- `ticket` moved due to this method call
...
30 |         println!("Your next task is: {}", ticket.title());
   |                                           ^^^^^^ 
   |                                value used here after move
   |
note: `Ticket::status` takes ownership of the receiver `self`, 
      which moves `ticket`
  --> src/main.rs:12:23
   |
12 |         pub fn status(self) -> String {
   |                       ^^^^
```

Congrats, this is your first borrow-checker error!
恭喜，这是你的第一个借用检查器错误！

## The perks of Rust's ownership system
## Rust 所有权系统的优点

Rust's ownership system is designed to ensure that:
Rust 的所有权系统旨在确保：

- Data is never mutated while it's being read
- 数据在被读取时永远不会被修改
- Data is never read while it's being mutated
- 数据在被修改时永远不会被读取
- Data is never accessed after it has been destroyed
- 数据在被销毁后永远不会被访问

These constraints are enforced by the **borrow checker**, a subsystem of the Rust compiler,
often the subject of jokes and memes in the Rust community.
这些约束由 **借用检查器（borrow checker）** 强制执行，它是 Rust 编译器的一个子系统，经常成为 Rust 社区笑话和梗的主题。

Ownership is a key concept in Rust, and it's what makes the language unique.
所有权是 Rust 中的一个关键概念，也是使这门语言与众不同的原因。
Ownership enables Rust to provide **memory safety without compromising performance**.
所有权使 Rust 能够提供**不牺牲性能的内存安全**。
All these things are true at the same time for Rust:
所有这些对 Rust 来说都是同时成立的：

1. There is no runtime garbage collector
1. 没有运行时垃圾回收器
2. As a developer, you rarely have to manage memory directly
2. 作为开发者，你很少需要直接管理内存
3. You can't cause dangling pointers, double frees, and other memory-related bugs
3. 你不会引起悬垂指针、双重释放和其他与内存相关的错误

Languages like Python, JavaScript, and Java give you 2. and 3., but not 1.\
像 Python、JavaScript 和 Java 这样的语言为你提供了 2 和 3，但没有 1。\
Language like C or C++ give you 1., but neither 2. nor 3.
像 C 或 C++ 这样的语言为你提供了 1，但既没有 2 也没有 3。

Depending on your background, 3. might sound a bit arcane: what is a "dangling pointer"?
What is a "double free"? Why are they dangerous?\
根据你的背景，第 3 点可能听起来有点费解：什么是"悬垂指针"？什么是"双重释放"？为什么它们很危险？\
Don't worry: we'll cover these concepts in more details during the rest of the course.
别担心：我们将在课程的其余部分更详细地介绍这些概念。

For now, though, let's focus on learning how to work within Rust's ownership system.
不过现在，让我们专注于学习如何在 Rust 的所有权系统内工作。

## The owner
## 所有者

In Rust, each value has an **owner**, statically determined at compile-time.
在 Rust 中，每个值都有一个**所有者**，在编译时静态确定。
There is only one owner for each value at any given time.
在任何给定时间，每个值只有一个所有者。

## Move semantics
## 移动语义

Ownership can be transferred.
所有权可以被转移。

If you own a value, for example, you can transfer ownership to another variable:
例如，如果你拥有一个值，你可以将所有权转移到另一个变量：

```rust
let a = "hello, world".to_string(); // <- `a` is the owner of the String
let b = a;  // <- `b` is now the owner of the String
```

Rust's ownership system is baked into the type system: each function has to declare in its signature
_how_ it wants to interact with its arguments.
Rust 的所有权系统融入了类型系统：每个函数必须在其签名中声明它希望_如何_与其参数交互。

So far, all our methods and functions have **consumed** their arguments: they've taken ownership of them.
到目前为止，我们所有的方法和函数都**消耗**了它们的参数：它们取得了参数的所有权。
For example:
例如：

```rust
impl Ticket {
    pub fn description(self) -> String {
        self.description
    }
}
```

`Ticket::description` takes ownership of the `Ticket` instance it's called on.\
`Ticket::description` 取得了它所调用的 `Ticket` 实例的所有权。\
This is known as **move semantics**: ownership of the value (`self`) is **moved** from the caller to
the callee, and the caller can't use it anymore.
这被称为**移动语义**：值（`self`）的所有权从调用者**移动**到被调用者，调用者不能再使用它。

That's exactly the language used by the compiler in the error message we saw earlier:
这正是编译器在我们之前看到的错误消息中使用的语言：

```text
error[E0382]: use of moved value: `ticket`
  --> src/main.rs:30:43
   |
25 |     let ticket = Ticket::new(/* */);
   |         ------ move occurs because `ticket` has type `Ticket`, 
   |                which does not implement the `Copy` trait
26 |     if ticket.status() == "To-Do" {
   |               -------- `ticket` moved due to this method call
...
30 |         println!("Your next task is: {}", ticket.title());
   |                                           ^^^^^^ 
   |                                 value used here after move
   |
note: `Ticket::status` takes ownership of the receiver `self`, 
      which moves `ticket`
  --> src/main.rs:12:23
   |
12 |         pub fn status(self) -> String {
   |                       ^^^^
```

In particular, this is the sequence of events that unfold when we call `ticket.status()`:
具体来说，这是我们调用 `ticket.status()` 时发生的事件序列：

- `Ticket::status` takes ownership of the `Ticket` instance
- `Ticket::status` 取得 `Ticket` 实例的所有权
- `Ticket::status` extracts `status` from `self` and transfers ownership of `status` back to the caller
- `Ticket::status` 从 `self` 中提取 `status` 并将 `status` 的所有权转移回调用者
- The rest of the `Ticket` instance is discarded (`title` and `description`)
- `Ticket` 实例的其余部分被丢弃（`title` 和 `description`）

When we try to use `ticket` again via `ticket.title()`, the compiler complains: the `ticket` value is gone now,
we no longer own it, therefore we can't use it anymore.
当我们尝试通过 `ticket.title()` 再次使用 `ticket` 时，编译器会报错：`ticket` 值已经不存在了，我们不再拥有它，因此我们不能再使用它。

To build _useful_ accessor methods we need to start working with **references**.
要构建_有用的_访问器方法，我们需要开始使用**引用**。

## Borrowing
## 借用

It is desirable to have methods that can read the value of a variable without taking ownership of it.\
拥有能够读取变量值而不取得其所有权的方法是很有必要的。\
Programming would be quite limited otherwise. In Rust, that's done via **borrowing**.
否则编程将受到很大限制。在 Rust 中，这是通过**借用**来实现的。

Whenever you borrow a value, you get a **reference** to it.\
每当你借用值时，你会得到一个**引用**。\
References are tagged with their privileges[^refine]:
引用带有其权限标签[^refine]：

- Immutable references (`&`) allow you to read the value, but not to mutate it
- 不可变引用（`&`）允许你读取值，但不能修改它
- Mutable references (`&mut`) allow you to read and mutate the value
- 可变引用（`&mut`）允许你读取和修改值

Going back to the goals of Rust's ownership system:
回到 Rust 所有权系统的目标：

- Data is never mutated while it's being read
- 数据在被读取时永远不会被修改
- Data is never read while it's being mutated
- 数据在被修改时永远不会被读取

To ensure these two properties, Rust has to introduce some restrictions on references:
为了确保这两个属性，Rust 必须对引用引入一些限制：

- You can't have a mutable reference and an immutable reference to the same value at the same time
- 你不能同时拥有对同一值的可变引用和不可变引用
- You can't have more than one mutable reference to the same value at the same time
- 你不能同时拥有多个对同一值的可变引用
- The owner can't mutate the value while it's being borrowed
- 所有者不能在值被借用时修改它
- You can have as many immutable references as you want, as long as there are no mutable references
- 只要没有可变引用，你可以拥有任意多的不可变引用

In a way, you can think of an immutable reference as a "read-only" lock on the value,
while a mutable reference is like a "read-write" lock.
在某种程度上，你可以把不可变引用看作是值上的"只读"锁，而可变引用就像是"读写"锁。

All these restrictions are enforced at compile-time by the borrow checker.
所有这些限制都在编译时由借用检查器强制执行。

### Syntax
### 语法

How do you borrow a value, in practice?\
在实践中如何借用值呢？\
By adding `&` or `&mut` **in front a variable**, you're borrowing its value.
通过在变量**前**添加 `&` 或 `&mut`，你就在借用它的值。
Careful though! The same symbols (`&` and `&mut`) in **front of a type** have a different meaning:
they denote a different type, a reference to the original type.
不过要小心！相同的符号（`&` 和 `&mut`）在**类型前**有不同的含义：它们表示不同的类型，即对原类型的引用。

For example:
例如：

```rust
struct Configuration {
    version: u32,
    active: bool,
}

fn main() {
    let config = Configuration {
        version: 1,
        active: true,
    };
    // `b` is a reference to the `version` field of `config`.
    // The type of `b` is `&u32`, since it contains a reference to 
    // a `u32` value.
    // We create a reference by borrowing `config.version`, using 
    // the `&` operator.
    // Same symbol (`&`), different meaning depending on the context!
    let b: &u32 = &config.version;
    //     ^ The type annotation is not necessary, 
    //       it's just there to clarify what's going on
}
```

The same concept applies to function arguments and return types:
同样的概念适用于函数参数和返回类型：

```rust
// `f` takes a mutable reference to a `u32` as an argument, 
// bound to the name `number`
fn f(number: &mut u32) -> &u32 {
    // [...]
}
```

## Breathe in, breathe out
## 吸气，呼气

Rust's ownership system can be a bit overwhelming at first.\
Rust 的所有权系统一开始可能有点让人不知所措。\
But don't worry: it'll become second nature with practice.\
但别担心：通过练习它会成为你的第二天性。\
And you're going to get a lot of practice over the rest of this chapter, as well as the rest of the course!
在本章的剩余部分以及课程的剩余部分，你将得到大量的练习！
We'll revisit each concept multiple times to make sure you get familiar with them
and truly understand how they work.
我们会多次复习每个概念，确保你熟悉它们并真正理解它们的工作原理。

Towards the end of this chapter we'll explain _why_ Rust's ownership system is designed the way it is.
到本章末尾，我们将解释_为什么_ Rust 的所有权系统被设计成这样。
For the time being, focus on understanding the _how_. Take each compiler error as a learning opportunity!
目前，专注于理解_如何_。把每个编译器错误都当作学习机会！

[^refine]: This is a great mental model to start out, but it doesn't capture the _full_ picture.
We'll refine our understanding of references [later in the course](../07_threads/06_interior_mutability.md).
[^refine]: 这是一个很好的起步思维模型，但它没有涵盖_全部_情况。我们将在[课程后面](../07_threads/06_interior_mutability.md)完善对引用的理解。
