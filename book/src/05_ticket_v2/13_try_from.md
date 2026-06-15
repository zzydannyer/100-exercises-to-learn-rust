# `TryFrom` and `TryInto`
# `TryFrom` 和 `TryInto`

In the previous chapter we looked at the [`From` and `Into` traits](../04_traits/09_from.md),
Rust's idiomatic interfaces for **infallible** type conversions.\
在上一章中，我们研究了 [`From` 和 `Into` 特征](../04_traits/09_from.md)，这是 Rust 用于**无失败**类型转换的惯用接口。\
But what if the conversion is not guaranteed to succeed?
但如果转换不能保证成功呢？

We now know enough about errors to discuss the **fallible** counterparts of `From` and `Into`:
`TryFrom` and `TryInto`.
我们现在对错误了解得足够多了，可以讨论 `From` 和 `Into` 的**易错**对应物：`TryFrom` 和 `TryInto`。

## `TryFrom` and `TryInto`
## `TryFrom` 和 `TryInto`

Both `TryFrom` and `TryInto` are defined in the `std::convert` module, just like `From` and `Into`.
`TryFrom` 和 `TryInto` 都定义在 `std::convert` 模块中，就像 `From` 和 `Into` 一样。

```rust
pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```

The main difference between `From`/`Into` and `TryFrom`/`TryInto` is that the latter return a `Result` type.\
`From`/`Into` 和 `TryFrom`/`TryInto` 之间的主要区别在于后者返回一个 `Result` 类型。\
This allows the conversion to fail, returning an error instead of panicking.
这允许转换失败，返回错误而不是恐慌。

## `Self::Error`
## `Self::Error`

Both `TryFrom` and `TryInto` have an associated `Error` type.
This allows each implementation to specify its own error type, ideally the most appropriate for the conversion
being attempted.
`TryFrom` 和 `TryInto` 都有一个关联的 `Error` 类型。这允许每个实现指定自己的错误类型，理想情况下是最适合正在尝试的转换的类型。

`Self::Error` is a way to refer to the `Error` associated type defined in the trait itself.
`Self::Error` 是引用特征本身中定义的 `Error` 关联类型的一种方式。

## Duality
## 对偶性

Just like `From` and `Into`, `TryFrom` and `TryInto` are dual traits.\
就像 `From` 和 `Into` 一样，`TryFrom` 和 `TryInto` 是对偶特征。\
If you implement `TryFrom` for a type, you get `TryInto` for free.
如果你为一个类型实现了 `TryFrom`，你就会免费获得 `TryInto`。
