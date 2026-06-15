# Mutable slices
# 可变切片

Every time we've talked about slice types (like `str` and `[T]`), we've used their immutable borrow form (`&str` and `&[T]`).\
每次我们讨论切片类型（如 `str` 和 `[T]`）时，我们都使用了它们的不可变借用形式（`&str` 和 `&[T]`）。\
But slices can also be mutable!
但切片也可以是可变的！

Here's how you create a mutable slice:
以下是你如何创建一个可变切片：

```rust
let mut numbers = vec![1, 2, 3];
let slice: &mut [i32] = &mut numbers;
```

You can then modify the elements in the slice:
然后你可以修改切片中的元素：

```rust
slice[0] = 42;
```

This will change the first element of the `Vec` to `42`.
这会将 `Vec` 的第一个元素改为 `42`。

## Limitations
## 限制

When working with immutable borrows, the recommendation was clear: prefer slice references over references to
the owned type (e.g. `&[T]` over `&Vec<T>`).\
当处理不可变借用时，建议很明确：优先选择切片引用而不是对拥有类型的引用（例如，`&[T]` 优于 `&Vec<T>`）。\
That's **not** the case with mutable borrows.
对于可变借用，情况**并非如此**。

Consider this scenario:
考虑这个场景：

```rust
let mut numbers = Vec::with_capacity(2);
let mut slice: &mut [i32] = &mut numbers;
slice.push(1);
```

It won't compile!\
它不会编译！\
`push` is a method on `Vec`, not on slices. This is the manifestation of a more general principle: Rust won't
allow you to add or remove elements from a slice. You will only be able to modify/replace the elements that are
already there.
`push` 是 `Vec` 的方法，而不是切片的方法。这是一个更一般原则的体现：Rust 不允许你向切片添加或从切片移除元素。你只能修改/替换已经存在的元素。

In this regard, a `&mut Vec` or a `&mut String` are strictly more powerful than a `&mut [T]` or a `&mut str`.\
在这方面，`&mut Vec` 或 `&mut String` 比 `&mut [T]` 或 `&mut str` 严格来说更强大。\
Choose the type that best fits based on the operations you need to perform.
根据你需要执行的操作选择最适合的类型。
