# Destructors
# 析构函数

When introducing the heap, we mentioned that you're responsible for freeing the memory you allocate.\
在介绍堆时，我们提到你有责任释放你分配的内存。\
When introducing the borrow-checker, we also stated that you rarely have to manage memory directly in Rust.
在介绍借用检查器时，我们也说过在 Rust 中很少需要直接管理内存。

These two statements might seem contradictory at first.
这两个说法乍一看似乎矛盾。
Let's see how they fit together by introducing **scopes** and **destructors**.
让我们通过介绍**作用域**和**析构函数**来看看它们如何协调统一。

## Scopes
## 作用域

The **scope** of a variable is the region of Rust code where that variable is valid, or **alive**.
变量的**作用域**是该变量有效或**存活**的 Rust 代码区域。

The scope of a variable starts with its declaration.
变量的作用域从声明开始。
It ends when one of the following happens:
当以下情况之一发生时，作用域结束：

1. the block (i.e. the code between `{}`) where the variable was declared ends
1. 声明变量的块（即 `{}` 之间的代码）结束
   ```rust
   fn main() {
      // `x` is not yet in scope here
      let y = "Hello".to_string();
      let x = "World".to_string(); // <-- x's scope starts here...
      let h = "!".to_string(); //   |
   } //  <-------------- ...and ends here
   ```
2. ownership of the variable is transferred to someone else (e.g. a function or another variable)
2. 变量的所有权转移给其他人（例如函数或其他变量）
   ```rust
   fn compute(t: String) {
      // Do something [...]
   }

   fn main() {
       let s = "Hello".to_string(); // <-- s's scope starts here...
                   //                    | 
       compute(s); // <------------------- ..and ends here
                   //   because `s` is moved into `compute`
   }
   ```

## Destructors
## 析构函数

When the owner of a value goes out of scope, Rust invokes its **destructor**.\
当值的所有者离开作用域时，Rust 会调用其**析构函数**。\
The destructor tries to clean up the resources used by that value—in particular, whatever memory it allocated.
析构函数试图清理该值使用的资源——特别是它分配的任何内存。

You can manually invoke the destructor of a value by passing it to `std::mem::drop`.\
你可以通过将值传递给 `std::mem::drop` 来手动调用其析构函数。\
That's why you'll often hear Rust developers saying "that value has been **dropped**" as a way to state that a value
has gone out of scope and its destructor has been invoked.
这就是为什么你经常听到 Rust 开发者说"那个值已经被**丢弃(dropped)**"，表示一个值离开了作用域并且其析构函数已被调用。

### Visualizing drop points
### 可视化丢弃点

We can insert explicit calls to `drop` to "spell out" what the compiler does for us. Going back to the previous example:
我们可以插入显式的 `drop` 调用来"阐明"编译器为我们做了什么。回到之前的例子：

```rust
fn main() {
   let y = "Hello".to_string();
   let x = "World".to_string();
   let h = "!".to_string();
}
```

It's equivalent to:
它等价于：

```rust
fn main() {
   let y = "Hello".to_string();
   let x = "World".to_string();
   let h = "!".to_string();
   // Variables are dropped in reverse order of declaration
   drop(h);
   drop(x);
   drop(y);
}
```

Let's look at the second example instead, where `s`'s ownership is transferred to `compute`:
让我们看看第二个例子，其中 `s` 的所有权转移给了 `compute`：

```rust
fn compute(s: String) {
   // Do something [...]
}

fn main() {
   let s = "Hello".to_string();
   compute(s);
}
```

It's equivalent to this:
它等价于：

```rust
fn compute(t: String) {
    // Do something [...]
    drop(t); // <-- Assuming `t` wasn't dropped or moved 
             //     before this point, the compiler will call 
             //     `drop` here, when it goes out of scope
}

fn main() {
    let s = "Hello".to_string();
    compute(s);
}
```

Notice the difference: even though `s` is no longer valid after `compute` is called in `main`, there is no `drop(s)`
in `main`.
注意区别：即使在 `main` 中调用 `compute` 后 `s` 不再有效，但 `main` 中没有 `drop(s)`。
When you transfer ownership of a value to a function, you're also **transferring the responsibility of cleaning it up**.
当你将值的所有权转移给函数时，你也在**转移清理它的责任**。

This ensures that the destructor for a value is called **at most[^leak] once**, preventing
[double free bugs](https://owasp.org/www-community/vulnerabilities/Doubly_freeing_memory) by design.
这确保了值的析构函数**最多[^leak]被调用一次**，从根本上防止了[双重释放错误](https://owasp.org/www-community/vulnerabilities/Doubly_freeing_memory)。

### Use after drop
### 丢弃后使用

What happens if you try to use a value after it's been dropped?
如果你尝试在值被丢弃后使用它会发生什么？

```rust
let x = "Hello".to_string();
drop(x);
println!("{}", x);
```

If you try to compile this code, you'll get an error:
如果你尝试编译这段代码，你会得到一个错误：

```rust
error[E0382]: use of moved value: `x`
 --> src/main.rs:4:20
  |
3 |     drop(x);
  |          - value moved here
4 |     println!("{}", x);
  |                    ^ value used here after move
```

Drop **consumes** the value it's called on, meaning that the value is no longer valid after the call.\
Drop **消耗**了它所调用的值，意味着调用后该值不再有效。\
The compiler will therefore prevent you from using it, avoiding [use-after-free bugs](https://owasp.org/www-community/vulnerabilities/Using_freed_memory).
因此编译器会阻止你使用它，避免[释放后使用错误](https://owasp.org/www-community/vulnerabilities/Using_freed_memory)。

### Dropping references
### 丢弃引用

What if a variable contains a reference?\
如果一个变量包含引用呢？\
For example:
例如：

```rust
let x = 42i32;
let y = &x;
drop(y);
```

When you call `drop(y)`... nothing happens.\
当你调用 `drop(y)` 时...什么也不会发生。\
If you actually try to compile this code, you'll get a warning:
如果你真的尝试编译这段代码，你会得到一个警告：

```text
warning: calls to `std::mem::drop` with a reference 
         instead of an owned value does nothing
 --> src/main.rs:4:5
  |
4 |     drop(y);
  |     ^^^^^-^
  |          |
  |          argument has type `&i32`
  |
```

It goes back to what we said earlier: we only want to call the destructor once.\
这回到我们之前说的：我们只想调用一次析构函数。\
You can have multiple references to the same value—if we called the destructor for the value they point at
when one of them goes out of scope, what would happen to the others?
你可以有指向同一值的多个引用——如果我们在其中一个离开作用域时调用它们所指向值的析构函数，其他引用会发生什么？
They would refer to a memory location that's no longer valid: a so-called [**dangling pointer**](https://en.wikipedia.org/wiki/Dangling_pointer),
a close relative of [**use-after-free bugs**](https://owasp.org/www-community/vulnerabilities/Using_freed_memory).
它们将指向一个不再有效的内存位置：即所谓的[**悬垂指针**](https://en.wikipedia.org/wiki/Dangling_pointer)，与[**释放后使用错误**](https://owasp.org/www-community/vulnerabilities/Using_freed_memory)密切相关。
Rust's ownership system rules out these kinds of bugs by design.
Rust 的所有权系统从根本上排除了这类错误。

[^leak]: Rust doesn't guarantee that destructors will run. They won't, for example, if
you explicitly choose to [leak memory](../07_threads/03_leak.md).
[^leak]: Rust 不保证析构函数一定会运行。例如，如果你明确选择[泄漏内存](../07_threads/03_leak.md)，析构函数就不会运行。
