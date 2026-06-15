# Arrays
# 数组

As soon as we start talking about "ticket management" we need to think about a way to store _multiple_ tickets.
一旦我们开始谈论"票据管理"，我们就需要考虑一种存储_多个_票据的方式。
In turn, this means we need to think about collections. In particular, homogeneous collections:
we want to store multiple instances of the same type.
反过来，这意味着我们需要考虑集合。具体来说，是同质集合：我们希望存储同一类型的多个实例。

What does Rust have to offer in this regard?
Rust 在这方面提供了什么？

## Arrays
## 数组

A first attempt could be to use an **array**.\
第一次尝试可以是使用**数组**。\
Arrays in Rust are fixed-size collections of elements of the same type.
Rust 中的数组是相同类型元素的固定大小集合。

Here's how you can define an array:
以下是你如何定义一个数组：

```rust
// Array type syntax: [ <type> ; <number of elements> ]
let numbers: [u32; 3] = [1, 2, 3];
```

This creates an array of 3 integers, initialized with the values `1`, `2`, and `3`.\
这将创建一个包含 3 个整数的数组，初始化为值 `1`、`2` 和 `3`。\
The type of the array is `[u32; 3]`, which reads as "an array of `u32`s with a length of 3".
数组的类型是 `[u32; 3]`，读作"长度为 3 的 `u32` 数组"。

If all array elements are the same, you can use a shorter syntax to initialize it:
如果所有数组元素都相同，你可以使用更短的语法来初始化它：

```rust
// [ <value> ; <number of elements> ]
let numbers: [u32; 3] = [1; 3];
```

`[1; 3]` creates an array of three elements, all equal to `1`.
`[1; 3]` 创建一个包含三个元素的数组，所有元素都等于 `1`。

### Accessing elements
### 访问元素

You can access elements of an array using square brackets:
你可以使用方括号访问数组的元素：

```rust
let first = numbers[0];
let second = numbers[1];
let third = numbers[2];
```

The index must be of type `usize`.\
索引的类型必须是 `usize`。\
Arrays are **zero-indexed**, like everything in Rust. You've seen this before with string slices and field indexing in
tuples/tuple-like variants.
数组是**零索引的**，就像 Rust 中的一切。你之前在字符串切片和元组/元组类变体中的字段索引中已经见过这一点。

### Out-of-bounds access
### 越界访问

If you try to access an element that's out of bounds, Rust will panic:
如果你尝试访问一个越界的元素，Rust 会恐慌：

```rust
let numbers: [u32; 3] = [1, 2, 3];
let fourth = numbers[3]; // This will panic
```

This is enforced at runtime using **bounds checking**. It comes with a small performance overhead, but it's how
Rust prevents buffer overflows.\
这是通过**边界检查**在运行时强制执行的。它带来少量的性能开销，但这就是 Rust 防止缓冲区溢出的方式。\
In some scenarios the Rust compiler can optimize away bounds checks, especially if iterators are involved—we'll speak
more about this later on.
在某些场景下，Rust 编译器可以优化掉边界检查，特别是当涉及迭代器时——我们稍后会更多地讨论这一点。

If you don't want to panic, you can use the `get` method, which returns an `Option<&T>`:
如果你不想恐慌，你可以使用 `get` 方法，它返回一个 `Option<&T>`：

```rust
let numbers: [u32; 3] = [1, 2, 3];
assert_eq!(numbers.get(0), Some(&1));
// You get a `None` if you try to access an out-of-bounds index
// rather than a panic.
assert_eq!(numbers.get(3), None);
```

### Performance
### 性能

Since the size of an array is known at compile-time, the compiler can allocate the array on the stack.
由于数组的大小在编译时已知，编译器可以在栈上分配数组。
If you run the following code:
如果你运行以下代码：

```rust
let numbers: [u32; 3] = [1, 2, 3];
```

You'll get the following memory layout:
你将得到以下内存布局：

```text
        +---+---+---+
Stack:  | 1 | 2 | 3 |
        +---+---+---+
```

In other words, the size of an array is `std::mem::size_of::<T>() * N`, where `T` is the type of the elements and `N` is
the number of elements.\
换句话说，数组的大小是 `std::mem::size_of::<T>() * N`，其中 `T` 是元素的类型，`N` 是元素的数量。\
You can access and replace each element in `O(1)` time.
你可以在 `O(1)` 时间内访问和替换每个元素。
