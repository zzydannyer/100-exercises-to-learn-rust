# Interior mutability
# 内部可变性

Let's take a moment to reason about the signature of `Sender`'s `send`:
让我们花点时间来思考一下 `Sender` 的 `send` 方法的签名：

```rust
impl<T> Sender<T> {
    pub fn send(&self, t: T) -> Result<(), SendError<T>> {
        // [...]
    }
}
```

`send` takes `&self` as its argument.\
`send` 接受 `&self` 作为其参数。\
But it's clearly causing a mutation: it's adding a new message to the channel.
但它明显在引起变化：它正在向通道添加一条新消息。
What's even more interesting is that `Sender` is cloneable: we can have multiple instances of `Sender`
trying to modify the channel state **at the same time**, from different threads.
更有趣的是 `Sender` 是可克隆的：我们可以有多个 `Sender` 实例尝试**同时**从不同线程修改通道状态。

That's the key property we are using to build this client-server architecture. But why does it work?
Doesn't it violate Rust's rules about borrowing? How are we performing mutations via an _immutable_ reference?
这就是我们构建这个客户端-服务器架构所使用的关键特性。但为什么它能工作？它不违反 Rust 关于借用的规则吗？我们如何通过_不可变_引用执行修改？

## Shared rather than immutable references
## 共享引用而非不可变引用

When we introduced the borrow-checker, we named the two types of references we can have in Rust:
当我们介绍借用检查器时，我们提到了 Rust 中两种类型的引用：

- immutable references (`&T`)
- 不可变引用（`&T`）
- mutable references (`&mut T`)
- 可变引用（`&mut T`）

It would have been more accurate to name them:
更准确的命名应该是：

- shared references (`&T`)
- 共享引用（`&T`）
- exclusive references (`&mut T`)
- 独占引用（`&mut T`）

Immutable/mutable is a mental model that works for the vast majority of cases, and it's a great one to get started
with Rust. But it's not the whole story, as you've just seen: `&T` doesn't actually guarantee that the data it
points to is immutable.\
不可变/可变是一种在绝大多数情况下都适用的心智模型，也是开始学习 Rust 的好方法。但正如你刚刚所见，这并非全部：`&T` 实际上并不能保证其指向的数据是不可变的。\
Don't worry, though: Rust is still keeping its promises.
不过别担心：Rust 仍然在履行其承诺。
It's just that the terms are a bit more nuanced than they might seem at first.
只是这些术语比它们最初看起来要更细致一些。

## `UnsafeCell`
## `UnsafeCell`

Whenever a type allows you to mutate data through a shared reference, you're dealing with **interior mutability**.
每当一个类型允许你通过共享引用修改数据时，你就在处理**内部可变性**。

By default, the Rust compiler assumes that shared references are immutable. It **optimises your code** based on that assumption.\
默认情况下，Rust 编译器假设共享引用是不可变的。它基于这个假设**优化你的代码**。\
The compiler can reorder operations, cache values, and do all sorts of magic to make your code faster.
编译器可以重排操作、缓存值，以及做各种神奇的事情来让你的代码更快。

You can tell the compiler "No, this shared reference is actually mutable" by wrapping the data in an `UnsafeCell`.\
你可以通过将数据包装在 `UnsafeCell` 中来告诉编译器："不，这个共享引用实际上是可变的"。\
Every time you see a type that allows interior mutability, you can be certain that `UnsafeCell` is involved,
either directly or indirectly.\
每当你看到一个允许内部可变性的类型，你可以确定 `UnsafeCell` 直接或间接参与其中。\
Using `UnsafeCell`, raw pointers and `unsafe` code, you can mutate data through shared references.
使用 `UnsafeCell`、原始指针和 `unsafe` 代码，你可以通过共享引用修改数据。

