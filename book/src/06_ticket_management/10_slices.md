# Slices
# 切片

Let's go back to the memory layout of a `Vec`:
让我们回到 `Vec` 的内存布局：

```rust
let mut numbers = Vec::with_capacity(3);
numbers.push(1);
numbers.push(2);
```

```text
      +---------+--------+----------+
Stack | pointer | length | capacity | 
      |  |      |   2    |    3     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+
Heap:  | 1 | 2 | ? |
       +---+---+---+
```

We already remarked how `String` is just a `Vec<u8>` in disguise.\
我们已经提到过 `String` 实际上就是伪装的 `Vec<u8>`。\
The similarity should prompt you to ask: "What's the equivalent of `&str` for `Vec`?"
这种相似性应该促使你问："`Vec` 对应的 `&str` 是什么？"

## `&[T]`
## `&[T]`

`[T]` is a **slice** of a contiguous sequence of elements of type `T`.\
`[T]` 是类型 `T` 的连续元素序列的**切片**。\
It's most commonly used in its borrowed form, `&[T]`.
它最常以借用形式 `&[T]` 使用。

There are various ways to create a slice reference from a `Vec`:
有几种方法可以从 `Vec` 创建切片引用：

```rust
let numbers = vec![1, 2, 3];
// Via index syntax
let slice: &[i32] = &numbers[..];
// Via a method
let slice: &[i32] = numbers.as_slice();
// Or for a subset of the elements
let slice: &[i32] = &numbers[1..];
```

`Vec` implements the `Deref` trait using `[T]` as the target type, so you can use slice methods on a `Vec` directly
thanks to deref coercion:
`Vec` 使用 `[T]` 作为目标类型实现了 `Deref` 特征，因此由于解引用强制转换，你可以直接在 `Vec` 上使用切片方法：

```rust
let numbers = vec![1, 2, 3];
// Surprise, surprise: `iter` is not a method on `Vec`!
// It's a method on `&[T]`, but you can call it on a `Vec` 
// thanks to deref coercion.
let sum: i32 = numbers.iter().sum();
```

### Memory layout
### 内存布局

A `&[T]` is a **fat pointer**, just like `&str`.\
`&[T]` 是一个**胖指针**，就像 `&str` 一样。\
It consists of a pointer to the first element of the slice and the length of the slice.
它由一个指向切片第一个元素的指针和切片的长度组成。

If you have a `Vec` with three elements:
如果你有一个包含三个元素的 `Vec`：

```rust
let numbers = vec![1, 2, 3];
```

and then create a slice reference:
然后创建一个切片引用：

```rust
let slice: &[i32] = &numbers[1..];
```

you'll get this memory layout:
你将得到这个内存布局：

```text
                  numbers                          slice
      +---------+--------+----------+      +---------+--------+
Stack | pointer | length | capacity |      | pointer | length |
      |    |    |   3    |    4     |      |    |    |   2    |
      +----|----+--------+----------+      +----|----+--------+
           |                                    |  
           |                                    |
           v                                    | 
         +---+---+---+---+                      |
Heap:    | 1 | 2 | 3 | ? |                      |
         +---+---+---+---+                      |
               ^                                |
               |                                |
               +--------------------------------+
```

### `&Vec<T>` vs `&[T]`
### `&Vec<T>` 与 `&[T]`

When you need to pass an immutable reference to a `Vec` to a function, prefer `&[T]` over `&Vec<T>`.\
当你需要将 `Vec` 的不可变引用传递给函数时，优先使用 `&[T]` 而不是 `&Vec<T>`。\
This allows the function to accept any kind of slice, not necessarily one backed by a `Vec`.
这允许函数接受任何类型的切片，不一定是 `Vec` 支持的切片。

For example, you can then pass a subset of the elements in a `Vec`.
例如，你可以传递 `Vec` 中一部分元素。
But it goes further than that—you could also pass a **slice of an array**:
但这不止于此——你还可以传递**数组的切片**：

```rust
let array = [1, 2, 3];
let slice: &[i32] = &array;
```

Array slices and `Vec` slices are the same type: they're fat pointers to a contiguous sequence of elements.
数组切片和 `Vec` 切片是同一类型：它们都是指向连续元素序列的胖指针。
In the case of arrays, the pointer points to the stack rather than the heap, but that doesn't matter
when it comes to using the slice.
对于数组，指针指向栈而不是堆，但这在使用切片时并不重要。
