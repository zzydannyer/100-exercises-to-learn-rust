# Conversions, pt. 1
# 转换，第一部分

We've repeated over and over again that Rust won't perform
implicit type conversions for integers.\
我们一再强调，Rust 不会对整数执行隐式类型转换。\
How do you perform _explicit_ conversions then?
那么如何执行_显式_转换呢？

## `as`
## `as`

You can use the `as` operator to convert between integer types.\
你可以使用 `as` 运算符在整数类型之间进行转换。\
`as` conversions are **infallible**.
`as` 转换是**不可失败的**。

For example:
例如：

```rust
let a: u32 = 10;

// Cast `a` into the `u64` type
let b = a as u64;

// You can use `_` as the target type
// if it can be correctly inferred 
// by the compiler. For example:
let c: u64 = a as _;
```

The semantics of this conversion are what you expect: all `u32` values are valid `u64`
values.
这种转换的语义正如你所期望的：所有 `u32` 值都是有效的 `u64` 值。

### Truncation
### 截断

Things get more interesting if we go in the opposite direction:
如果我们往相反的方向走，事情就变得更有趣了：

```rust
// A number that's too big 
// to fit into a `u8`
let a: u16 = 255 + 1;
let b = a as u8;
```

This program will run without issues, because `as` conversions are infallible.
这个程序将正常运行，因为 `as` 转换是不可失败的。
But what is the value of `b`?
但是 `b` 的值是多少呢？
When going from a larger integer type to a smaller, the Rust compiler will perform
a **truncation**.
当从较大的整数类型转到较小的整数类型时，Rust 编译器会执行**截断(truncation)**。

To understand what happens, let's start by looking at how `256u16` is
represented in memory, as a sequence of bits:
为了理解发生了什么，让我们先看看 `256u16` 在内存中是如何表示的，即一系列比特：

```text
 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0
|               |               |
+---------------+---------------+
  First 8 bits    Last 8 bits
```

When converting to a `u8`, the Rust compiler will keep the last 8 bits of a `u16`
memory representation:
当转换为 `u8` 时，Rust 编译器会保留 `u16` 内存表示的最后 8 位：

```text
 0 0 0 0 0 0 0 0 
|               |
+---------------+
  Last 8 bits
```

Hence `256 as u8` is equal to `0`. That's... not ideal, in most scenarios.\
因此 `256 as u8` 等于 `0`。这在大多数场景中...并不理想。\
In fact, the Rust compiler will actively try to stop you if it sees you trying
to cast a literal value which will result in a truncation:
事实上，如果 Rust 编译器发现你试图转换一个会导致截断的字面量值，它会主动阻止你：

```text
error: literal out of range for `i8`
  |
4 |     let a = 255 as i8;
  |             ^^^
  |
  = note: the literal `255` does not fit into the type `i8` 
          whose range is `-128..=127`
  = help: consider using the type `u8` instead
  = note: `#[deny(overflowing_literals)]` on by default
```

### Recommendation
### 建议

As a rule of thumb, be quite careful with `as` casting.\
作为经验法则，使用 `as` 转换时要非常小心。\
Use it _exclusively_ for going from a smaller type to a larger type.
_仅仅_在从较小类型转到较大类型时使用它。
To convert from a larger to smaller integer type, rely on the
[_fallible_ conversion machinery](../05_ticket_v2/13_try_from.md) that we'll
explore later in the course.
要从较大的整数类型转换为较小的整数类型，请依赖我们将在课程后面探索的[_可失败的_ 转换机制](../05_ticket_v2/13_try_from.md)。

### Limitations
### 局限性

Surprising behaviour is not the only downside of `as` casting.
令人惊讶的行为并不是 `as` 转换的唯一缺点。
It is also fairly limited: you can only rely on `as` casting
for primitive types and a few other special cases.\
它也有相当大的局限性：你只能在原始类型和其他一些特殊情况下依赖 `as` 转换。\
When working with composite types, you'll have to rely on
different conversion mechanisms ([fallible](../05_ticket_v2/13_try_from.md)
and [infallible](../04_traits/09_from.md)), which we'll explore later on.
在处理复合类型时，你必须依赖不同的转换机制（[可失败的](../05_ticket_v2/13_try_from.md)和[不可失败的](../04_traits/09_from.md)），我们将在后面探索这些。

## Further reading
## 扩展阅读

- Check out [Rust's official reference](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast)
  to learn the precise behaviour of `as` casting for each source/target combination,
  as well as the exhaustive list of allowed conversions.
- 查看 [Rust 官方参考](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast) 了解 `as` 转换在每个源/目标组合上的精确行为，以及允许的转换的详尽列表。
