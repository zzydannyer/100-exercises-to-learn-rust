# Threads
# 线程

Before we start writing multithreaded code, let's take a step back and talk about what threads are
and why we might want to use them.
在我们开始编写多线程代码之前，让我们退一步，讨论一下什么是线程以及为什么我们可能想要使用它们。

## What is a thread?
## 什么是线程？

A **thread** is an execution context managed by the underlying operating system.\
**线程**是由底层操作系统管理的执行上下文。\
Each thread has its own stack and instruction pointer.
每个线程都有自己的栈和指令指针。

A single **process** can manage multiple threads.
一个**进程**可以管理多个线程。
These threads share the same memory space, which means they can access the same data.
这些线程共享相同的内存空间，这意味着它们可以访问相同的数据。

Threads are a **logical** construct. In the end, you can only run one set of instructions
at a time on a CPU core, the **physical** execution unit.\
线程是一种**逻辑**构造。最终，你一次只能在一个 CPU 核心（**物理**执行单元）上运行一组指令。\
Since there can be many more threads than there are CPU cores, the operating system's
**scheduler** is in charge of deciding which thread to run at any given time,
partitioning CPU time among them to maximize throughput and responsiveness.
由于线程的数量可能远多于 CPU 核心的数量，操作系统的**调度器**负责决定在任何给定时间运行哪个线程，在它们之间分配 CPU 时间以最大化吞吐量和响应性。

## `main`
## `main`

When a Rust program starts, it runs on a single thread, the **main thread**.\
当 Rust 程序启动时，它运行在单个线程上，即**主线程**。\
This thread is created by the operating system and is responsible for running the `main`
function.
该线程由操作系统创建，负责运行 `main` 函数。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from the main thread!");
    }
}
```

## `std::thread`
## `std::thread`

Rust's standard library provides a module, `std::thread`, that allows you to create
and manage threads.
Rust 的标准库提供了一个模块 `std::thread`，允许你创建和管理线程。

### `spawn`
### `spawn`

You can use `std::thread::spawn` to create new threads and execute code on them.
你可以使用 `std::thread::spawn` 创建新线程并在其上执行代码。

For example:
例如：

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("Hello from a thread!");
        }
    });
    
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from the main thread!");
    }
}
```

If you execute this program on the [Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=afedf7062298ca8f5a248bc551062eaa)
you'll see that the main thread and the spawned thread run concurrently.\
如果你在 [Rust 游乐场](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=afedf7062298ca8f5a248bc551062eaa) 上执行这个程序，你会看到主线程和生成的线程并发运行。\
Each thread makes progress independently of the other.
每个线程独立于另一个线程取得进展。

### Process termination
### 进程终止

When the main thread finishes, the overall process will exit.\
当主线程完成时，整个进程将退出。\
A spawned thread will continue running until it finishes or the main thread finishes.
一个生成的线程将继续运行，直到它完成或主线程完成。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("Hello from a thread!");
        }
    });

    thread::sleep(Duration::from_secs(5));
}
```

In the example above, you can expect to see the message "Hello from a thread!" printed roughly five times.\
在上面的例子中，你大约可以看到"Hello from a thread!"消息被打印五次。\
Then the main thread will finish (when the `sleep` call returns), and the spawned thread will be terminated
since the overall process exits.
然后主线程将完成（当 `sleep` 调用返回时），生成的线程将因为整个进程退出而被终止。

### `join`
### `join`

You can also wait for a spawned thread to finish by calling the `join` method on the `JoinHandle` that `spawn` returns.
你也可以通过调用 `spawn` 返回的 `JoinHandle` 上的 `join` 方法来等待生成的线程完成。

```rust
use std::thread;
fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
}
```

In this example, the main thread will wait for the spawned thread to finish before exiting.\
在这个例子中，主线程会等待生成的线程完成后再退出。\
This introduces a form of **synchronization** between the two threads: you're guaranteed to see the message
"Hello from a thread!" printed before the program exits, because the main thread won't exit
until the spawned thread has finished.
这引入了两个线程之间的一种**同步**形式：你保证会在程序退出前看到"Hello from a thread!"消息被打印，因为主线程在生成的线程完成之前不会退出。
