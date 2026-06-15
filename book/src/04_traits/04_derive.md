# Derive macros
# 派生宏

Implementing `PartialEq` for `Ticket` was a bit tedious, wasn't it?
为 `Ticket` 实现 `PartialEq` 有点繁琐，不是吗？
You had to manually compare each field of the struct.
你必须手动比较结构体的每个字段。

## Destructuring syntax
## 解构语法

Furthermore, the implementation is brittle: if the struct definition changes
(e.g. a new field is added), you have to remember to update the `PartialEq` implementation.
此外，实现是脆弱的：如果结构体定义发生变化（例如添加了新字段），你必须记得更新 `PartialEq` 实现。

You can mitigate the risk by **destructuring** the struct into its fields:
你可以通过将结构体**解构**为其字段来降低风险：

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        // [...]
    }
}
```

If the definition of `Ticket` changes, the compiler will error out, complaining that your
destructuring is no longer exhaustive.\
如果 `Ticket` 的定义发生变化，编译器会报错，提示你的解构不再详尽。\
You can also rename struct fields, to avoid variable shadowing:
你也可以重命名结构体字段，以避免变量遮蔽：

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        let Ticket {
            title: other_title,
            description: other_description,
            status: other_status,
        } = other;
        // [...]
    }
}
```

Destructuring is a useful pattern to have in your toolkit, but
there's an even more convenient way to do this: **derive macros**.
解构是一个有用的模式，可以放在你的工具箱中，但有一种更方便的方法：**派生宏**。

## Macros
## 宏

You've already encountered a few macros in past exercises:
你在过去的练习中已经遇到过一些宏：

- `assert_eq!` and `assert!`, in the test cases
- `assert_eq!` 和 `assert!`，在测试用例中
- `println!`, to print to the console
- `println!`，用于打印到控制台

Rust macros are **code generators**.\
Rust 宏是**代码生成器**。\
They generate new Rust code based on the input you provide, and that generated code is then compiled alongside
the rest of your program. Some macros are built into Rust's standard library, but you can also
write your own. We won't be creating our own macro in this course, but you can find some useful
pointers in the ["Further reading" section](#further-reading).
它们根据你提供的输入生成新的 Rust 代码，然后生成的代码与程序的其余部分一起编译。一些宏内置于 Rust 的标准库中，但你也可以编写自己的宏。在本课程中，我们不会创建自己的宏，但你可以在["延伸阅读"部分](#further-reading)找到一些有用的指导。

### Inspection
### 检查

Some IDEs let you expand a macro to inspect the generated code. If that's not possible, you can use
[`cargo-expand`](https://github.com/dtolnay/cargo-expand).
某些 IDE 允许你展开宏来检查生成的代码。如果无法做到，你可以使用 [`cargo-expand`](https://github.com/dtolnay/cargo-expand)。

### Derive macros
### 派生宏

A **derive macro** is a particular flavour of Rust macro. It is specified as an **attribute** on top of a struct.
**派生宏**是 Rust 宏的一种特定形式。它被指定为结构体顶部的**属性**。

```rust
#[derive(PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: String
}
```

Derive macros are used to automate the implementation of common (and "obvious") traits for custom types.
派生宏用于自动化为自定义类型实现常见（且"明显"）的特征。
In the example above, the `PartialEq` trait is automatically implemented for `Ticket`.
在上面的例子中，`PartialEq` 特征自动为 `Ticket` 实现。
If you expand the macro, you'll see that the generated code is functionally equivalent to the one you wrote manually,
although a bit more cumbersome to read:
如果你展开宏，你会看到生成的代码在功能上与你手动编写的代码等效，尽管阅读起来有点繁琐：

```rust
#[automatically_derived]
impl ::core::cmp::PartialEq for Ticket {
    #[inline]
    fn eq(&self, other: &Ticket) -> bool {
        self.title == other.title 
            && self.description == other.description
            && self.status == other.status
    }
}
```

The compiler will nudge you to derive traits when possible.
编译器会在可能的时候提示你派生特征。

## Further reading
## 延伸阅读

- [The little book of Rust macros](https://veykril.github.io/tlborm/)
- [Rust 宏小册子](https://veykril.github.io/tlborm/)
- [Proc macro workshop](https://github.com/dtolnay/proc-macro-workshop)
- [过程宏工作坊](https://github.com/dtolnay/proc-macro-workshop)
