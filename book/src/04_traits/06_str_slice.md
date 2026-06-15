# String slices
# 字符串切片

Throughout the previous chapters you've seen quite a few **string literals** being used in the code,
like `"To-Do"` or `"A ticket description"`.
在之前的章节中，你已经看到代码中使用了相当多的**字符串字面量**，比如 `"To-Do"` 或 `"A ticket description"`。
They were always followed by a call to `.to_string()` or `.into()`. It's time to understand why!
它们后面总是跟着对 `.to_string()` 或 `.into()` 的调用。是时候理解为什么了！

## String literals
## 字符串字面量

You define a string literal by enclosing the raw text in double quotes:
你可以通过将原始文本括在双引号中来定义字符串字面量：

```rust
let s = "Hello, world!";
```

The type of `s` is `&str`, a **reference to a string slice**.
`s` 的类型是 `&str`，一个**对字符串切片的引用**。

## Memory layout
## 内存布局

`&str` and `String` are different types—they're not interchangeable.\
`&str` 和 `String` 是不同的类型——它们不可互换。\
Let's recall the memory layout of a `String` from our
[previous exploration](../03_ticket_v1/09_heap.md).
让我们从[之前的探索](../03_ticket_v1/09_heap.md)中回忆一下 `String` 的内存布局。
If we run:
如果我们运行：

```rust
let mut s = String::with_capacity(5);
s.push_str("Hello");
```

we'll get this scenario in memory:
我们将在内存中看到如下情况：

```text
      +---------+--------+----------+
Stack | pointer | length | capacity | 
      |  |      |   5    |    5     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+---+---+
Heap:  | H | e | l | l | o |
       +---+---+---+---+---+
```

If you remember, we've [also examined](../03_ticket_v1/10_references_in_memory.md)
how a `&String` is laid out in memory:
如果你还记得，我们[还检查过](../03_ticket_v1/10_references_in_memory.md) `&String` 在内存中是如何布局的：

```text
     --------------------------------------
     |                                    |         
+----v----+--------+----------+      +----|----+
| pointer | length | capacity |      | pointer |
|    |    |   5    |    5     |      |         |
+----|----+--------+----------+      +---------+
     |        s                          &s 
     |       
     v       
   +---+---+---+---+---+
   | H | e | l | l | o |
   +---+---+---+---+---+
```

`&String` points to the memory location where the `String`'s metadata is stored.\
`&String` 指向存储 `String` 元数据的内存位置。\
If we follow the pointer, we get to the heap-allocated data. In particular, we get to the first byte of the string, `H`.
如果我们跟踪指针，就会到达堆分配的数据。具体来说，我们到达字符串的第一个字节 `H`。

What if we wanted a type that represents a **substring** of `s`? E.g. `ello` in `Hello`?
如果我们想要一个表示 `s` 的**子字符串**的类型呢？例如 `Hello` 中的 `ello`？

## String slices
## 字符串切片

A `&str` is a **view** into a string, a **reference** to a sequence of UTF-8 bytes stored elsewhere.
`&str` 是字符串的一个**视图**，是对存储在其他地方的 UTF-8 字节序列的**引用**。
You can, for example, create a `&str` from a `String` like this:
例如，你可以像这样从 `String` 创建一个 `&str`：

```rust
let mut s = String::with_capacity(5);
s.push_str("Hello");
// Create a string slice reference from the `String`, 
// skipping the first byte.
let slice: &str = &s[1..];
```

In memory, it'd look like this:
在内存中，它看起来像这样：

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

`slice` stores two pieces of information on the stack:
`slice` 在栈上存储两段信息：

- A pointer to the first byte of the slice.
- 一个指向切片第一个字节的指针。
- The length of the slice.
- 切片的长度。

`slice` doesn't own the data, it just points to it: it's a **reference** to the `String`'s heap-allocated data.\
`slice` 不拥有数据，它只是指向它：它是对 `String` 堆分配数据的**引用**。\
When `slice` is dropped, the heap-allocated data won't be deallocated, because it's still owned by `s`.
当 `slice` 被丢弃时，堆分配的数据不会被释放，因为它仍然由 `s` 拥有。
That's why `slice` doesn't have a `capacity` field: it doesn't own the data, so it doesn't need to know how much
space it was allocated for it; it only cares about the data it references.
这就是为什么 `slice` 没有 `capacity` 字段：它不拥有数据，所以它不需要知道为它分配了多少空间；它只关心它引用的数据。

## `&str` vs `&String`
## `&str` 与 `&String`

As a rule of thumb, use `&str` rather than `&String` whenever you need a reference to textual data.\
根据经验法则，当你需要引用文本数据时，使用 `&str` 而不是 `&String`。\
`&str` is more flexible and generally considered more idiomatic in Rust code.
`&str` 更灵活，通常被认为在 Rust 代码中更地道。

If a method returns a `&String`, you're promising that there is heap-allocated UTF-8 text somewhere that
**matches exactly** the one you're returning a reference to.\
如果一个方法返回 `&String`，你就是在承诺在某个地方有堆分配的 UTF-8 文本，该文本**完全匹配**你返回引用的那个。\
If a method returns a `&str`, instead, you have a lot more freedom: you're just saying that _somewhere_ there's a
bunch of text data and that a subset of it matches what you need, therefore you're returning a reference to it.
相反，如果方法返回 `&str`，你就有更多的自由：你只是说_某个地方_有一堆文本数据，其中一部分匹配你的需要，因此你返回对其的引用。
