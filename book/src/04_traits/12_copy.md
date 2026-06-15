# Copying values, pt. 2
# 复制值，第二部分

Let's consider the same example as before, but with a slight twist: using `u32` rather than `String` as a type.
让我们考虑与之前相同的例子，但稍有变化：使用 `u32` 而不是 `String` 作为类型。

```rust
fn consumer(s: u32) { /* */ }

fn example() {
     let s: u32 = 5;
     consumer(s);
     let t = s + 1;
}
```

It'll compile without errors! What's going on here? What's the difference between `String` and `u32`
that makes the latter work without `.clone()`?
它会无错误地编译！这是怎么回事？`String` 和 `u32` 之间有什么区别，使得后者不需要 `.clone()` 就能工作？

## `Copy`
## `Copy` 特征

`Copy` is another trait defined in Rust's standard library:
`Copy` 是 Rust 标准库中定义的另一个特征：

```rust
pub trait Copy: Clone { }
```

It is a marker trait, just like `Sized`.
它是一个标记特征，就像 `Sized` 一样。

If a type implements `Copy`, there's no need to call `.clone()` to create a new instance of the type:
Rust does it **implicitly** for you.\
如果一个类型实现了 `Copy`，就无需调用 `.clone()` 来创建该类型的新实例：Rust 会**隐式地**为你做这件事。\
`u32` is an example of a type that implements `Copy`, which is why the example above compiles without errors:
when `consumer(s)` is called, Rust creates a new `u32` instance by performing a **bitwise copy** of `s`,
and then passes that new instance to `consumer`. It all happens behind the scenes, without you having to do anything.
`u32` 是实现 `Copy` 的类型的一个例子，这就是为什么上面的例子无错误地编译：当调用 `consumer(s)` 时，Rust 通过对 `s` 执行**逐位复制**创建一个新的 `u32` 实例，然后将该新实例传递给 `consumer`。这一切都在幕后发生，你无需做任何事情。

## What can be `Copy`?
## 什么可以是 `Copy`？

`Copy` is not equivalent to "automatic cloning", although it implies it.\
`Copy` 不等同于"自动克隆"，尽管它隐含了这一点。\
Types must meet a few requirements in order to be allowed to implement `Copy`.
类型必须满足一些要求才能被允许实现 `Copy`。

First of all, it must implement `Clone`, since `Copy` is a subtrait of `Clone`.
This makes sense: if Rust can create a new instance of a type _implicitly_, it should
also be able to create a new instance _explicitly_ by calling `.clone()`.
首先，它必须实现 `Clone`，因为 `Copy` 是 `Clone` 的子特征。这是有道理的：如果 Rust 可以_隐式地_创建类型的新实例，它也应当能够通过调用 `.clone()` _显式地_创建新实例。

That's not all, though. A few more conditions must be met:
不过，这还不是全部。还必须满足一些条件：

1. The type doesn't manage any _additional_ resources (e.g. heap memory, file handles, etc.) beyond the `std::mem::size_of`
   bytes that it occupies in memory.
1. 该类型不管理超出其在内存中占用的 `std::mem::size_of` 字节之外的任何_额外_资源（例如堆内存、文件句柄等）。
2. The type is not a mutable reference (`&mut T`).
2. 该类型不是可变引用（`&mut T`）。

If both conditions are met, then Rust can safely create a new instance of the type by performing a **bitwise copy**
of the original instance—this is often referred to as a `memcpy` operation, after the C standard library function
that performs the bitwise copy.
如果两个条件都满足，那么 Rust 可以通过对原始实例执行**逐位复制**来安全地创建该类型的新实例——这通常被称为 `memcpy` 操作，以执行逐位复制的 C 标准库函数命名。

### Case study 1: `String`
### 案例研究 1：`String`

`String` is a type that doesn't implement `Copy`.\
`String` 是一种没有实现 `Copy` 的类型。\
Why? Because it manages an additional resource: the heap-allocated memory buffer that stores the string's data.
为什么？因为它管理着一个额外的资源：存储字符串数据的堆分配内存缓冲区。

Let's imagine that Rust allowed `String` to implement `Copy`.\
让我们想象一下，Rust 允许 `String` 实现 `Copy`。\
Then, when a new `String` instance is created by performing a bitwise copy of the original instance, both the original
and the new instance would point to the same memory buffer:
那么，当通过对原始实例执行逐位复制来创建新的 `String` 实例时，原始实例和新实例将指向同一个内存缓冲区：

```text
              s                                 copied_s
+---------+--------+----------+      +---------+--------+----------+
| pointer | length | capacity |      | pointer | length | capacity |
|  |      |   5    |    5     |      |  |      |   5    |    5     |
+--|------+--------+----------+      +--|------+--------+----------+
   |                                    |
   |                                    |
   v                                    |
 +---+---+---+---+---+                  |
 | H | e | l | l | o |                  |
 +---+---+---+---+---+                  |
   ^                                    |
   |                                    |
   +------------------------------------+
```

This is bad!
这很糟糕！
Both `String` instances would try to free the memory buffer when they go out of scope,
leading to a double-free error.
两个 `String` 实例在超出作用域时都会尝试释放内存缓冲区，导致双重释放错误。
You could also create two distinct `&mut String` references that point to the same memory buffer,
violating Rust's borrowing rules.
你还可能创建两个指向同一内存缓冲区的不同 `&mut String` 引用，违反了 Rust 的借用规则。

### Case study 2: `u32`
### 案例研究 2：`u32`

`u32` implements `Copy`. All integer types do, in fact.\
`u32` 实现了 `Copy`。实际上，所有整数类型都实现了。\
An integer is "just" the bytes that represent the number in memory. There's nothing more!
整数"只是"在内存中表示该数字的字节。没有其他东西了！
If you copy those bytes, you get another perfectly valid integer instance.
如果你复制这些字节，你会得到另一个完全有效的整数实例。
Nothing bad can happen, so Rust allows it.
不会发生任何坏事，所以 Rust 允许它。

### Case study 3: `&mut u32`
### 案例研究 3：`&mut u32`

When we introduced ownership and mutable borrows, we stated one rule quite clearly: there
can only ever be _one_ mutable borrow of a value at any given time.\
当我们介绍所有权和可变借用时，我们非常清楚地说明了一个规则：在任何给定时间，一个值只能有_一个_可变借用。\
That's why `&mut u32` doesn't implement `Copy`, even though `u32` does.
这就是为什么 `&mut u32` 没有实现 `Copy`，即使 `u32` 实现了。

If `&mut u32` implemented `Copy`, you could create multiple mutable references to
the same value and modify it in multiple places at the same time.
如果 `&mut u32` 实现了 `Copy`，你可以创建对同一值的多个可变引用，并在多个地方同时修改它。
That'd be a violation of Rust's borrowing rules!
这将违反 Rust 的借用规则！
It follows that `&mut T` never implements `Copy`, no matter what `T` is.
因此，无论 `T` 是什么，`&mut T` 从不实现 `Copy`。

## Implementing `Copy`
## 实现 `Copy`

In most cases, you don't need to manually implement `Copy`.
在大多数情况下，你不需要手动实现 `Copy`。
You can just derive it, like this:
你可以直接派生它，像这样：

```rust
#[derive(Copy, Clone)]
struct MyStruct {
    field: u32,
}
```
