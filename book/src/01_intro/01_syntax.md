# Syntax
# 语法

<div class="warning">

Don't jump ahead!\
不要跳步！\
Complete the exercise for the previous section before you start this one.\
在开始本节之前，先完成上一节的练习。\
It's located in `exercises/01_intro/00_welcome`, in the [course GitHub's repository](https://github.com/mainmatter/100-exercises-to-learn-rust).\
练习位于 `exercises/01_intro/00_welcome`，在[课程 GitHub 仓库](https://github.com/mainmatter/100-exercises-to-learn-rust)中。\
Use [`wr`](00_welcome.md#wr-the-workshop-runner) to start the course and verify your solutions.
使用 [`wr`](00_welcome.md#wr-the-workshop-runner) 开始课程并验证你的解决方案。

</div>

The previous task doesn't even qualify as an exercise, but it already exposed you to quite a bit of Rust **syntax**.
上一个任务甚至算不上一个练习，但它已经让你接触到不少 Rust **语法**了。
We won't cover every single detail of Rust's syntax used in the previous exercise.
我们不会涵盖上一个练习中使用的所有 Rust 语法细节。
Instead, we'll cover _just enough_ to keep going without getting stuck in the details.\
相反，我们会讲解_刚好够用_的内容，让你能够继续前进而不被细节困住。\
One step at a time!
一步一个脚印！

## Comments
## 注释

You can use `//` for single-line comments:
你可以使用 `//` 进行单行注释：

```rust
// This is a single-line comment
// Followed by another single-line comment
```

## Functions
## 函数

Functions in Rust are defined using the `fn` keyword, followed by the function's name, its input parameters, and its
return type.
Rust 中的函数使用 `fn` 关键字定义，后跟函数名、输入参数和返回类型。
The function's body is enclosed in curly braces `{}`.
函数体用花括号 `{}` 括起来。

In previous exercise, you saw the `greeting` function:
在上一练习中，你看到了 `greeting` 函数：

```rust
// `fn` <function_name> ( <input params> ) -> <return_type> { <body> }
fn greeting() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to __!"
}
```

`greeting` has no input parameters and returns a reference to a string slice (`&'static str`).
`greeting` 没有输入参数，返回一个字符串切片的引用（`&'static str`）。

### Return type
### 返回类型

The return type can be omitted from the signature if the function doesn't return anything (i.e. if it returns `()`,
Rust's unit type).
如果函数不返回任何内容（即返回 `()`，Rust 的单位类型），则可以从签名中省略返回类型。
That's what happened with the `test_welcome` function:
这就是 `test_welcome` 函数的情况：

```rust
fn test_welcome() {
    assert_eq!(greeting(), "I'm ready to learn Rust!");
}
```

The above is equivalent to:
上述代码等价于：

```rust
// Spelling out the unit return type explicitly
//                   👇
fn test_welcome() -> () {
    assert_eq!(greeting(), "I'm ready to learn Rust!");
}
```

### Returning values
### 返回值

The last expression in a function is implicitly returned:
函数中的最后一个表达式会被隐式返回：

```rust
fn greeting() -> &'static str {
    // This is the last expression in the function
    // Therefore its value is returned by `greeting`
    "I'm ready to learn Rust!"
}
```

You can also use the `return` keyword to return a value early:
你也可以使用 `return` 关键字提前返回一个值：

```rust
fn greeting() -> &'static str {
    // Notice the semicolon at the end of the line!
    return "I'm ready to learn Rust!";
}
```

It is considered idiomatic to omit the `return` keyword when possible.
在可能的情况下省略 `return` 关键字被认为是惯用写法。

### Input parameters
### 输入参数

Input parameters are declared inside the parentheses `()` that follow the function's name.\
输入参数在函数名后的圆括号 `()` 内声明。\
Each parameter is declared with its name, followed by a colon `:`, followed by its type.
每个参数先声明其名称，后跟冒号 `:`，再跟其类型。

For example, the `greet` function below takes a `name` parameter of type `&str` (a "string slice"):
例如，下面的 `greet` 函数接受一个类型为 `&str`（"字符串切片"）的 `name` 参数：

```rust
// An input parameter
//        👇
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

If there are multiple input parameters, they must be separated with commas.
如果有多个输入参数，它们必须用逗号分隔。

### Type annotations
### 类型标注

Since we've been mentioned "types" a few times, let's state it clearly: Rust is a **statically typed language**.\
既然我们已经提到"类型"几次了，让我们明确说明：Rust 是一门**静态类型语言**。\
Every single value in Rust has a type and that type must be known to the compiler at compile-time.
Rust 中的每一个值都有一个类型，并且该类型必须在编译时被编译器知道。

Types are a form of **static analysis**.\
类型是一种**静态分析**的形式。\
You can think of a type as a **tag** that the compiler attaches to every value in your program. Depending on the
tag, the compiler can enforce different rules—e.g. you can't add a string to a number, but you can add two numbers
together.
你可以把类型想象成编译器附加到程序中每个值上的一个**标签**。根据标签的不同，编译器可以执行不同的规则——例如，你不能将字符串和数字相加，但你可以将两个数字相加。
If leveraged correctly, types can prevent whole classes of runtime bugs.
如果正确利用，类型可以防止整个类别的运行时错误。
