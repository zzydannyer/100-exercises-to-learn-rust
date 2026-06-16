# Control flow, part 1
# 控制流，第一部分

All our programs so far have been pretty straightforward.\
到目前为止，我们所有的程序都非常直接。\
A sequence of instructions is executed from top to bottom, and that's it.
一系列指令从上到下执行，就是这样。

It's time to introduce some **branching**.
是时候介绍一些**分支**了。

## `if` clauses
## `if` 子句

The `if` keyword is used to execute a block of code only if a condition is true.
`if` 关键字用于仅在条件为真时执行一段代码。

Here's a simple example:
这是一个简单的例子：

```rust
let number = 3;
if number < 5 {
    println!("`number` is smaller than 5");
}
```

This program will print `number is smaller than 5` because the condition `number < 5` is true.
这个程序会输出 `number is smaller than 5`，因为条件 `number < 5` 为真。

### `else` clauses
### `else` 子句

Like most programming languages, Rust supports an optional `else` branch to execute a block of code when the condition in an
`if` expression is false.\
与大多数编程语言一样，Rust 支持可选的 `else` 分支，用于在 `if` 表达式中的条件为假时执行一段代码。\
For example:
例如：

```rust
let number = 3;

if number < 5 {
    println!("`number` is smaller than 5");
} else {
    println!("`number` is greater than or equal to 5");
}
```

### `else if` clauses
### `else if` 子句

Your code drifts more and more to the right when you have multiple `if` expressions, one nested inside the other.
当你有多个 `if` 表达式一个嵌套在另一个中时，你的代码会越来越向右偏移。

```rust
let number = 3;

if number < 5 {
    println!("`number` is smaller than 5");
} else {
    if number < 8 {
        println!("`number` is greater than or equal to 5, but smaller than 8");
    } else {
        println!("`number` is greater than or equal to 8");
    }
}
```

You can use the `else if` keyword to combine multiple `if` expressions into a single one:
你可以使用 `else if` 关键字将多个 `if` 表达式组合成一个：

```rust
let number = 3;

if number < 5 {
    println!("`number` is smaller than 5");
} else if number < 8 {
    println!("`number` is greater than or equal to 5, but smaller than 8");
} else {
    println!("`number` is greater than or equal to 8");
}
```

## Booleans
## 布尔值

The condition in an `if` expression must be of type `bool`, a **boolean**.\
`if` 表达式中的条件必须是 `bool` 类型，即**布尔值**。\
Booleans, just like integers, are a primitive type in Rust.
布尔值，就像整数一样，是 Rust 中的原始类型。

A boolean can have one of two values: `true` or `false`.
布尔值只能有两个值之一：`true` 或 `false`。

### No truthy or falsy values
### 没有真值或假值

If the condition in an `if` expression is not a boolean, you'll get a compilation error.
如果 `if` 表达式中的条件不是布尔值，你会得到一个编译错误。

For example, the following code will not compile:
例如，以下代码将无法编译：

```rust
let number = 3;
if number {
    println!("`number` is not zero");
}
```

You'll get the following compilation error:
你会得到以下编译错误：

```text
error[E0308]: mismatched types
 --> src/main.rs:3:8
  |
3 |     if number {
  |        ^^^^^^ expected `bool`, found integer
```

This follows from Rust's philosophy around type coercion: there's no automatic conversion from non-boolean types to booleans.
这遵循了 Rust 关于类型转换的理念：不存在从非布尔类型到布尔类型的自动转换。
Rust doesn't have the concept of **truthy** or **falsy** values, like JavaScript or Python.\
Rust 没有像 JavaScript 或 Python 那样的**真值(truthy)**或**假值(falsy)**概念。\
You have to be explicit about the condition you want to check.
你必须明确你要检查的条件。

### Comparison operators
### 比较运算符

It's quite common to use comparison operators to build conditions for `if` expressions.\
使用比较运算符构建 `if` 表达式的条件是非常常见的。\
Here are the comparison operators available in Rust when working with integers:
以下是 Rust 中在处理整数时可用的比较运算符：

- `==`: equal to
- `==`：等于
- `!=`: not equal to
- `!=`：不等于
- `<`: less than
- `<`：小于
- `>`: greater than
- `>`：大于
- `<=`: less than or equal to
- `<=`：小于或等于
- `>=`: greater than or equal to
- `>=`：大于或等于

## `if/else` is an expression
## `if/else` 是一个表达式

In Rust, `if` expressions are **expressions**, not statements: they return a value.\
在 Rust 中，`if` 表达式是**表达式**，而不是语句：它们会返回一个值。\
That value can be assigned to a variable or used in other expressions. For example:
该值可以赋给变量或在其他表达式中使用。例如：

```rust
let number = 3;
let message = if number < 5 {
    "smaller than 5"
} else {
    "greater than or equal to 5"
};
```

In the example above, each branch of the `if` evaluates to a string literal,
which is then assigned to the `message` variable.\
在上面的例子中，`if` 的每个分支都求值为一个字符串字面量，然后被赋给 `message` 变量。\
The only requirement is that both `if` branches return the same type.
唯一的要求是 `if` 的两个分支返回相同的类型。
