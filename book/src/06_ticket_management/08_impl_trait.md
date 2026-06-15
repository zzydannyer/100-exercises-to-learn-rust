# `impl Trait`
# `impl Trait`

`TicketStore::to_dos` returns a `Vec<&Ticket>`.\
`TicketStore::to_dos` 返回一个 `Vec<&Ticket>`。\
That signature introduces a new heap allocation every time `to_dos` is called, which may be unnecessary depending
on what the caller needs to do with the result.
这个签名在每次调用 `to_dos` 时都会引入一个新的堆分配，根据调用者对结果的处理方式，这可能是不必要的。
It'd be better if `to_dos` returned an iterator instead of a `Vec`, thus empowering the caller to decide whether to
collect the results into a `Vec` or just iterate over them.
如果 `to_dos` 返回一个迭代器而不是 `Vec` 会更好，这样调用者就可以决定是将结果收集到 `Vec` 中还是直接迭代它们。

That's tricky though!
但这很棘手！
What's the return type of `to_dos`, as implemented below?
如下实现的 `to_dos` 的返回类型是什么？

```rust
impl TicketStore {
    pub fn to_dos(&self) -> ??? {
        self.tickets.iter().filter(|t| t.status == Status::ToDo)
    }
}
```

## Unnameable types
## 不可命名的类型

The `filter` method returns an instance of `std::iter::Filter`, which has the following definition:
`filter` 方法返回一个 `std::iter::Filter` 的实例，其定义如下：

```rust
pub struct Filter<I, P> { /* fields omitted */ }
```

where `I` is the type of the iterator being filtered on and `P` is the predicate used to filter the elements.\
其中 `I` 是被过滤的迭代器的类型，`P` 是用于过滤元素的谓词。\
We know that `I` is `std::slice::Iter<'_, Ticket>` in this case, but what about `P`?\
我们知道在这种情况下 `I` 是 `std::slice::Iter<'_, Ticket>`，但是 `P` 呢？\
`P` is a closure, an **anonymous function**. As the name suggests, closures don't have a name,
so we can't write them down in our code.
`P` 是一个闭包，一个**匿名函数**。顾名思义，闭包没有名称，所以我们不能在代码中写出它们。

Rust has a solution for this: **impl Trait**.
Rust 对此有一个解决方案：**impl Trait**。

## `impl Trait`
## `impl Trait`

`impl Trait` is a feature that allows you to return a type without specifying its name.
You just declare what trait(s) the type implements, and Rust figures out the rest.
`impl Trait` 是一个特性，允许你返回一个类型而不指定其名称。你只需声明该类型实现了什么特征，Rust 会解决剩下的问题。

In this case, we want to return an iterator of references to `Ticket`s:
在这种情况下，我们想要返回一个对 `Ticket` 引用的迭代器：

```rust
impl TicketStore {
    pub fn to_dos(&self) -> impl Iterator<Item = &Ticket> {
        self.tickets.iter().filter(|t| t.status == Status::ToDo)
    }
}
```

That's it!
就是这样！

## Generic?
## 泛型？

`impl Trait` in return position is **not** a generic parameter.
`impl Trait` 在返回位置**不是**泛型参数。

Generics are placeholders for types that are filled in by the caller of the function.
A function with a generic parameter is **polymorphic**: it can be called with different types, and the compiler will generate
a different implementation for each type.
泛型是由函数的调用者填充的类型的占位符。具有泛型参数的函数是**多态的**：它可以与不同的类型一起调用，并且编译器会为每种类型生成不同的实现。

That's not the case with `impl Trait`.
The return type of a function with `impl Trait` is **fixed** at compile time, and the compiler will generate
a single implementation for it.
情况并非如此。使用 `impl Trait` 的函数的返回类型在编译时是**固定的**，编译器会为其生成一个单一的实现。
This is why `impl Trait` is also called **opaque return type**: the caller doesn't know the exact type of the return value,
only that it implements the specified trait(s). But the compiler knows the exact type, there is no polymorphism involved.
这就是为什么 `impl Trait` 也被称为**不透明返回类型**：调用者不知道返回值的具体类型，只知道它实现了指定的特征。但编译器知道具体类型，不涉及多态。

## RPIT
## RPIT

If you read RFCs or deep-dives about Rust, you might come across the acronym **RPIT**.\
如果你阅读关于 Rust 的 RFC 或深度文章，你可能会遇到缩写 **RPIT**。\
It stands for **"Return Position Impl Trait"** and refers to the use of `impl Trait` in return position.
它代表**"返回位置 Impl Trait"**，指的是在返回位置使用 `impl Trait`。
