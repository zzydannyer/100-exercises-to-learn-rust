# Resizing
# 调整大小

We said that `Vec` is a "growable" vector type, but what does that mean?
我们说过 `Vec` 是一个"可增长"的向量类型，但这意味着什么？
What happens if you try to insert an element into a `Vec` that's already at maximum capacity?
如果你尝试向一个已经达到最大容量的 `Vec` 中插入一个元素会发生什么？

```rust
let mut numbers = Vec::with_capacity(3);
numbers.push(1);
numbers.push(2);
numbers.push(3); // Max capacity reached
numbers.push(4); // What happens here?
```

The `Vec` will **resize** itself.\
`Vec` 会**调整**自身大小。\
It will ask the allocator for a new (larger) chunk of heap memory, copy the elements over, and deallocate the old memory.
它会向分配器请求一块新的（更大的）堆内存，将元素复制过去，然后释放旧内存。

This operation can be expensive, as it involves a new memory allocation and copying all existing elements.
这个操作可能很昂贵，因为它涉及新的内存分配和复制所有现有元素。

## `Vec::with_capacity`
## `Vec::with_capacity`

If you have a rough idea of how many elements you'll store in a `Vec`, you can use the `Vec::with_capacity`
method to pre-allocate enough memory upfront.\
如果你大致知道要在 `Vec` 中存储多少个元素，你可以使用 `Vec::with_capacity` 方法预先分配足够的内存。\
This can avoid a new allocation when the `Vec` grows, but it may waste memory if you overestimate actual usage.
这可以避免在 `Vec` 增长时进行新的分配，但如果你高估了实际使用量，可能会浪费内存。

Evaluate on a case-by-case basis.
请根据具体情况评估。
