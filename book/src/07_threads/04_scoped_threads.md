# Scoped threads
# 作用域线程

All the lifetime issues we discussed so far have a common source:
the spawned thread can outlive its parent.\
我们到目前为止讨论的所有生命周期问题都有一个共同的根源：生成的线程可以比其父线程存活得更久。\
We can sidestep this issue by using **scoped threads**.
我们可以通过使用**作用域线程**来规避这个问题。

```rust
let v = vec![1, 2, 3];
let midpoint = v.len() / 2;

std::thread::scope(|scope| {
    scope.spawn(|| {
        let first = &v[..midpoint];
        println!("Here's the first half of v: {first:?}");
    });
    scope.spawn(|| {
        let second = &v[midpoint..];
        println!("Here's the second half of v: {second:?}");
    });
});

println!("Here's v: {v:?}");
```

Let's unpack what's happening.
让我们来解析一下这里发生的事情。

## `scope`
## `scope`

The `std::thread::scope` function creates a new **scope**.\
`std::thread::scope` 函数创建一个新的**作用域**。\
`std::thread::scope` takes a closure as input, with a single argument: a `Scope` instance.
`std::thread::scope` 接受一个闭包作为输入，该闭包有一个参数：一个 `Scope` 实例。

## Scoped spawns
## 作用域生成

`Scope` exposes a `spawn` method.\
`Scope` 公开了一个 `spawn` 方法。\
Unlike `std::thread::spawn`, all threads spawned using a `Scope` will be
**automatically joined** when the scope ends.
与 `std::thread::spawn` 不同，所有使用 `Scope` 生成的线程将在作用域结束时被**自动 join**。

If we were to "translate" the previous example to `std::thread::spawn`,
it'd look like this:
如果我们把前面的例子"翻译"成 `std::thread::spawn`，会是这样的：

```rust
let v = vec![1, 2, 3];
let midpoint = v.len() / 2;

let handle1 = std::thread::spawn(|| {
    let first = &v[..midpoint];
    println!("Here's the first half of v: {first:?}");
});
let handle2 = std::thread::spawn(|| {
    let second = &v[midpoint..];
    println!("Here's the second half of v: {second:?}");
});

handle1.join().unwrap();
handle2.join().unwrap();

println!("Here's v: {v:?}");
```

## Borrowing from the environment
## 从环境中借用

The translated example wouldn't compile, though: the compiler would complain
that `&v` can't be used from our spawned threads since its lifetime isn't
`'static`.
不过，翻译后的示例无法编译：编译器会报错，说 `&v` 不能在我们的生成线程中使用，因为它的生命周期不是 `'static`。

That's not an issue with `std::thread::scope`—you can **safely borrow from the environment**.
这对于 `std::thread::scope` 来说不是问题——你可以**安全地从环境中借用**。

In our example, `v` is created before the spawning points.
在我们的例子中，`v` 是在生成点之前创建的。
It will only be dropped _after_ `scope` returns. At the same time,
all threads spawned inside `scope` are guaranteed to finish _before_ `scope` returns,
therefore there is no risk of having dangling references.
它只会在 `scope` 返回_之后_被丢弃。同时，在 `scope` 内部生成的所有线程都保证在 `scope` 返回_之前_完成，因此不存在悬垂引用的风险。

The compiler won't complain!
编译器不会报错！
