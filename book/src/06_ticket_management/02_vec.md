# Vectors
# 向量

Arrays' strength is also their weakness: their size must be known upfront, at compile-time.
数组的优点也是它们的弱点：它们的大小必须在编译时预先知道。
If you try to create an array with a size that's only known at runtime, you'll get a compilation error:
如果你尝试创建一个大小仅在运行时才知道的数组，你会得到一个编译错误：

```rust
let n = 10;
let numbers: [u32; n];
```

```text
error[E0435]: attempt to use a non-constant value in a constant
 --> src/main.rs:3:20
  |
2 | let n = 10;
3 | let numbers: [u32; n];
  |                    ^ non-constant value
```

Arrays wouldn't work for our ticket management system—we don't know how many tickets we'll need to store at compile-time.
数组不适用于我们的票据管理系统——我们在编译时不知道需要存储多少张票据。
This is where `Vec` comes in.
这就是 `Vec` 的用武之地。

## `Vec`
## `Vec`

`Vec` is a growable array type, provided by the standard library.\
`Vec` 是一个可增长的数组类型，由标准库提供。\
You can create an empty array using the `Vec::new` function:
你可以使用 `Vec::new` 函数创建一个空数组：

```rust
let mut numbers: Vec<u32> = Vec::new();
```

You would then push elements into the vector using the `push` method:
然后你可以使用 `push` 方法将元素推入向量：

```rust
numbers.push(1);
numbers.push(2);
numbers.push(3);
```

New values are added to the end of the vector.\
新值被添加到向量的末尾。\
You can also create an initialized vector using the `vec!` macro, if you know the values at creation time:
如果你在创建时就知道值，你也可以使用 `vec!` 宏创建一个已初始化的向量：

```rust
let numbers = vec![1, 2, 3];
```

## Accessing elements
## 访问元素

The syntax for accessing elements is the same as with arrays:
访问元素的语法与数组相同：

```rust
let numbers = vec![1, 2, 3];
let first = numbers[0];
let second = numbers[1];
let third = numbers[2];
```

The index must be of type `usize`.\
索引的类型必须是 `usize`。\
You can also use the `get` method, which returns an `Option<&T>`:
你也可以使用 `get` 方法，它返回一个 `Option<&T>`：

```rust
let numbers = vec![1, 2, 3];
assert_eq!(numbers.get(0), Some(&1));
// You get a `None` if you try to access an out-of-bounds index
// rather than a panic.
assert_eq!(numbers.get(3), None);
```

Access is bounds-checked, just like element access with arrays. It has O(1) complexity.
访问是有边界检查的，就像使用数组访问元素一样。它具有 O(1) 的复杂度。

## Memory layout
## 内存布局

`Vec` is a heap-allocated data structure.\
`Vec` 是一种堆分配的数据结构。\
When you create a `Vec`, it allocates memory on the heap to store the elements.
当你创建一个 `Vec` 时，它会在堆上分配内存来存储元素。

If you run the following code:
如果你运行以下代码：

```rust
let mut numbers = Vec::with_capacity(3);
numbers.push(1);
numbers.push(2);
```

you'll get the following memory layout:
你将得到以下内存布局：

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

`Vec` keeps track of three things:
`Vec` 跟踪三件事：

- The **pointer** to the heap region you reserved.
- 指向你保留的堆区域的**指针**。
- The **length** of the vector, i.e. how many elements are in the vector.
- 向量的**长度**，即向量中有多少个元素。
- The **capacity** of the vector, i.e. the number of elements that can fit in the space reserved on the heap.
- 向量的**容量**，即堆上保留的空间可以容纳的元素数量。

This layout should look familiar: it's exactly the same as `String`!\
这种布局应该看起来很熟悉：它与 `String` 完全相同！\
That's not a coincidence: `String` is defined as a vector of bytes, `Vec<u8>`, under the hood:
这并非巧合：`String` 在底层被定义为字节向量 `Vec<u8>`：

```rust
pub struct String {
    vec: Vec<u8>,
}
```
