# Modules
# 模块

The `new` method you've just defined is trying to enforce some **constraints** on the field values for `Ticket`.
你刚刚定义的 `new` 方法试图对 `Ticket` 的字段值施加一些**约束**。
But are those invariants really enforced? What prevents a developer from creating a `Ticket`
without going through `Ticket::new`?
但是这些不变量真的被强制执行了吗？是什么阻止了开发者不通过 `Ticket::new` 来创建 `Ticket`？

To get proper **encapsulation** you need to become familiar with two new concepts: **visibility** and **modules**.
要获得适当的**封装**，你需要熟悉两个新概念：**可见性**和**模块**。
Let's start with modules.
让我们从模块开始。

## What is a module?
## 什么是模块？

In Rust a **module** is a way to group related code together, under a common namespace (i.e. the module's name).\
在 Rust 中，**模块**是一种将相关代码分组到共同命名空间（即模块名）下的方式。\
You've already seen modules in action: the unit tests that verify the correctness of your code are defined in a
different module, named `tests`.
你已经看到模块在起作用：验证代码正确性的单元测试定义在一个名为 `tests` 的不同模块中。

```rust
#[cfg(test)]
mod tests {
    // [...]
}
```

## Inline modules
## 内联模块

The `tests` module above is an example of an **inline module**: the module declaration (`mod tests`) and the module
contents (the stuff inside `{ ... }`) are next to each other.
上面的 `tests` 模块是一个**内联模块**的例子：模块声明（`mod tests`）和模块内容（`{ ... }` 内部的内容）是相邻的。

## Module tree
## 模块树

Modules can be nested, forming a **tree** structure.\
模块可以嵌套，形成**树**结构。\
The root of the tree is the **crate** itself, which is the top-level module that contains all the other modules.
树的根是 **crate** 本身，它是包含所有其他模块的顶级模块。
For a library crate, the root module is usually `src/lib.rs` (unless its location has been customized).
对于库 crate，根模块通常是 `src/lib.rs`（除非其位置已被自定义）。
The root module is also known as the **crate root**.
根模块也称为 **crate 根**。

The crate root can have submodules, which in turn can have their own submodules, and so on.
crate 根可以有子模块，子模块又可以有它们自己的子模块，依此类推。

## External modules and the filesystem
## 外部模块和文件系统

Inline modules are useful for small pieces of code, but as your project grows you'll want to split your code into
multiple files. In the parent module, you declare the existence of a submodule using the `mod` keyword.
内联模块适用于小段代码，但随着项目增长，你会想将代码拆分为多个文件。在父模块中，你使用 `mod` 关键字声明子模块的存在。

```rust
mod dog;
```

`cargo`, Rust's build tool, is then in charge of finding the file that contains
the module implementation.\
`cargo`（Rust 的构建工具）负责查找包含模块实现的文件。\
If your module is declared in the root of your crate (e.g. `src/lib.rs` or `src/main.rs`),
`cargo` expects the file to be named either:
如果你的模块在 crate 的根目录中声明（例如 `src/lib.rs` 或 `src/main.rs`），`cargo` 期望文件命名为以下之一：

- `src/<module_name>.rs`
- `src/<module_name>/mod.rs`

If your module is a submodule of another module, the file should be named:
如果你的模块是另一个模块的子模块，文件应命名为：

- `[..]/<parent_module>/<module_name>.rs`
- `[..]/<parent_module>/<module_name>/mod.rs`

E.g. `src/animals/dog.rs` or `src/animals/dog/mod.rs` if `dog` is a submodule of `animals`.
例如，如果 `dog` 是 `animals` 的子模块，则文件为 `src/animals/dog.rs` 或 `src/animals/dog/mod.rs`。

Your IDE might help you create these files automatically when you declare a new module using the `mod` keyword.
当你使用 `mod` 关键字声明新模块时，你的 IDE 可能会自动帮助你创建这些文件。

## Item paths and `use` statements
## 项目路径和 `use` 语句

You can access items defined in the same module without any special syntax. You just use their name.
你可以访问同一模块中定义的项目，无需特殊语法。只需使用它们的名称即可。

```rust
struct Ticket {
    // [...]
}

// No need to qualify `Ticket` in any way here
// because we're in the same module
fn mark_ticket_as_done(ticket: Ticket) {
    // [...]
}
```

That's not the case if you want to access an entity from a different module.\
如果你想访问来自不同模块的实体，情况就不同了。\
You have to use a **path** pointing to the entity you want to access.
你必须使用指向要访问的实体的**路径**。

You can compose the path in various ways:
你可以通过多种方式组合路径：

- starting from the root of the current crate, e.g. `crate::module_1::MyStruct`
- 从当前 crate 的根开始，例如 `crate::module_1::MyStruct`
- starting from the parent module, e.g. `super::my_function`
- 从父模块开始，例如 `super::my_function`
- starting from the current module, e.g. `sub_module_1::MyStruct`
- 从当前模块开始，例如 `sub_module_1::MyStruct`

Both `crate` and `super` are **keywords**.\
`crate` 和 `super` 都是**关键字**。\
`crate` refers to the root of the current crate, while `super` refers to the parent of the current module.
`crate` 指的是当前 crate 的根，而 `super` 指的是当前模块的父级。

Having to write the full path every time you want to refer to a type can be cumbersome.
每次想要引用类型时都要写完整路径可能很繁琐。
To make your life easier, you can introduce a `use` statement to bring the entity into scope.
为了让你的生活更轻松，你可以引入 `use` 语句将实体引入作用域。

```rust
// Bring `MyStruct` into scope
use crate::module_1::module_2::MyStruct;

// Now you can refer to `MyStruct` directly
fn a_function(s: MyStruct) {
     // [...]
}
```

### Star imports
### 星号导入

You can also import all the items from a module with a single `use` statement.
你也可以通过一个 `use` 语句导入模块中的所有项目。

```rust
use crate::module_1::module_2::*;
```

This is known as a **star import**.\
这被称为**星号导入**。\
It is generally discouraged because it can pollute the current namespace, making it hard to understand
where each name comes from and potentially introducing name conflicts.\
通常不鼓励这样做，因为它可能污染当前的命名空间，使理解每个名称的来源变得困难，并可能引入名称冲突。\
Nonetheless, it can be useful in some cases, like when writing unit tests. You might have noticed
that most of our test modules start with a `use super::*;` statement to bring all the items from the parent module
(the one being tested) into scope.
尽管如此，它在某些情况下还是很有用的，比如在编写单元测试时。你可能已经注意到，我们的大多数测试模块都以 `use super::*;` 语句开头，将父模块（被测试的模块）中的所有项目引入作用域。

## Visualizing the module tree
## 可视化模块树

If you're struggling to picture the module tree of your project, you can try using
[`cargo-modules`](https://crates.io/crates/cargo-modules) to visualize it!
如果你难以想象项目的模块树，可以尝试使用 [`cargo-modules`](https://crates.io/crates/cargo-modules) 来可视化它！

Refer to their documentation for installation instructions and usage examples.
请参考他们的文档了解安装说明和使用示例。
