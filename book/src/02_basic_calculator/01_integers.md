# Types, part 1
# 类型，第一部分

In the ["Syntax" section](../01_intro/01_syntax.md) `compute`'s input parameters were of type `u32`.\
在["语法"部分](../01_intro/01_syntax.md)中，`compute` 的输入参数是 `u32` 类型。\
Let's unpack what that _means_.
让我们深入解析一下这_意味着什么_。

## Primitive types
## 原始类型

`u32` is one of Rust's **primitive types**. Primitive types are the most basic building blocks of a language.
`u32` 是 Rust 的**原始类型**之一。原始类型是语言最基本的构建块。
They're built into the language itself—i.e. they are not defined in terms of other types.
它们内建于语言本身——也就是说，它们不是由其他类型定义的。

You can combine these primitive types to create more complex types. We'll see how soon enough.
你可以组合这些原始类型来创建更复杂的类型。我们很快就会看到如何操作。

## Integers
## 整数

`u32`, in particular, is an **unsigned 32-bit integer**.
具体来说，`u32` 是一个**无符号 32 位整数**。

An integer is a number that can be written without a fractional component. E.g. `1` is an integer, while `1.2` is not.
整数是可以不带小数部分书写的数字。例如，`1` 是整数，而 `1.2` 不是。

### Signed vs. unsigned
### 有符号 vs 无符号

An integer can be **signed** or **unsigned**.\
整数可以是**有符号**的或**无符号**的。\
An unsigned integer can only represent non-negative numbers (i.e. `0` or greater).
无符号整数只能表示非负数（即 `0` 或更大）。
A signed integer can represent both positive and negative numbers (e.g. `-1`, `12`, etc.).
有符号整数可以表示正数和负数（例如 `-1`、`12` 等）。

The `u` in `u32` stands for **unsigned**.\
`u32` 中的 `u` 代表**无符号(unsigned)**。\
The equivalent type for signed integer is `i32`, where the `i` stands for integer (i.e. any integer, positive or
negative).
相应的有符号整数类型是 `i32`，其中 `i` 代表整数（即任何整数，正数或负数）。

### Bit width
### 位宽

The `32` in `u32` refers to the **number of bits[^bit]** used to represent the number in memory.\
`u32` 中的 `32` 指的是用于在内存中表示该数字的**比特数[^bit]**。\
The more bits, the larger the range of numbers that can be represented.
比特数越多，可以表示的数字范围就越大。

Rust supports multiple bit widths for integers: `8`, `16`, `32`, `64`, `128`.
Rust 支持多种位宽的整数：`8`、`16`、`32`、`64`、`128`。