Let's be clear, though: `UnsafeCell` isn't a magic wand that allows you to ignore the borrow-checker!\
但是要明确一点：`UnsafeCell` 并不是允许你忽略借用检查器的魔法棒！\
`unsafe` code is still subject to Rust's rules about borrowing and aliasing.
`unsafe` 代码仍然受 Rust 关于借用和别名的规则约束。
It's an (advanced) tool that you can leverage to build **safe abstractions** whose safety can't be directly expressed
in Rust's type system. Whenever you use the `unsafe` keyword you're telling the compiler:
"I know what I'm doing, I won't violate your invariants, trust me."
这是一个（高级）工具，你可以利用它来构建其安全性无法直接在 Rust 类型系统中表达的**安全抽象**。每当你使用 `unsafe` 关键字时，你是在告诉编译器："我知道我在做什么，我不会违反你的不变量，相信我。"

Every time you call an `unsafe` function, there will be documentation explaining its **safety preconditions**:
under what circumstances it's safe to execute its `unsafe` block. You can find the ones for `UnsafeCell`
[in `std`'s documentation](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html).
每次你调用 `unsafe` 函数时，都会有文档解释其**安全前提条件**：在什么情况下执行其 `unsafe` 块是安全的。你可以在 [`std` 的文档](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html)中找到 `UnsafeCell` 的相关说明。

We won't be using `UnsafeCell` directly in this course, nor will we be writing `unsafe` code.
But it's important to know that it's there, why it exists and how it relates to the types you use
every day in Rust.
我们不会在本课程中直接使用 `UnsafeCell`，也不会编写 `unsafe` 代码。但了解它存在、为什么存在以及它与你日常在 Rust 中使用的类型之间的关系是很重要的。

## Key examples
## 关键示例

Let's go through a couple of important `std` types that leverage interior mutability.\
让我们来看几个利用内部可变性的重要 `std` 类型。\
These are types that you'll encounter somewhat often in Rust code, especially if you peek under the hood of
some the libraries you use.
这些是在 Rust 代码中你会经常遇到的类型，特别是如果你窥探一些你使用的库的内部实现的话。

### Reference counting
### 引用计数

`Rc` is a reference-counted pointer.\
`Rc` 是一个引用计数指针。\
It wraps around a value and keeps track of how many references to the value exist.
它包装一个值并跟踪对该值的引用数量。
When the last reference is dropped, the value is deallocated.\
当最后一个引用被丢弃时，该值会被释放。\
The value wrapped in an `Rc` is immutable: you can only get shared references to it.
包装在 `Rc` 中的值是不可变的：你只能获得对其的共享引用。

```rust
use std::rc::Rc;

let a: Rc<String> = Rc::new("My string".to_string());
// Only one reference to the string data exists.
assert_eq!(Rc::strong_count(&a), 1);

// When we call `clone`, the string data is not copied!
// Instead, the reference count for `Rc` is incremented.
let b = Rc::clone(&a);
assert_eq!(Rc::strong_count(&a), 2);
assert_eq!(Rc::strong_count(&b), 2);
// ^ Both `a` and `b` point to the same string data
//   and share the same reference counter.
```

`Rc` uses `UnsafeCell` internally to allow shared references to increment and decrement the reference count.
`Rc` 内部使用 `UnsafeCell` 以允许共享引用增加和减少引用计数。

### `RefCell`
### `RefCell`

`RefCell` is one of the most common examples of interior mutability in Rust.
It allows you to mutate the value wrapped in a `RefCell` even if you only have an
immutable reference to the `RefCell` itself.
`RefCell` 是 Rust 中内部可变性最常见的例子之一。它允许你修改包装在 `RefCell` 中的值，即使你只有对 `RefCell` 本身的不可变引用。

This is done via **runtime borrow checking**.
这是通过**运行时借用检查**来实现的。
The `RefCell` keeps track of the number (and type) of references to the value it contains at runtime.
`RefCell` 在运行时跟踪对其包含的值的引用数量（和类型）。
If you try to borrow the value mutably while it's already borrowed immutably,
the program will panic, ensuring that Rust's borrowing rules are always enforced.
如果你在该值已经被不可变借用时尝试可变借用它，程序会 panic，确保 Rust 的借用规则始终得到执行。

```rust
use std::cell::RefCell;

let x = RefCell::new(42);

let y = x.borrow(); // Immutable borrow
let z = x.borrow_mut(); // Panics! There is an active immutable borrow.
```
