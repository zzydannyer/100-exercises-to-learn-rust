# `Deref` trait
# `Deref` 特征

In the previous exercise you didn't have to do much, did you?
在之前的练习中，你不需要做太多事情，是吗？

Changing
将

```rust
impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }
}
```

to
改为

```rust
impl Ticket {
    pub fn title(&self) -> &str {
        &self.title
    }
}
```

was all you needed to do to get the code to compile and the tests to pass.
就是让代码编译并通过测试所需要做的全部事情。
Some alarm bells should be ringing in your head though.
不过，你的脑海里应该响起一些警钟了。

## It shouldn't work, but it does
## 它不应该有效，但它确实有效

Let's review the facts:
让我们回顾一下事实：

- `self.title` is a `String`
- `self.title` 是一个 `String`
- `&self.title` is, therefore, a `&String`
- 因此，`&self.title` 是一个 `&String`
- The output of the (modified) `title` method is `&str`
-（修改后的）`title` 方法的输出是 `&str`

You would expect a compiler error, wouldn't you? `Expected &String, found &str` or something similar.
Instead, it just works. **Why**?
你会期望一个编译器错误，不是吗？`Expected &String, found &str` 或类似的信息。相反，它就能工作。**为什么**？

## `Deref` to the rescue
## `Deref` 来救援

The `Deref` trait is the mechanism behind the language feature known as [**deref coercion**](https://doc.rust-lang.org/std/ops/trait.Deref.html#deref-coercion).\
`Deref` 特征是一种称为[**解引用强制转换**](https://doc.rust-lang.org/std/ops/trait.Deref.html#deref-coercion)的语言特性背后的机制。\
The trait is defined in the standard library, in the `std::ops` module:
该特征定义在标准库的 `std::ops` 模块中：

```rust
// I've slightly simplified the definition for now.
// We'll see the full definition later on.
pub trait Deref {
    type Target;
    
    fn deref(&self) -> &Self::Target;
}
```

`type Target` is an **associated type**.\
`type Target` 是一个**关联类型**。\
It's a placeholder for a concrete type that must be specified when the trait is implemented.
它是具体类型的占位符，在实现特征时必须指定。

## Deref coercion
## 解引用强制转换

By implementing `Deref<Target = U>` for a type `T` you're telling the compiler that `&T` and `&U` are
somewhat interchangeable.\
通过为类型 `T` 实现 `Deref<Target = U>`，你告诉编译器 `&T` 和 `&U` 在某种程度上是可以互换的。\
In particular, you get the following behavior:
具体来说，你会获得以下行为：

- References to `T` are implicitly converted into references to `U` (i.e. `&T` becomes `&U`)
- 对 `T` 的引用被隐式转换为对 `U` 的引用（即 `&T` 变为 `&U`）
- You can call on `&T` all the methods defined on `U` that take `&self` as input.
- 你可以在 `&T` 上调用在 `U` 上定义的所有以 `&self` 作为输入的方法。

There is one more thing around the dereference operator, `*`, but we don't need it yet (see `std`'s docs
if you're curious).
关于解引用运算符 `*` 还有更多内容，但我们暂时不需要它（如果你好奇的话，可以查看 `std` 的文档）。

## `String` implements `Deref`
## `String` 实现了 `Deref`

`String` implements `Deref` with `Target = str`:
`String` 实现了 `Target = str` 的 `Deref`：

```rust
impl Deref for String {
    type Target = str;
    
    fn deref(&self) -> &str {
        // [...]
    }
}
```

Thanks to this implementation and deref coercion, a `&String` is automatically converted into a `&str` when needed.
得益于这个实现和解引用强制转换，`&String` 在需要时会自动转换为 `&str`。

## Don't abuse deref coercion
## 不要滥用解引用强制转换

Deref coercion is a powerful feature, but it can lead to confusion.\
解引用强制转换是一个强大的特性，但它可能导致混淆。\
Automatically converting types can make the code harder to read and understand. If a method with the same name
is defined on both `T` and `U`, which one will be called?
自动转换类型会使代码更难阅读和理解。如果同名方法在 `T` 和 `U` 上都定义了，会调用哪一个？

We'll examine later in the course the "safest" use cases for deref coercion: smart pointers.
我们将在课程的后面部分探讨解引用强制转换的"最安全"用例：智能指针。