With 32 bits, `u32` can represent numbers from `0` to `2^32 - 1` (a.k.a. [`u32::MAX`](https://doc.rust-lang.org/std/primitive.u32.html#associatedconstant.MAX)).\
使用 32 位，`u32` 可以表示从 `0` 到 `2^32 - 1`（即 [`u32::MAX`](https://doc.rust-lang.org/std/primitive.u32.html#associatedconstant.MAX)）的数字。\
With the same number of bits, a signed integer (`i32`) can represent numbers from `-2^31` to `2^31 - 1`
(i.e. from [`i32::MIN`](https://doc.rust-lang.org/std/primitive.i32.html#associatedconstant.MIN)
to [`i32::MAX`](https://doc.rust-lang.org/std/primitive.i32.html#associatedconstant.MAX)).\
使用相同的位数，有符号整数（`i32`）可以表示从 `-2^31` 到 `2^31 - 1`（即从 [`i32::MIN`](https://doc.rust-lang.org/std/primitive.i32.html#associatedconstant.MIN) 到 [`i32::MAX`](https://doc.rust-lang.org/std/primitive.i32.html#associatedconstant.MAX)）的数字。\
The maximum value for `i32` is smaller than the maximum value for `u32` because one bit is used to represent
the sign of the number. Check out the [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement)
representation for more details on how signed integers are represented in memory.
`i32` 的最大值小于 `u32` 的最大值，因为一个位用于表示数字的符号。请查看[二进制补码](https://en.wikipedia.org/wiki/Two%27s_complement)表示法，了解更多关于有符号整数如何在内存中表示的详细信息。

### Summary
### 总结

Combining the two variables (signed/unsigned and bit width), we get the following integer types:
结合两个变量（有符号/无符号和位宽），我们得到以下整数类型：

| Bit width | Signed | Unsigned |
| --------- | ------ | -------- |
| 8-bit     | `i8`   | `u8`     |
| 16-bit    | `i16`  | `u16`    |
| 32-bit    | `i32`  | `u32`    |
| 64-bit    | `i64`  | `u64`    |
| 128-bit   | `i128` | `u128`   |

## Literals
## 字面量

A **literal** is a notation for representing a fixed value in source code.\
**字面量**是用于在源代码中表示固定值的记法。\
For example, `42` is a Rust literal for the number forty-two.
例如，`42` 是数字四十二的 Rust 字面量。

### Type annotations for literals
### 字面量的类型标注

But all values in Rust have a type, so... what's the type of `42`?
但是 Rust 中所有值都有类型，那么... `42` 的类型是什么？

The Rust compiler will try to infer the type of a literal based on how it's used.\
Rust 编译器会尝试根据字面量的使用方式来推断其类型。\
If you don't provide any context, the compiler will default to `i32` for integer literals.\
如果你不提供任何上下文，编译器会默认整数类型字面量为 `i32`。\
If you want to use a different type, you can add the desired integer type as a suffix—e.g. `2u64` is a 2 that's
explicitly typed as a `u64`.
如果你想使用不同的类型，你可以将目标整数类型作为后缀添加——例如，`2u64` 是一个显式类型化为 `u64` 的 2。

### Underscores in literals
### 字面量中的下划线

You can use underscores `_` to improve the readability of large numbers.\
你可以使用下划线 `_` 来提高大数字的可读性。\
For example, `1_000_000` is the same as `1000000`.
例如，`1_000_000` 等同于 `1000000`。

## Arithmetic operators
## 算术运算符

Rust supports the following arithmetic operators[^traits] for integers:
Rust 为整数支持以下算术运算符[^traits]：

- `+` for addition
- `+` 加法
- `-` for subtraction
- `-` 减法
- `*` for multiplication
- `*` 乘法
- `/` for division
- `/` 除法
- `%` for remainder
- `%` 取余

Precedence and associativity rules for these operators are the same as in mathematics.\
这些运算符的优先级和结合性规则与数学中相同。\
You can use parentheses to override the default precedence. E.g. `2 * (3 + 4)`.
你可以使用括号来覆盖默认优先级。例如，`2 * (3 + 4)`。

> ⚠️ **Warning**
>
> ⚠️ **警告**
>
> The division operator `/` performs integer division when used with integer types.
> 除法运算符 `/` 在与整数类型一起使用时执行整数除法。
> I.e. the result is truncated towards zero. For example, `5 / 2` is `2`, not `2.5`.
> 也就是说，结果向零截断。例如，`5 / 2` 是 `2`，而不是 `2.5`。

## No automatic type coercion
## 无自动类型转换

As we discussed in the previous exercise, Rust is a statically typed language.\
正如我们在上一个练习中讨论的，Rust 是一门静态类型语言。\
In particular, Rust is quite strict about type coercion. It won't automatically convert a value from one type to
another[^coercion],
even if the conversion is lossless. You have to do it explicitly.
特别是，Rust 在类型转换方面相当严格。即使转换是无损的，它也不会自动将值从一种类型转换为另一种类型[^coercion]。你必须显式地进行转换。

For example, you can't assign a `u8` value to a variable with type `u32`, even though all `u8` values are valid `u32`
values:
例如，你不能将 `u8` 值赋给类型为 `u32` 的变量，即使所有 `u8` 值都是有效的 `u32` 值：

```rust
let b: u8 = 100;
let a: u32 = b;
```

It'll throw a compilation error:
它会抛出一个编译错误：

```text
error[E0308]: mismatched types
  |
3 |     let a: u32 = b;
  |            ---   ^ expected `u32`, found `u8`
  |            |
  |            expected due to this
  |
```

We'll see how to convert between types [later in this course](../04_traits/09_from.md).
我们将在[本课程后面](../04_traits/09_from.md)学习如何在类型之间进行转换。

## Further reading
## 扩展阅读

- [The integer types section](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types) in the official Rust book
- [Rust 官方书籍中的整数类型章节](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types)

[^bit]: A bit is the smallest unit of data in a computer. It can only have two values: `0` or `1`.
[^bit]: 比特是计算机中最小的数据单位。它只能有两个值：`0` 或 `1`。

[^traits]: Rust doesn't let you define custom operators, but it puts you in control of how the built-in operators behave.
[^traits]: Rust 不允许你定义自定义运算符，但它让你控制内置运算符的行为。
We'll talk about operator overloading [later in the course](../04_traits/03_operator_overloading.md), after we've covered traits.
我们将在[课程后面](../04_traits/03_operator_overloading.md)讨论运算符重载，在学习了特征之后。

[^coercion]: There are some exceptions to this rule, mostly related to references, smart pointers and ergonomics. We'll cover those [later on](../04_traits/07_deref.md).
[^coercion]: 这个规则存在一些例外，主要涉及引用、智能指针和人体工程学。我们将在[后面](../04_traits/07_deref.md)介绍这些。
A mental model of "all conversions are explicit" will serve you well in the meantime.
"所有转换都是显式的"这种思维模型在目前对你很有帮助。
