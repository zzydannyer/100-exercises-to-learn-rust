# `Error::source`
# `Error::source`

There's one more thing we need to talk about to complete our coverage of the `Error` trait: the `source` method.
为了完整地覆盖 `Error` 特征，还有一件事我们需要讨论：`source` 方法。

```rust
// Full definition this time!
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
```

The `source` method is a way to access the **error cause**, if any.\
`source` 方法是一种访问**错误原因**的方式（如果有的话）。\
Errors are often chained, meaning that one error is the cause of another: you have a high-level error (e.g.
cannot connect to the database) that is caused by a lower-level error (e.g. can't resolve the database hostname).
The `source` method allows you to "walk" the full chain of errors, often used when capturing error context in logs.
错误通常是链式的，意味着一个错误是另一个错误的起因：你有一个高级错误（例如无法连接到数据库），它是由一个低级错误（例如无法解析数据库主机名）引起的。`source` 方法允许你"遍历"完整的错误链，通常用于在日志中捕获错误上下文。

## Implementing `source`
## 实现 `source`

The `Error` trait provides a default implementation that always returns `None` (i.e. no underlying cause). That's why
you didn't have to care about `source` in the previous exercises.\
`Error` 特征提供了一个总是返回 `None`（即没有潜在原因）的默认实现。这就是为什么你在之前的练习中不需要关心 `source`。\
You can override this default implementation to provide a cause for your error type.
你可以覆盖这个默认实现，为你的错误类型提供一个原因。

```rust
use std::error::Error;

#[derive(Debug)]
struct DatabaseError {
    source: std::io::Error
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to connect to the database")
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
```

In this example, `DatabaseError` wraps an `std::io::Error` as its source.
We then override the `source` method to return this source when called.
在这个例子中，`DatabaseError` 包装了一个 `std::io::Error` 作为其来源。然后我们覆盖 `source` 方法，以便在调用时返回这个来源。

## `&(dyn Error + 'static)`
## `&(dyn Error + 'static)`

What's this `&(dyn Error + 'static)` type?\
这个 `&(dyn Error + 'static)` 类型是什么？\
Let's unpack it:
让我们来解析它：

- `dyn Error` is a **trait object**. It's a way to refer to any type that implements the `Error` trait.
- `dyn Error` 是一个**特征对象**。它是一种引用任何实现了 `Error` 特征的类型的方式。
- `'static` is a special **lifetime specifier**.
  `'static` implies that the reference is valid for "as long as we need it", i.e. the entire program execution.
- `'static` 是一个特殊的**生命周期说明符**。`'static` 意味着该引用"只要我们需要它"就有效，即整个程序执行期间。

Combined: `&(dyn Error + 'static)` is a reference to a trait object that implements the `Error` trait
and is valid for the entire program execution.
结合起来：`&(dyn Error + 'static)` 是对一个实现了 `Error` 特征并且在整个程序执行期间有效的特征对象的引用。

Don't worry too much about either of these concepts for now. We'll cover them in more detail in future chapters.
现在不要太担心这些概念。我们将在未来的章节中更详细地介绍它们。

## Implementing `source` using `thiserror`
## 使用 `thiserror` 实现 `source`

`thiserror` provides three ways to automatically implement `source` for your error types:
`thiserror` 提供了三种方式来自动为你的错误类型实现 `source`：

- A field named `source` will automatically be used as the source of the error.
- 名为 `source` 的字段将自动被用作错误的来源。
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          source: std::io::Error
      }
  }
  ```
- A field annotated with the `#[source]` attribute will automatically be used as the source of the error.
- 带有 `#[source]` 属性注解的字段将自动被用作错误的来源。
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[source]
          inner: std::io::Error
      }
  }
  ```
- A field annotated with the `#[from]` attribute will automatically be used as the source of the error **and**
  `thiserror` will automatically generate a `From` implementation to convert the annotated type into your error type.
- 带有 `#[from]` 属性注解的字段将自动被用作错误的来源，**并且** `thiserror` 将自动生成一个 `From` 实现，将注解的类型转换为你的错误类型。
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[from]
          inner: std::io::Error
      }
  }
  ```

## The `?` operator
## `?` 运算符

The `?` operator is a shorthand for propagating errors.\
`?` 运算符是传播错误的简写。\
When used in a function that returns a `Result`, it will return early with an error if the `Result` is `Err`.
当在返回 `Result` 的函数中使用时，如果 `Result` 是 `Err`，它会提前返回一个错误。

For example:
例如：

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

is equivalent to:
等价于：

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = match File::open("file.txt") {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            return Err(e);
        }
    }
    Ok(contents)
}
```

You can use the `?` operator to shorten your error handling code significantly.\
你可以使用 `?` 运算符来显著缩短你的错误处理代码。\
In particular, the `?` operator will automatically convert the error type of the fallible operation into the error type
of the function, if a conversion is possible (i.e. if there is a suitable `From` implementation)
特别地，如果转换是可能的（即如果有合适的 `From` 实现），`?` 运算符会自动将易错操作的错误类型转换为函数的错误类型。
