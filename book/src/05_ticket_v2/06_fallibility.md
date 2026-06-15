# Fallibility
# 易错性

Let's revisit the `Ticket::new` function from the previous exercise:
让我们重新审视上一个练习中的 `Ticket::new` 函数：

```rust
impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: Status
    ) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }

        Ticket {
            title,
            description,
            status,
        }
    }
}
```

As soon as one of the checks fails, the function panics.
只要其中一个检查失败，函数就会恐慌。
This is not ideal, as it doesn't give the caller a chance to **handle the error**.
这并不理想，因为它没有给调用者**处理错误**的机会。

It's time to introduce the `Result` type, Rust's primary mechanism for error handling.
是时候介绍 `Result` 类型了，这是 Rust 用于错误处理的主要机制。

## The `Result` type
## `Result` 类型

The `Result` type is an enum defined in the standard library:
`Result` 类型是标准库中定义的一个枚举：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

It has two variants:
它有两个变体：

- `Ok(T)`: represents a successful operation. It holds `T`, the output of the operation.
- `Ok(T)`：表示一个成功的操作。它持有 `T`，即操作的输出。
- `Err(E)`: represents a failed operation. It holds `E`, the error that occurred.
- `Err(E)`：表示一个失败的操作。它持有 `E`，即发生的错误。

Both `Ok` and `Err` are generic, allowing you to specify your own types for the success and error cases.
`Ok` 和 `Err` 都是泛型的，允许你为成功和错误情况指定你自己的类型。

## No exceptions
## 没有异常

Recoverable errors in Rust are **represented as values**.\
Rust 中的可恢复错误被**表示为值**。\
They're just an instance of a type, being passed around and manipulated like any other value.
它们只是类型的一个实例，像其他任何值一样被传递和操作。
This is a significant difference from other languages, such as Python or C#, where **exceptions** are used to signal errors.
这与其他语言（如 Python 或 C#）有显著不同，在这些语言中使用**异常**来指示错误。

Exceptions create a separate control flow path that can be hard to reason about.\
异常创建了一个单独的控制流路径，这可能难以推理。\
You don't know, just by looking at a function's signature, if it can throw an exception or not.
仅仅通过查看函数的签名，你不知道它是否可以抛出异常。
You don't know, just by looking at a function's signature, **which** exception types it can throw.\
仅仅通过查看函数的签名，你不知道它**能抛出哪些**异常类型。\
You must either read the function's documentation or look at its implementation to find out.
你必须阅读函数的文档或查看其实现才能找到答案。

Exception handling logic has very poor locality: the code that throws the exception is far removed from the code
that catches it, and there's no direct link between the two.
异常处理逻辑的局部性非常差：抛出异常的代码与捕获异常的代码相隔甚远，两者之间没有直接联系。

## Fallibility is encoded in the type system
## 易错性编码在类型系统中

Rust, with `Result`, forces you to **encode fallibility in the function's signature**.\
Rust 通过 `Result` 强制你在**函数的签名中编码易错性**。\
If a function can fail (and you want the caller to have a shot at handling the error), it must return a `Result`.
如果一个函数可能失败（并且你希望调用者有机会处理错误），它必须返回一个 `Result`。

```rust
// Just by looking at the signature, you know that this function 
// can fail. You can also inspect `ParseIntError` to see what 
// kind of failures to expect.
fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    // ...
}
```

That's the big advantage of `Result`: it makes fallibility explicit.
这就是 `Result` 的巨大优势：它使易错性显式化。

Keep in mind, though, that panics exist. They aren't tracked by the type system, just like exceptions in other languages.
But they're meant for **unrecoverable errors** and should be used sparingly.
不过请记住，恐慌是存在的。它们不像其他语言中的异常那样被类型系统跟踪。但它们是用于**不可恢复的错误**，应该谨慎使用。
