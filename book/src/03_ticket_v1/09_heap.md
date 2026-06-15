# Heap
# 堆

The stack is great, but it can't solve all our problems. What about data whose size is not known at compile time?
栈很好，但它不能解决我们所有的问题。那些大小在编译时未知的数据呢？
Collections, strings, and other dynamically-sized data cannot be (entirely) stack-allocated.
集合、字符串和其他动态大小的数据不能（完全）在栈上分配。
That's where the **heap** comes in.
这时候**堆**就派上用场了。

## Heap allocations
## 堆分配

You can visualize the heap as a big chunk of memory—a huge array, if you will.\
你可以把堆想象成一大块内存——如果你愿意，可以想象成一个巨大的数组。\
Whenever you need to store data on the heap, you ask a special program, the **allocator**, to reserve for you
a subset of the heap. We call this interaction (and the memory you reserved) a **heap allocation**.
每当你需要在堆上存储数据时，你请求一个特殊的程序——**分配器(allocator)**——为你保留堆的一个子集。我们把这种交互（以及你保留的内存）称为**堆分配**。
If the allocation succeeds, the allocator will give you a **pointer** to the start of the reserved block.
如果分配成功，分配器会给你一个指向保留块起始位置的**指针**。

## No automatic de-allocation
## 无自动释放

The heap is structured quite differently from the stack.\
堆的结构与栈有很大不同。\
Heap allocations are not contiguous, they can be located anywhere inside the heap.
堆分配不是连续的，它们可以位于堆内的任何位置。

```
+---+---+---+---+---+---+-...-+-...-+---+---+---+---+---+---+---+
|  Allocation 1 | Free  | ... | ... |  Allocation N |    Free   |
+---+---+---+---+---+---+ ... + ... +---+---+---+---+---+---+---+
```

It's the allocator's job to keep track of which parts of the heap are in use and which are free.
跟踪堆的哪些部分正在使用、哪些是空闲的，这是分配器的工作。
The allocator won't automatically free the memory you allocated, though: you need to be deliberate about it,
calling the allocator again to **free** the memory you no longer need.
不过，分配器不会自动释放你分配的内存：你需要刻意地再次调用分配器来**释放**你不再需要的内存。

## Performance
## 性能

The heap's flexibility comes at a cost: heap allocations are **slower** than stack allocations.
堆的灵活性是有代价的：堆分配比栈分配**更慢**。
There's a lot more bookkeeping involved!\
涉及更多的簿记工作！\
If you read articles about performance optimization you'll often be advised to minimize heap allocations
and prefer stack-allocated data whenever possible.
如果你阅读关于性能优化的文章，你经常会被建议最小化堆分配，并尽可能优先使用栈分配的数据。

## `String`'s memory layout
## `String` 的内存布局

When you create a local variable of type `String`,
Rust is forced to allocate on the heap[^empty]: it doesn't know in advance how much text you're going to put in it,
so it can't reserve the right amount of space on the stack.\
当你创建一个 `String` 类型的局部变量时，Rust 被迫在堆上分配[^empty]：它无法预先知道你要在其中放多少文本，因此无法在栈上预留正确的空间量。\
But a `String` is not _entirely_ heap-allocated, it also keeps some data on the stack. In particular:
但是 `String` 并不_完全_是在堆上分配的，它也在栈上保留一些数据。具体来说：

- The **pointer** to the heap region you reserved.
- 指向你保留的堆区域的**指针**。
- The **length** of the string, i.e. how many bytes are in the string.
- 字符串的**长度**，即字符串中有多少字节。
- The **capacity** of the string, i.e. how many bytes have been reserved on the heap.
- 字符串的**容量**，即在堆上保留了多少字节。

Let's look at an example to understand this better:
让我们看一个例子来更好地理解：

```rust
let mut s = String::with_capacity(5);
```

If you run this code, memory will be laid out like this:
如果你运行这段代码，内存布局如下：

```
      +---------+--------+----------+
Stack | pointer | length | capacity | 
      |  |      |   0    |    5     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+---+---+
Heap:  | ? | ? | ? | ? | ? |
       +---+---+---+---+---+
```

We asked for a `String` that can hold up to 5 bytes of text.\
我们请求一个最多可容纳 5 字节文本的 `String`。\
`String::with_capacity` goes to the allocator and asks for 5 bytes of heap memory. The allocator returns
a pointer to the start of that memory block.\
`String::with_capacity` 向分配器请求 5 字节的堆内存。分配器返回一个指向该内存块起始位置的指针。\
The `String` is empty, though. On the stack, we keep track of this information by distinguishing between
the length and the capacity: this `String` can hold up to 5 bytes, but it currently holds 0 bytes of
actual text.
不过这个 `String` 是空的。在栈上，我们通过区分长度和容量来跟踪这些信息：这个 `String` 最多可以容纳 5 字节，但当前包含 0 字节的实际文本。

If you push some text into the `String`, the situation will change:
如果你向 `String` 中推入一些文本，情况就会改变：

```rust
s.push_str("Hey");
```

```
      +---------+--------+----------+
Stack | pointer | length | capacity |
      |  |      |   3    |    5     |
      +--|  ----+--------+----------+
         |
         |
         v
       +---+---+---+---+---+
Heap:  | H | e | y | ? | ? |
       +---+---+---+---+---+
```

