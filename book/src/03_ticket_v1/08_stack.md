# Memory layout
# 内存布局

We've looked at ownership and references from an operational point of view—what you can and can't do with them.
我们已经从操作的角度看了所有权和引用——你可以用它们做什么，不能做什么。
Now it's a good time to take a look under the hood: let's talk about **memory**.
现在是时候深入了解一下内部机制了：让我们谈谈**内存**。

## Stack and heap
## 栈和堆

When discussing memory, you'll often hear people talk about the **stack** and the **heap**.\
在讨论内存时，你经常听到人们谈论**栈**和**堆**。\
These are two different memory regions used by programs to store data.
这是程序用来存储数据的两个不同内存区域。

Let's start with the stack.
让我们从栈开始。

## Stack
## 栈

The **stack** is a **LIFO** (Last In, First Out) data structure.\
**栈**是一种 **LIFO**（后进先出）数据结构。\
When you call a function, a new **stack frame** is added on top of the stack. That stack frame stores
the function's arguments, local variables and a few "bookkeeping" values.\
当你调用函数时，一个新的**栈帧**被添加到栈的顶部。该栈帧存储函数的参数、局部变量和一些"簿记"值。\
When the function returns, the stack frame is popped off the stack[^stack-overflow].
当函数返回时，栈帧从栈中弹出[^stack-overflow]。

```text
+-----------------+
| frame for func1 |
+-----------------+
        |
        | func2 is 
        | called
        v
+-----------------+
| frame for func2 |
+-----------------+
| frame for func1 |
+-----------------+
        |
        | func2  
        | returns
        v
+-----------------+
| frame for func1 |
+-----------------+
```

From an operational point of view, stack allocation/de-allocation is **very fast**.\
从操作的角度来看，栈分配/释放**非常快**。\
We are always pushing and popping data from the top of the stack, so we don't need to search for free memory.
我们总是从栈顶推入和弹出数据，所以我们不需要搜索空闲内存。
We also don't have to worry about fragmentation: the stack is a single contiguous block of memory.
我们也不需要担心碎片化：栈是一个单一的连续内存块。

### Rust
### Rust

Rust will often allocate data on the stack.\
Rust 通常会在栈上分配数据。\
You have a `u32` input argument in a function? Those 32 bits will be on the stack.\
函数中有一个 `u32` 输入参数？那 32 位会在栈上。\
You define a local variable of type `i64`? Those 64 bits will be on the stack.\
你定义一个 `i64` 类型的局部变量？那 64 位会在栈上。\
It all works quite nicely because the size of those integers is known at compile time, therefore
the compiled program knows how much space it needs to reserve on the stack for them.
这一切都很顺利，因为这些整数的大小在编译时就已知了，因此编译后的程序知道它需要在栈上为它们预留多少空间。

### `std::mem::size_of`
### `std::mem::size_of`

You can verify how much space a type would take on the stack
using the [`std::mem::size_of`](https://doc.rust-lang.org/std/mem/fn.size_of.html) function.
你可以使用 [`std::mem::size_of`](https://doc.rust-lang.org/std/mem/fn.size_of.html) 函数验证一个类型在栈上会占用多少空间。

For a `u8`, for example:
例如，对于 `u8`：

```rust
// We'll explain this funny-looking syntax (`::<u8>`) later on.
// Ignore it for now.
assert_eq!(std::mem::size_of::<u8>(), 1);
```

1 makes sense, because a `u8` is 8 bits long, or 1 byte.
1 是合理的，因为一个 `u8` 是 8 位长，即 1 字节。

[^stack-overflow]: If you have nested function calls, each function pushes its data onto the stack when it's called but
it doesn't pop it off until the innermost function returns.
[^stack-overflow]: 如果你有嵌套函数调用，每个函数在被调用时将其数据推入栈，但直到最内层函数返回时才将其弹出。
If you have too many nested function calls, you can run out of stack space—the stack is not infinite!
如果你有太多嵌套函数调用，你可能会耗尽栈空间——栈不是无限的！
That's called a [**stack overflow**](https://en.wikipedia.org/wiki/Stack_overflow).
这被称为[**栈溢出**](https://en.wikipedia.org/wiki/Stack_overflow)。
