# `From` and `Into`
# `From` 和 `Into`

Let's go back to where our string journey started:
让我们回到字符串之旅开始的地方：

```rust
let ticket = Ticket::new(
    "A title".into(), 
    "A description".into(), 
    "To-Do".into()
);
```

We now know enough to start unpacking what `.into()` is doing here.
我们现在知道足够多的知识来开始解析 `.into()` 在这里的作用了。

## The problem
## 问题

This is the signature of the `new` method:
这是 `new` 方法的签名：

```rust
impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: String
    ) -> Self {
        // [...]
    }
}
```

We've also seen that string literals (such as `"A title"`) are of type `&str`.\
我们还看到字符串字面量（如 `"A title"`）的类型是 `&str`。\
We have a type mismatch here: a `String` is expected, but we have a `&str`.
No magical coercion will come to save us this time; we need **to perform a conversion**.
这里存在类型不匹配：期望的是 `String`，但我们有的是 `&str`。这次没有魔法强制转换来救我们了；我们需要**执行转换**。

## `From` and `Into`
## `From` 和 `Into`

The Rust standard library defines two traits for **infallible conversions**: `From` and `Into`,
in the `std::convert` module.
Rust 标准库在 `std::convert` 模块中定义了两种用于**无失败转换**的特征：`From` 和 `Into`。

```rust
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized {
    fn into(self) -> T;
}
```

These trait definitions showcase a few concepts that we haven't seen before: **supertraits** and **implicit trait bounds**.
Let's unpack those first.
这些特征定义展示了一些我们之前没见过的新概念：**超特征**和**隐式特征约束**。让我们先解析这些。

### Supertrait / Subtrait
### 超特征 / 子特征

The `From: Sized` syntax implies that `From` is a **subtrait** of `Sized`: any type that
implements `From` must also implement `Sized`.
Alternatively, you could say that `Sized` is a **supertrait** of `From`.
`From: Sized` 语法意味着 `From` 是 `Sized` 的**子特征**：任何实现 `From` 的类型也必须实现 `Sized`。或者，你也可以说 `Sized` 是 `From` 的**超特征**。

### Implicit trait bounds
### 隐式特征约束

Every time you have a generic type parameter, the compiler implicitly assumes that it's `Sized`.
每当你有一个泛型类型参数时，编译器会隐式地假设它是 `Sized` 的。

For example:
例如：

```rust
pub struct Foo<T> {
    inner: T,
}
```

is actually equivalent to:
实际上等价于：

```rust
pub struct Foo<T: Sized> 
{
    inner: T,
}
```

In the case of `From<T>`, the trait definition is equivalent to:
对于 `From<T>`，特征定义等价于：

```rust
pub trait From<T: Sized>: Sized {
    fn from(value: T) -> Self;
}
```

In other words, _both_ `T` and the type implementing `From<T>` must be `Sized`, even
though the former bound is implicit.
换句话说，`T` 和实现 `From<T>` 的类型_都_必须是 `Sized` 的，尽管前一个约束是隐式的。

### Negative trait bounds
### 负特征约束

You can opt out of the implicit `Sized` bound with a **negative trait bound**:
你可以通过**负特征约束**来选择退出隐式的 `Sized` 约束：

```rust
pub struct Foo<T: ?Sized> {
    //            ^^^^^^^
    //            This is a negative trait bound
    inner: T,
}
```

This syntax reads as "`T` may or may not be `Sized`", and it allows you to
bind `T` to a DST (e.g. `Foo<str>`). It is a special case, though: negative trait bounds are exclusive to `Sized`,
you can't use them with other traits.
这个语法读作"`T` 可能是也可能不是 `Sized`"，它允许你将 `T` 绑定到 DST（例如 `Foo<str>`）。不过，这是一个特殊情况：负特征约束是 `Sized` 独有的，你不能将它们用于其他特征。

## `&str` to `String`
## `&str` 到 `String`

In [`std`'s documentation](https://doc.rust-lang.org/std/convert/trait.From.html#implementors)
you can see which `std` types implement the `From` trait.\
在 [`std` 的文档](https://doc.rust-lang.org/std/convert/trait.From.html#implementors)中，你可以看到哪些 `std` 类型实现了 `From` 特征。\
You'll find that `String` implements `From<&str> for String`. Thus, we can write:
你会发现 `String` 实现了 `From<&str> for String`。因此，我们可以编写：

```rust
let title = String::from("A title");
```

We've been primarily using `.into()`, though.\
不过，我们主要使用的是 `.into()`。\
If you check out the [implementors of `Into`](https://doc.rust-lang.org/std/convert/trait.Into.html#implementors)
you won't find `Into<String> for &str`. What's going on?
如果你查看 [`Into` 的实现者](https://doc.rust-lang.org/std/convert/trait.Into.html#implementors)，你不会找到 `Into<String> for &str`。这是怎么回事？

`From` and `Into` are **dual traits**.\
`From` 和 `Into` 是**对偶特征**。\
In particular, `Into` is implemented for any type that implements `From` using a **blanket implementation**:
具体来说，`Into` 通过**覆盖实现**为任何实现了 `From` 的类型实现：

```rust
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
```

If a type `U` implements `From<T>`, then `Into<U> for T` is automatically implemented. That's why
we can write `let title = "A title".into();`.
如果类型 `U` 实现了 `From<T>`，那么 `Into<U> for T` 就会自动实现。这就是为什么我们可以写 `let title = "A title".into();`。

## `.into()`
## `.into()`

Every time you see `.into()`, you're witnessing a conversion between types.\
每当你看到 `.into()`，你就在目睹类型之间的转换。\
What's the target type, though?
但是，目标类型是什么？

In most cases, the target type is either:
在大多数情况下，目标类型要么是：

- Specified by the signature of a function/method (e.g. `Ticket::new` in our example above)
- 由函数/方法的签名指定（例如上面例子中的 `Ticket::new`）
- Specified in the variable declaration with a type annotation (e.g. `let title: String = "A title".into();`)
- 在变量声明中使用类型注解指定（例如 `let title: String = "A title".into();`）

`.into()` will work out of the box as long as the compiler can infer the target type from the context without ambiguity.
只要编译器能够从上下文无歧义地推断出目标类型，`.into()` 就能直接工作。
