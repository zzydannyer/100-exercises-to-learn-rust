# Loops, part 2: `for`
# 循环，第二部分：`for`

Having to manually increment a counter variable is somewhat tedious. The pattern is also extremely common!\
手动递增计数器变量有点繁琐。而且这种模式也非常常见！\
To make this easier, Rust provides a more concise way to iterate over a range of values: the `for` loop.
为了让这更简单，Rust 提供了一种更简洁的迭代值范围的方法：`for` 循环。

## The `for` loop
## `for` 循环

A `for` loop is a way to execute a block of code for each element in an iterator[^iterator].
`for` 循环是一种为迭代器[^iterator]中的每个元素执行代码块的方法。

Here's the general syntax:
以下是通用语法：

```rust
for <element> in <iterator> {
    // code to execute
}
```

## Ranges
## 范围

Rust's standard library provides **range** type that can be used to iterate over a sequence of numbers[^weird-ranges].
Rust 的标准库提供了**范围(range)**类型，可用于迭代数字序列[^weird-ranges]。

For example, if we want to sum the numbers from 1 to 5:
例如，如果我们想对从 1 到 5 的数字求和：

```rust
let mut sum = 0;
for i in 1..=5 {
    sum += i;
}
```

Every time the loop runs, `i` will be assigned the next value in the range before executing the block of code.
每次循环运行时，`i` 将被赋值为范围内的下一个值，然后执行代码块。

There are five kinds of ranges in Rust:
Rust 中有五种范围：

- `1..5`: A (half-open) range. It includes all numbers from 1 to 4. It doesn't include the last value, 5.
- `1..5`：（半开）范围。包含从 1 到 4 的所有数字。不包含最后一个值 5。
- `1..=5`: An inclusive range. It includes all numbers from 1 to 5. It includes the last value, 5.
- `1..=5`：包含范围。包含从 1 到 5 的所有数字。包含最后一个值 5。
- `1..`: An open-ended range. It includes all numbers from 1 to infinity (well, until the maximum value of the integer type).
- `1..`：无上限范围。包含从 1 到无穷大（好吧，直到整数类型的最大值）的所有数字。
- `..5`: A range that starts at the minimum value for the integer type and ends at 4. It doesn't include the last value, 5.
- `..5`：从整数类型的最小值开始到 4 结束的范围。不包含最后一个值 5。
- `..=5`: A range that starts at the minimum value for the integer type and ends at 5. It includes the last value, 5.
- `..=5`：从整数类型的最小值开始到 5 结束的范围。包含最后一个值 5。

You can use a `for` loop with the first three kinds of ranges, where the starting point
is explicitly specified. The last two range types are used in other contexts, that we'll cover later.
你可以将 `for` 循环用于前三种范围，其中起始点是显式指定的。后两种范围类型用于其他上下文，我们将在后面介绍。

The extreme values of a range don't have to be integer literals—they can be variables or expressions too!
范围的极值不一定是整数字面量——它们也可以是变量或表达式！

For example:
例如：

```rust
let end = 5;
let mut sum = 0;

for i in 1..(end + 1) {
    sum += i;
}
```

## Further reading
## 扩展阅读

- [`for` loop documentation](https://doc.rust-lang.org/std/keyword.for.html)
- [`for` 循环文档](https://doc.rust-lang.org/std/keyword.for.html)

[^iterator]: Later in the course we'll give a precise definition of what counts as an "iterator".
For now, think of it as a sequence of values that you can loop over.
[^iterator]: 在课程后面，我们会给出"迭代器"的精确定义。目前，可以把它看作一个你可以循环遍历的值序列。
[^weird-ranges]: You can use ranges with other types too (e.g. characters and IP addresses),
but integers are definitely the most common case in day-to-day Rust programming.
[^weird-ranges]: 你也可以将范围用于其他类型（例如字符和 IP 地址），但整数无疑是日常 Rust 编程中最常见的情况。
