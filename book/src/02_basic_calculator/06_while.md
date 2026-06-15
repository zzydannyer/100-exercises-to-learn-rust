# Loops, part 1: `while`
# 循环，第一部分：`while`

Your implementation of `factorial` has been forced to use recursion.\
你的 `factorial` 实现一直被迫使用递归。\
This may feel natural to you, especially if you're coming from a functional programming background.
这对你来说可能感觉很自然，特别是如果你来自函数式编程背景。
Or it may feel strange, if you're used to more imperative languages like C or Python.
或者如果你习惯了像 C 或 Python 这样的命令式语言，这可能会感觉很奇怪。

Let's see how you can implement the same functionality using a **loop** instead.
让我们看看如何用**循环**来实现同样的功能。

## The `while` loop
## `while` 循环

A `while` loop is a way to execute a block of code as long as a **condition** is true.\
`while` 循环是一种只要**条件**为真就执行代码块的方法。\
Here's the general syntax:
以下是通用语法：

```rust
while <condition> {
    // code to execute
}
```

For example, we might want to sum the numbers from 1 to 5:
例如，我们可能想要对从 1 到 5 的数字求和：

```rust
let sum = 0;
let i = 1;
// "while i is less than or equal to 5"
while i <= 5 {
    // `+=` is a shorthand for `sum = sum + i`
    sum += i;
    i += 1;
}
```

This will keep adding 1 to `i` and `i` to `sum` until `i` is no longer less than or equal to 5.
这将不断给 `i` 加 1，并将 `i` 加到 `sum`，直到 `i` 不再小于等于 5。

## The `mut` keyword
## `mut` 关键字

The example above won't compile as is. You'll get an error like:
上面的例子不能直接编译。你会得到这样的错误：

```text
error[E0384]: cannot assign twice to immutable variable `sum`
 --> src/main.rs:7:9
  |
2 |     let sum = 0;
  |         ---
  |         |
  |         first assignment to `sum`
  |         help: consider making this binding mutable: `mut sum`
...
7 |         sum += i;
  |         ^^^^^^^^ cannot assign twice to immutable variable

error[E0384]: cannot assign twice to immutable variable `i`
 --> src/main.rs:8:9
  |
3 |     let i = 1;
  |         -
  |         |
  |         first assignment to `i`
  |         help: consider making this binding mutable: `mut i`
...
8 |         i += 1;
  |         ^^^^^^ cannot assign twice to immutable variable
```

This is because variables in Rust are **immutable** by default.\
这是因为 Rust 中的变量默认是**不可变的**。\
You can't change their value once it has been assigned.
一旦赋值，你就不能改变它们的值。

If you want to allow modifications, you have to declare the variable as **mutable** using the `mut` keyword:
如果你允许修改，必须使用 `mut` 关键字将变量声明为**可变的**：

```rust
// `sum` and `i` are mutable now!
let mut sum = 0;
let mut i = 1;

while i <= 5 {
    sum += i;
    i += 1;
}
```

This will compile and run without errors.
这将编译并运行无错误。

## Further reading
## 扩展阅读

- [`while` loop documentation](https://doc.rust-lang.org/std/keyword.while.html)
- [`while` 循环文档](https://doc.rust-lang.org/std/keyword.while.html)
