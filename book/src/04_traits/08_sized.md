# `Sized`
# `Sized` 特征

There's more to `&str` than meets the eye, even after having
investigated deref coercion.\
即使研究了解引用强制转换之后，`&str` 仍有更多不为人知的内容。\
From our previous [discussion on memory layouts](../03_ticket_v1/10_references_in_memory.md),
it would have been reasonable to expect `&str` to be represented as a single `usize` on
the stack, a pointer. That's not the case though. `&str` stores some **metadata** next
to the pointer: the length of the slice it points to. Going back to the example from
[a previous section](06_str_slice.md):
从我们之前关于[内存布局的讨论](../03_ticket_v1/10_references_in_memory.md)来看，合理地期望 `&str` 在栈上被表示为一个单一的 `usize`（一个指针）。但事实并非如此。`&str` 在指针旁边存储了一些**元数据**：它指向的切片的长度。回到[前一节](06_str_slice.md)中的例子：

```rust
let mut s = String::with_capacity(5);
s.push_str("Hello");
// Create a string slice reference from the `String`, 
// skipping the first byte.
let slice: &str = &s[1..];
```

In memory, we get:
在内存中，我们得到：

```text
                    s                              slice
      +---------+--------+----------+      +---------+--------+
Stack | pointer | length | capacity |      | pointer | length |
      |    |    |   5    |    5     |      |    |    |   4    |
      +----|----+--------+----------+      +----|----+--------+
           |        s                           |  
           |                                    |
           v                                    | 
         +---+---+---+---+---+                  |
Heap:    | H | e | l | l | o |                  |
         +---+---+---+---+---+                  |
               ^                                |
               |                                |
               +--------------------------------+
```

What's going on?
这是怎么回事？

## Dynamically sized types
## 动态大小类型

`str` is a **dynamically sized type** (DST).\
`str` 是一种**动态大小类型**（DST）。\
A DST is a type whose size is not known at compile time. Whenever you have a
reference to a DST, like `&str`, it has to include additional
information about the data it points to. It is a **fat pointer**.\
DST 是一种在编译时大小未知的类型。当你有一个对 DST 的引用时，比如 `&str`，它必须包含关于它指向的数据的额外信息。它是一个**胖指针**。\
In the case of `&str`, it stores the length of the slice it points to.
We'll see more examples of DSTs in the rest of the course.
对于 `&str`，它存储了它指向的切片的长度。我们将在课程的其余部分看到更多 DST 的例子。

## The `Sized` trait
## `Sized` 特征

Rust's `std` library defines a trait called `Sized`.
Rust 的 `std` 库定义了一个名为 `Sized` 的特征。

```rust
pub trait Sized {
    // This is an empty trait, no methods to implement.
}
```

A type is `Sized` if its size is known at compile time. In other words, it's not a DST.
如果类型的大小在编译时已知，则该类型是 `Sized` 的。换句话说，它不是 DST。

### Marker traits
### 标记特征

`Sized` is your first example of a **marker trait**.\
`Sized` 是你遇到的第一个**标记特征**的例子。\
A marker trait is a trait that doesn't require any methods to be implemented. It doesn't define any behavior.
It only serves to **mark** a type as having certain properties.
The mark is then leveraged by the compiler to enable certain behaviors or optimizations.
标记特征是不需要实现任何方法的特征。它不定义任何行为。它仅仅用于**标记**一个类型具有某些属性。然后编译器利用该标记来启用某些行为或优化。

### Auto traits
### 自动特征

In particular, `Sized` is also an **auto trait**.\
特别是，`Sized` 也是一个**自动特征**。\
You don't need to implement it explicitly; the compiler implements it automatically for you
based on the type's definition.
你不需要显式地实现它；编译器会根据类型的定义自动为你实现它。

### Examples
### 示例

All the types we've seen so far are `Sized`: `u32`, `String`, `bool`, etc.
到目前为止我们看到的所有类型都是 `Sized` 的：`u32`、`String`、`bool` 等。

`str`, as we just saw, is not `Sized`.\
`str`，正如我们刚才所见，不是 `Sized` 的。\
`&str` is `Sized` though! We know its size at compile time: two `usize`s, one for the pointer
and one for the length.
不过 `&str` 是 `Sized` 的！我们在编译时就知道它的大小：两个 `usize`，一个用于指针，一个用于长度。
