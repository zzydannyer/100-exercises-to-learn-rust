# Operator overloading
# 运算符重载

Now that we have a basic understanding of what traits are, let's circle back to **operator overloading**.
既然我们对特征有了基本的了解，让我们回到**运算符重载**。
Operator overloading is the ability to define custom behavior for operators like `+`, `-`, `*`, `/`, `==`, `!=`, etc.
运算符重载是为像 `+`、`-`、`*`、`/`、`==`、`!=` 等运算符定义自定义行为的能力。

## Operators are traits
## 运算符就是特征

In Rust, operators are traits.\
在 Rust 中，运算符就是特征。\
For each operator, there is a corresponding trait that defines the behavior of that operator.
By implementing that trait for your type, you **unlock** the usage of the corresponding operators.
对于每个运算符，都有一个相应的特征来定义该运算符的行为。
通过为你的类型实现该特征，你**解锁**了相应运算符的使用。

For example, the [`PartialEq` trait](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) defines the behavior of
the `==` and `!=` operators:
例如，[`PartialEq` 特征](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) 定义了 `==` 和 `!=` 运算符的行为：

```rust
// The `PartialEq` trait definition, from Rust's standard library
// (It is *slightly* simplified, for now)
pub trait PartialEq {
    // Required method
    //
    // `Self` is a Rust keyword that stands for 
    // "the type that is implementing the trait"
    fn eq(&self, other: &Self) -> bool;

    // Provided method
    fn ne(&self, other: &Self) -> bool { ... }
}
```

When you write `x == y` the compiler will look for an implementation of the `PartialEq` trait for the types of `x` and `y`
and replace `x == y` with `x.eq(y)`. It's syntactic sugar!
当你编写 `x == y` 时，编译器会查找 `PartialEq` 特征针对 `x` 和 `y` 类型的实现，并将 `x == y` 替换为 `x.eq(y)`。这是语法糖！

This is the correspondence for the main operators:
以下是主要运算符的对应关系：

| Operator                 | Trait                                                                   |
| ------------------------ | ----------------------------------------------------------------------- |
| `+`                      | [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html)               |
| `-`                      | [`Sub`](https://doc.rust-lang.org/std/ops/trait.Sub.html)               |
| `*`                      | [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html)               |
| `/`                      | [`Div`](https://doc.rust-lang.org/std/ops/trait.Div.html)               |
| `%`                      | [`Rem`](https://doc.rust-lang.org/std/ops/trait.Rem.html)               |
| `==` and `!=`            | [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)   |
| `<`, `>`, `<=`, and `>=` | [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) |

Arithmetic operators live in the [`std::ops`](https://doc.rust-lang.org/std/ops/index.html) module,
while comparison ones live in the [`std::cmp`](https://doc.rust-lang.org/std/cmp/index.html) module.
算术运算符位于 [`std::ops`](https://doc.rust-lang.org/std/ops/index.html) 模块中，
而比较运算符位于 [`std::cmp`](https://doc.rust-lang.org/std/cmp/index.html) 模块中。

## Default implementations
## 默认实现

The comment on `PartialEq::ne` states that "`ne` is a provided method".\
`PartialEq::ne` 上的注释说明"`ne` 是一个提供的方法"。\
It means that `PartialEq` provides a **default implementation** for `ne` in the trait definition—the `{ ... }` elided
block in the definition snippet.\
这意味着 `PartialEq` 在特征定义中为 `ne` 提供了一个**默认实现**——即定义片段中省略的 `{ ... }` 块。\
If we expand the elided block, it looks like this:
如果我们展开省略的块，它看起来像这样：

```rust
pub trait PartialEq {
    fn eq(&self, other: &Self) -> bool;

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
```

It's what you expect: `ne` is the negation of `eq`.\
正如你所期望的：`ne` 是 `eq` 的否定。\
Since a default implementation is provided, you can skip implementing `ne` when you implement `PartialEq` for your type.
It's enough to implement `eq`:
由于提供了默认实现，当你为你的类型实现 `PartialEq` 时，你可以跳过实现 `ne`。实现 `eq` 就足够了：

```rust
struct WrappingU8 {
    inner: u8,
}

impl PartialEq for WrappingU8 {
    fn eq(&self, other: &WrappingU8) -> bool {
        self.inner == other.inner
    }
    
    // No `ne` implementation here
}
```

You are not forced to use the default implementation though.
不过，你并不强制要求使用默认实现。
You can choose to override it when you implement the trait:
你可以选择在实现该特征时覆盖它：

```rust
struct MyType;

impl PartialEq for MyType {
    fn eq(&self, other: &MyType) -> bool {
        // Custom implementation
    }

    fn ne(&self, other: &MyType) -> bool {
        // Custom implementation
    }
}
```
