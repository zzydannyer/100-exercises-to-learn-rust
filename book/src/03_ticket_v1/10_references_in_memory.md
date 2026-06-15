# References
# 引用

What about references, like `&String` or `&mut String`? How are they represented in memory?
那引用呢，比如 `&String` 或 `&mut String`？它们在内存中如何表示？

Most references[^fat] in Rust are represented, in memory, as a pointer to a memory location.\
Rust 中的大多数引用[^fat]在内存中表示为指向某个内存位置的指针。\
It follows that their size is the same as the size of a pointer, a `usize`.
因此它们的大小与指针的大小相同，即 `usize`。

You can verify this using `std::mem::size_of`:
你可以使用 `std::mem::size_of` 来验证：

```rust
assert_eq!(std::mem::size_of::<&String>(), 8);
assert_eq!(std::mem::size_of::<&mut String>(), 8);
```

A `&String`, in particular, is a pointer to the memory location where the `String`'s metadata is stored.\
具体来说，`&String` 是一个指向存储 `String` 元数据的内存位置的指针。\
If you run this snippet:
如果你运行这段代码：

```rust
let s = String::from("Hey");
let r = &s;
```

you'll get something like this in memory:
在内存中你会得到类似这样的布局：

```
           --------------------------------------
           |                                    |
      +----v----+--------+----------+      +----|----+
Stack | pointer | length | capacity |      | pointer |
      |  |      |   3    |    5     |      |         |
      +--|  ----+--------+----------+      +---------+
         |          s                           r
         |
         v
       +---+---+---+---+---+
Heap   | H | e | y | ? | ? |
       +---+---+---+---+---+
```

It's a pointer to a pointer to the heap-allocated data, if you will.
如果你愿意，可以把它看作指向堆分配数据的指针的指针。
The same goes for `&mut String`.
`&mut String` 也一样。

## Not all pointers point to the heap
## 并非所有指针都指向堆

The example above should clarify one thing: not all pointers point to the heap.\
上面的例子应该澄清了一件事：并非所有指针都指向堆。\
They just point to a memory location, which _may_ be on the heap, but doesn't have to be.
它们只是指向一个内存位置，该位置_可能_在堆上，但不一定。

[^fat]: [Later in the course](../04_traits/06_str_slice.md) we'll talk about **fat pointers**,
i.e. pointers with additional metadata. As the name implies, they are larger than
the pointers we discussed in this chapter, also known as **thin pointers**.
[^fat]: 在[课程后面](../04_traits/06_str_slice.md)，我们将讨论**胖指针(fat pointers)**，即带有附加元数据的指针。顾名思义，它们比我们在本章中讨论的指针（也称为**瘦指针(thin pointers)**）更大。
