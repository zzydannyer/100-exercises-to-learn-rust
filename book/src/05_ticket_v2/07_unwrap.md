# Unwrapping
# 解包

`Ticket::new` now returns a `Result` instead of panicking on invalid inputs.\
`Ticket::new` 现在返回一个 `Result`，而不是在无效输入时恐慌。\
What does this mean for the caller?
这对调用者意味着什么？

## Failures can't be (implicitly) ignored
## 失败不能被（隐式地）忽略

Unlike exceptions, Rust's `Result` forces you to **handle errors at the call site**.\
与异常不同，Rust 的 `Result` 强制你在**调用点处理错误**。\
If you call a function that returns a `Result`, Rust won't allow you to implicitly ignore the error case.
如果你调用一个返回 `Result` 的函数，Rust 不会允许你隐式地忽略错误情况。

```rust
fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    // ...
}

// This won't compile: we're not handling the error case.
// We must either use `match` or one of the combinators provided by 
// `Result` to "unwrap" the success value or handle the error.
let number = parse_int("42") + 2;
```

## You got a `Result`. Now what?
## 你得到了一个 `Result`。现在怎么办？

When you call a function that returns a `Result`, you have two key options:
当你调用一个返回 `Result` 的函数时，你有两个关键选择：

- Panic if the operation failed.
  This is done using either the `unwrap` or `expect` methods.
- 如果操作失败则恐慌。这通过使用 `unwrap` 或 `expect` 方法来完成。
  ```rust
  // Panics if `parse_int` returns an `Err`.
  let number = parse_int("42").unwrap();
  // `expect` lets you specify a custom panic message.
  let number = parse_int("42").expect("Failed to parse integer");
  ```
- Destructure the `Result` using a `match` expression to deal with the error case explicitly.
- 使用 `match` 表达式解构 `Result` 来显式处理错误情况。
  ```rust
  match parse_int("42") {
      Ok(number) => println!("Parsed number: {}", number),
      Err(err) => eprintln!("Error: {}", err),
  }
  ```
