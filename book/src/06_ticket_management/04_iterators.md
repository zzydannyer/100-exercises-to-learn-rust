# Iteration
# 迭代

During the very first exercises, you learned that Rust lets you iterate over collections using `for` loops.
在最初的练习中，你学到了 Rust 允许你使用 `for` 循环迭代集合。
We were looking at ranges at that point (e.g. `0..5`), but the same holds true for collections like arrays and vectors.
那时我们看的是范围（例如 `0..5`），但对于数组和向量等集合也是如此。

```rust
// It works for `Vec`s
let v = vec![1, 2, 3];
for n in v {
    println!("{}", n);
}

// It also works for arrays
let a: [u32; 3] = [1, 2, 3];
for n in a {
    println!("{}", n);
}
```

It's time to understand how this works under the hood.
是时候了解这在底层是如何工作的了。

## `for` desugaring
## `for` 的解糖

Every time you write a `for` loop in Rust, the compiler _desugars_ it into the following code:
每次你在 Rust 中编写 `for` 循环时，编译器都会将其_解糖_为以下代码：

```rust
let mut iter = IntoIterator::into_iter(v);
loop {
    match iter.next() {
        Some(n) => {
            println!("{}", n);
        }
        None => break,
    }
}
```

`loop` is another looping construct, on top of `for` and `while`.\
`loop` 是另一个循环构造，除了 `for` 和 `while` 之外。\
A `loop` block will run forever, unless you explicitly `break` out of it.
`loop` 块会永远运行，除非你显式地 `break` 跳出它。

## `Iterator` trait
## `Iterator` 特征

The `next` method in the previous code snippet comes from the `Iterator` trait.
The `Iterator` trait is defined in Rust's standard library and provides a shared interface for
types that can produce a sequence of values:
前面代码片段中的 `next` 方法来自 `Iterator` 特征。`Iterator` 特征定义在 Rust 的标准库中，并为可以产生值序列的类型提供了一个共享接口：

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

The `Item` associated type specifies the type of the values produced by the iterator.
`Item` 关联类型指定了迭代器产生的值的类型。

`next` returns the next value in the sequence.\
`next` 返回序列中的下一个值。\
It returns `Some(value)` if there's a value to return, and `None` when there isn't.
如果有值要返回，它返回 `Some(value)`，如果没有则返回 `None`。

Be careful: there is no guarantee that an iterator is exhausted when it returns `None`. That's only
guaranteed if the iterator implements the (more restrictive)
[`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html) trait.
小心：当迭代器返回 `None` 时，并不能保证它已被耗尽。只有当迭代器实现了（更严格的）[`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html) 特征时才能保证这一点。

## `IntoIterator` trait
## `IntoIterator` 特征

Not all types implement `Iterator`, but many can be converted into a type that does.\
并非所有类型都实现 `Iterator`，但许多类型可以转换为实现 `Iterator` 的类型。\
That's where the `IntoIterator` trait comes in:
这就是 `IntoIterator` 特征的用武之地：

```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}
```

The `into_iter` method consumes the original value and returns an iterator over its elements.\
`into_iter` 方法消耗原始值并返回对其元素的迭代器。\
A type can only have one implementation of `IntoIterator`: there can be no ambiguity as to what `for` should desugar to.
一个类型只能有一个 `IntoIterator` 的实现：对于 `for` 应该解糖成什么，不能有任何歧义。

One detail: every type that implements `Iterator` automatically implements `IntoIterator` as well.
They just return themselves from `into_iter`!
一个细节：每个实现了 `Iterator` 的类型也会自动实现 `IntoIterator`。它们只是从 `into_iter` 返回自身！

## Bounds checks
## 边界检查

Iterating over iterators has a nice side effect: you can't go out of bounds, by design.\
对迭代器进行迭代有一个很好的副作用：设计上，你不会越界。\
This allows Rust to remove bounds checks from the generated machine code, making iteration faster.
这允许 Rust 从生成的机器代码中移除边界检查，使迭代更快。

In other words,
换句话说，

```rust
let v = vec![1, 2, 3];
for n in v {
    println!("{}", n);
}
```

is usually faster than
通常比

```rust
let v = vec![1, 2, 3];
for i in 0..v.len() {
    println!("{}", v[i]);
}
```

更快

There are exceptions to this rule: the compiler can sometimes prove that you're not going out of bounds even
with manual indexing, thus removing the bounds checks anyway. But in general, prefer iteration to indexing
where possible.
这个规则也有例外：编译器有时可以证明即使使用手动索引你也不会越界，从而仍然移除边界检查。但一般来说，尽可能优先选择迭代而不是索引。
