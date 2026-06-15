# `.iter()`
# `.iter()`

`IntoIterator` **consumes** `self` to create an iterator.
`IntoIterator` **消耗** `self` 来创建迭代器。

This has its benefits: you get **owned** values from the iterator.
这有它的好处：你从迭代器获得**拥有的**值。
For example: if you call `.into_iter()` on a `Vec<Ticket>` you'll get an iterator that returns `Ticket` values.
例如：如果你在 `Vec<Ticket>` 上调用 `.into_iter()`，你会得到一个返回 `Ticket` 值的迭代器。

That's also its downside: you can no longer use the original collection after calling `.into_iter()` on it.
这也是它的缺点：在调用 `.into_iter()` 后，你不能再使用原始集合。
Quite often you want to iterate over a collection without consuming it, looking at **references** to the values instead.
通常情况下，你想要在不消耗集合的情况下进行迭代，而是查看值的**引用**。
In the case of `Vec<Ticket>`, you'd want to iterate over `&Ticket` values.
对于 `Vec<Ticket>`，你会想要迭代 `&Ticket` 值。

Most collections expose a method called `.iter()` that returns an iterator over references to the collection's elements.
大多数集合都暴露了一个名为 `.iter()` 的方法，它返回一个对集合元素引用的迭代器。
For example:
例如：

```rust
let numbers: Vec<u32> = vec![1, 2];
// `n` has type `&u32` here
for n in numbers.iter() {
    // [...]
}
```

This pattern can be simplified by implementing `IntoIterator` for a **reference to the collection**.
这种模式可以通过为**对集合的引用**实现 `IntoIterator` 来简化。
In our example above, that would be `&Vec<Ticket>`.\
在我们上面的例子中，那就是 `&Vec<Ticket>`。\
The standard library does this, that's why the following code works:
标准库实现了这一点，这就是为什么以下代码可以工作：

```rust
let numbers: Vec<u32> = vec![1, 2];
// `n` has type `&u32` here
// We didn't have to call `.iter()` explicitly
// It was enough to use `&numbers` in the `for` loop
for n in &numbers {
    // [...]
}
```

It's idiomatic to provide both options:
提供两种选项是地道的做法：

- An implementation of `IntoIterator` for a reference to the collection.
- 为对集合的引用实现 `IntoIterator`。
- An `.iter()` method that returns an iterator over references to the collection's elements.
- 一个返回对集合元素引用的迭代器的 `.iter()` 方法。

The former is convenient in `for` loops, the latter is more explicit and can be used in other contexts.
前者在 `for` 循环中很方便，后者更显式，可以在其他上下文中使用。
