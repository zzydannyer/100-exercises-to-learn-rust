# Nullability
# 可空性

Our implementation of the `assigned` method is fairly blunt: panicking for to-do and done tickets is far from ideal.\
我们对 `assigned` 方法的实现相当生硬：对待办和已完成的票据进行恐慌远非理想。\
We can do better using **Rust's `Option` type**.
我们可以使用 **Rust 的 `Option` 类型**做得更好。

## `Option`
## `Option`

`Option` is a Rust type that represents **nullable values**.\
`Option` 是一种表示**可空值**的 Rust 类型。\
It is an enum, defined in Rust's standard library:
它是一个枚举，定义在 Rust 的标准库中：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option` encodes the idea that a value might be present (`Some(T)`) or absent (`None`).\
`Option` 编码了值可能存在（`Some(T)`）或不存在（`None`）的概念。\
It also forces you to **explicitly handle both cases**. You'll get a compiler error if you are working with
a nullable value and you forget to handle the `None` case.\
它还强制你**显式处理两种情况**。如果你正在处理一个可空值，并且忘记了处理 `None` 情况，你会收到一个编译器错误。\
This is a significant improvement over "implicit" nullability in other languages, where you can forget to check
for `null` and thus trigger a runtime error.
这比其他语言中的"隐式"可空性有了显著改进，在其他语言中，你可能会忘记检查 `null`，从而触发运行时错误。

## `Option`'s definition
## `Option` 的定义

`Option`'s definition uses a Rust construct that you haven't seen before: **tuple-like variants**.
`Option` 的定义使用了一个你之前没见过的 Rust 构造：**元组类变体**。

### Tuple-like variants
### 元组类变体

`Option` has two variants: `Some(T)` and `None`.\
`Option` 有两个变体：`Some(T)` 和 `None`。\
`Some` is a **tuple-like variant**: it's a variant that holds **unnamed fields**.
`Some` 是一个**元组类变体**：它是一个持有**未命名字段**的变体。

Tuple-like variants are often used when there is a single field to store, especially when we're looking at a
"wrapper" type like `Option`.
当只有一个字段需要存储时，通常使用元组类变体，特别是当我们看到一个像 `Option` 这样的"包装器"类型时。

### Tuple-like structs
### 元组类结构体

They're not specific to enums—you can define tuple-like structs too:
它们并不是枚举特有的——你也可以定义元组类结构体：

```rust
struct Point(i32, i32);
```

You can then access the two fields of a `Point` instance using their positional index:
然后你可以使用它们的位置索引访问 `Point` 实例的两个字段：

```rust
let point = Point(3, 4);
let x = point.0;
let y = point.1;
```

### Tuples
### 元组

It's weird to say that something is tuple-like when we haven't seen tuples yet!\
当我们还没见过元组时，说某物是元组类是很奇怪的！\
Tuples are another example of a primitive Rust type.
元组是另一种 Rust 基本类型的例子。
They group together a fixed number of values with (potentially different) types:
它们将固定数量的值（可能具有不同类型）组合在一起：

```rust
// Two values, same type
let first: (i32, i32) = (3, 4);
// Three values, different types
let second: (i32, u32, u8) = (-42, 3, 8);
```

The syntax is simple: you list the types of the values between parentheses, separated by commas.
语法很简单：你在圆括号中列出值的类型，用逗号分隔。
You can access the fields of a tuple using the dot notation and the field index:
你可以使用点符号和字段索引访问元组的字段：

```rust
assert_eq!(second.0, -42);
assert_eq!(second.1, 3);
assert_eq!(second.2, 8);
```

Tuples are a convenient way of grouping values together when you can't be bothered to define a dedicated struct type.
当你懒得定义专门的结构体类型时，元组是一种将值分组在一起的便捷方式。
