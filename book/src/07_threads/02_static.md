# `'static`
# `'static`

If you tried to borrow a slice from the vector in the previous exercise,
you probably got a compiler error that looks something like this:
如果你尝试在前一个练习中从 vector 中借用切片，你可能会看到类似这样的编译错误：

```text
error[E0597]: `v` does not live long enough
   |
11 | pub fn sum(v: Vec<i32>) -> i32 {
   |            - binding `v` declared here
...
15 |     let right = &v[split_point..];
   |                  ^ borrowed value does not live long enough
16 |     let left_handle = spawn(move || left.iter().sum::<i32>());
   |                             -------------------------------- 
                     argument requires that `v` is borrowed for `'static`
19 | }
   |  - `v` dropped here while still borrowed
```

`argument requires that v is borrowed for 'static`, what does that mean?
`argument requires that v is borrowed for 'static`，这是什么意思？

The `'static` lifetime is a special lifetime in Rust.\
`'static` 生命周期是 Rust 中一个特殊的生命周期。\
It means that the value will be valid for the entire duration of the program.
它意味着该值在整个程序运行期间都是有效的。

## Detached threads
## 分离线程

A thread launched via `thread::spawn` can **outlive** the thread that spawned it.\
通过 `thread::spawn` 启动的线程可以**比**启动它的线程**存活得更久**。\
For example:
例如：

```rust
use std::thread;

fn f() {
    thread::spawn(|| {
        thread::spawn(|| {
            loop {
                thread::sleep(std::time::Duration::from_secs(1));
                println!("Hello from the detached thread!");
            }
        });
    });
}
```

In this example, the first spawned thread will in turn spawn
a child thread that prints a message every second.\
在这个例子中，第一个被生成的线程又会生成一个每秒打印一条消息的子线程。\
The first thread will then finish and exit. When that happens,
its child thread will **continue running** for as long as the
overall process is running.\
然后第一个线程会完成并退出。此时，它的子线程会**继续运行**，直到整个进程结束。\
In Rust's lingo, we say that the child thread has **outlived**
its parent.
用 Rust 的说法，我们说子线程**比**它的父线程**活得久**。

## `'static` lifetime
## `'static` 生命周期

Since a spawned thread can:
由于被生成的线程可以：

- outlive the thread that spawned it (its parent thread)
- 比生成它的线程（其父线程）存活得更久
- run until the program exits
- 一直运行到程序退出

it must not borrow any values that might be dropped before the program exits;
violating this constraint would expose us to a use-after-free bug.\
它不能借用任何可能在程序退出前被丢弃的值；违反这个约束会导致 use-after-free 错误。\
That's why `std::thread::spawn`'s signature requires that the closure passed to it
has the `'static` lifetime:
这就是为什么 `std::thread::spawn` 的签名要求传递给它的闭包具有 `'static` 生命周期：

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T> 
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static
{
    // [..]
}
```

## `'static` is not (just) about references
## `'static` 不仅仅与引用有关

All values in Rust have a lifetime, not just references.
Rust 中所有值都有生命周期，不仅仅是引用。

In particular, a type that owns its data (like a `Vec` or a `String`)
satisfies the `'static` constraint: if you own it, you can keep working with it
for as long as you want, even after the function that originally created it
has returned.
特别地，一个拥有其数据的类型（如 `Vec` 或 `String`）满足 `'static` 约束：如果你拥有它，你可以任意长时间地使用它，即使最初创建它的函数已经返回。

You can thus interpret `'static` as a way to say:
因此你可以将 `'static` 理解为一种方式来表达：

- Give me an owned value
- 给我一个拥有的值
- Give me a reference that's valid for the entire duration of the program
- 给我一个在整个程序运行期间有效的引用

The first approach is how you solved the issue in the previous exercise:
by allocating new vectors to hold the left and right parts of the original vector,
which were then moved into the spawned threads.
第一种方法就是你解决前一个练习问题的方式：分配新的 vector 来持有原始 vector 的左右两部分，然后将其移动到生成的线程中。

## `'static` references
## `'static` 引用

Let's talk about the second case, references that are valid for the entire
duration of the program.
让我们来谈谈第二种情况，即在整个程序运行期间有效的引用。

### Static data
### 静态数据

The most common case is a reference to **static data**, such as string literals:
最常见的情况是对**静态数据**的引用，例如字符串字面量：

```rust
let s: &'static str = "Hello world!";
```

Since string literals are known at compile-time, Rust stores them _inside_ your executable,
in a region known as **read-only data segment**.
由于字符串字面量在编译期已知，Rust 将它们存储在可执行文件的**只读数据段**中。
All references pointing to that region will therefore be valid for as long as
the program runs; they satisfy the `'static` contract.
因此，指向该区域的所有引用在程序运行期间都是有效的；它们满足 `'static` 约定。

## Further reading
## 延伸阅读

- [The data segment](https://en.wikipedia.org/wiki/Data_segment)
- [数据段](https://en.wikipedia.org/wiki/Data_segment)