`s` now holds 3 bytes of text. Its length is updated to 3, but capacity remains 5.
`s` 现在包含 3 字节的文本。其长度更新为 3，但容量保持为 5。
Three of the five bytes on the heap are used to store the characters `H`, `e`, and `y`.
堆上的五个字节中有三个用于存储字符 `H`、`e` 和 `y`。

### `usize`
### `usize`

How much space do we need to store pointer, length and capacity on the stack?\
我们需要在栈上存储指针、长度和容量各需要多少空间？\
It depends on the **architecture** of the machine you're running on.
这取决于你运行的机器的**架构**。

Every memory location on your machine has an [**address**](https://en.wikipedia.org/wiki/Memory_address), commonly
represented as an unsigned integer.
你机器上的每个内存位置都有一个[**地址**](https://en.wikipedia.org/wiki/Memory_address)，通常表示为一个无符号整数。
Depending on the maximum size of the address space (i.e. how much memory your machine can address),
this integer can have a different size. Most modern machines use either a 32-bit or a 64-bit address space.
根据地址空间的最大大小（即你的机器可以寻址多少内存），这个整数可以有不同的大小。大多数现代机器使用 32 位或 64 位的地址空间。

Rust abstracts away these architecture-specific details by providing the `usize` type:
an unsigned integer that's as big as the number of bytes needed to address memory on your machine.
Rust 通过提供 `usize` 类型来抽象这些架构特定的细节：这是一个无符号整数，其大小与在你的机器上寻址内存所需的字节数相同。
On a 32-bit machine, `usize` is equivalent to `u32`. On a 64-bit machine, it matches `u64`.
在 32 位机器上，`usize` 相当于 `u32`。在 64 位机器上，它等于 `u64`。

Capacity, length and pointers are all represented as `usize`s in Rust[^equivalence].
在 Rust 中，容量、长度和指针都表示为 `usize`[^equivalence]。

### No `std::mem::size_of` for the heap
### 堆没有 `std::mem::size_of`

`std::mem::size_of` returns the amount of space a type would take on the stack,
which is also known as the **size of the type**.
`std::mem::size_of` 返回一个类型在栈上占用的空间量，这也称为**类型的大小**。

> What about the memory buffer that `String` is managing on the heap? Isn't that
> part of the size of `String`?
> 那 `String` 在堆上管理的内存缓冲区呢？那不是 `String` 大小的一部分吗？

No!\
不是！\
That heap allocation is a **resource** that `String` is managing.
那个堆分配是 `String` 正在管理的**资源**。
It's not considered to be part of the `String` type by the compiler.
编译器不认为它是 `String` 类型的一部分。

`std::mem::size_of` doesn't know (or care) about additional heap-allocated data
that a type might manage or refer to via pointers, as is the case with `String`,
therefore it doesn't track its size.
`std::mem::size_of` 不知道（也不关心）类型可能管理或通过指针引用的额外堆分配数据，就像 `String` 的情况一样，因此它不跟踪其大小。

Unfortunately there is no equivalent of `std::mem::size_of` to measure the amount of
heap memory that a certain value is allocating at runtime. Some types might
provide methods to inspect their heap usage (e.g. `String`'s `capacity` method),
but there is no general-purpose "API" to retrieve runtime heap usage in Rust.\
不幸的是，没有等同于 `std::mem::size_of` 的方法来测量某个值在运行时分配的堆内存量。某些类型可能提供检查其堆使用情况的方法（例如 `String` 的 `capacity` 方法），但在 Rust 中没有通用的"API"来检索运行时堆使用情况。\
You can, however, use a memory profiler tool (e.g. [DHAT](https://valgrind.org/docs/manual/dh-manual.html)
or [a custom allocator](https://docs.rs/dhat/latest/dhat/)) to inspect the heap usage of your program.
不过，你可以使用内存分析工具（例如 [DHAT](https://valgrind.org/docs/manual/dh-manual.html) 或[自定义分配器](https://docs.rs/dhat/latest/dhat/)）来检查程序的堆使用情况。

[^empty]: `std` doesn't allocate if you create an **empty** `String` (i.e. `String::new()`).
[^empty]: 如果你创建一个**空的** `String`（即 `String::new()`），`std` 不会分配内存。
Heap memory will be reserved when you push data into it for the first time.
当你首次向其中推入数据时，才会保留堆内存。

[^equivalence]: The size of a pointer depends on the operating system too.
[^equivalence]: 指针的大小也取决于操作系统。
In certain environments, a pointer is **larger** than a memory address (e.g. [CHERI](https://web.archive.org/web/20240517051950/https://blog.acolyer.org/2019/05/28/cheri-abi/)).
在某些环境中，指针**大于**内存地址（例如 [CHERI](https://web.archive.org/web/20240517051950/https://blog.acolyer.org/2019/05/28/cheri-abi/)）。
Rust makes the simplifying assumption that pointers are the same size as memory addresses,
which is true for most modern systems you're likely to encounter.
Rust 做了一个简化假设，即指针与内存地址大小相同，这对于你可能遇到的大多数现代系统来说都是成立的。
