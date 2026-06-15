# Libraries and binaries
# 库和二进制文件

It took a bit of code to implement the `Error` trait for `TicketNewError`, didn't it?\
为 `TicketNewError` 实现 `Error` 特征花了不少代码，不是吗？\
A manual `Display` implementation, plus an `Error` impl block.
手动实现 `Display`，再加上一个 `Error` 实现块。

We can remove some of the boilerplate by using [`thiserror`](https://docs.rs/thiserror/latest/thiserror/),
a Rust crate that provides a **procedural macro** to simplify the creation of custom error types.\
我们可以通过使用 [`thiserror`](https://docs.rs/thiserror/latest/thiserror/) 来减少一些样板代码，这是一个 Rust crate，它提供了一个**过程宏**来简化自定义错误类型的创建。\
But we're getting ahead of ourselves: `thiserror` is a third-party crate, it'd be our first dependency!
但我们有点超前了：`thiserror` 是一个第三方 crate，它将是我们的第一个依赖项！

Let's take a step back to talk about Rust's packaging system before we dive into dependencies.
让我们退一步，在深入讨论依赖项之前，先谈谈 Rust 的打包系统。

## What is a package?
## 什么是包？

A Rust package is defined by the `[package]` section in a `Cargo.toml` file, also known as its **manifest**.
Within `[package]` you can set the package's metadata, such as its name and version.
Rust 包由 `Cargo.toml` 文件中的 `[package]` 部分定义，也称为其**清单**。在 `[package]` 中，你可以设置包的元数据，例如其名称和版本。

Go check the `Cargo.toml` file in the directory of this section's exercise!
去查看本小节练习目录中的 `Cargo.toml` 文件吧！

## What is a crate?
## 什么是 crate？

Inside a package, you can have one or more **crates**, also known as **targets**.\
在一个包内，你可以有一个或多个 **crate**，也称为**目标**。\
The two most common crate types are **binary crates** and **library crates**.
两种最常见的 crate 类型是**二进制 crate** 和**库 crate**。

### Binaries
### 二进制文件

A binary is a program that can be compiled to an **executable file**.\
二进制文件是可以编译为**可执行文件**的程序。\
It must include a function named `main`—the program's entry point. `main` is invoked when the program is executed.
它必须包含一个名为 `main` 的函数——程序的入口点。`main` 在程序执行时被调用。

### Libraries
### 库

Libraries, on the other hand, are not executable on their own. You can't _run_ a library,
but you can _import its code_ from another package that depends on it.\
另一方面，库本身是不可执行的。你不能_运行_一个库，但你可以从依赖它的另一个包中_导入它的代码_。\
A library groups together code (i.e. functions, types, etc.) that can be leveraged by other packages as a **dependency**.
库将代码（即函数、类型等）组合在一起，其他包可以将其作为**依赖项**来使用。

All the exercises you've solved so far have been structured as libraries, with a test suite attached to them.
到目前为止，你解决的所有练习都结构化为库，并附带了测试套件。

### Conventions
### 约定

There are some conventions around Rust packages that you need to keep in mind:
关于 Rust 包，有一些你需要记住的约定：

- The package's source code is usually located in the `src` directory.
- 包的源代码通常位于 `src` 目录中。
- If there's a `src/lib.rs` file, `cargo` will infer that the package contains a library crate.
- 如果有 `src/lib.rs` 文件，`cargo` 会推断该包包含一个库 crate。
- If there's a `src/main.rs` file, `cargo` will infer that the package contains a binary crate.
- 如果有 `src/main.rs` 文件，`cargo` 会推断该包包含一个二进制 crate。

You can override these defaults by explicitly declaring your targets in the `Cargo.toml` file—see
[`cargo`'s documentation](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#cargo-targets) for more details.
你可以通过在 `Cargo.toml` 文件中显式声明你的目标来覆盖这些默认设置——更多细节请参阅 [`cargo` 的文档](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#cargo-targets)。

Keep in mind that while a package can contain multiple crates, it can only contain one library crate.
请记住，虽然一个包可以包含多个 crate，但它只能包含一个库 crate。
