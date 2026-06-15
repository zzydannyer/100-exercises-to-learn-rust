# Error enums
# 错误枚举

Your solution to the previous exercise may have felt awkward: matching on strings is not ideal!\
你对上一个练习的解决方案可能感觉有些笨拙：匹配字符串并不理想！\
A colleague might rework the error messages returned by `Ticket::new` (e.g. to improve readability) and,
all of a sudden, your calling code would break.
同事可能会重新编写 `Ticket::new` 返回的错误消息（例如为了提高可读性），然后突然之间，你的调用代码就会出错。

You already know the machinery required to fix this: enums!
你已经知道修复这个问题所需的机制：枚举！

## Reacting to errors
## 对错误做出反应

When you want to allow the caller to behave differently based on the specific error that occurred, you can
use an enum to represent the different error cases:
当你想允许调用者根据发生的具体错误做出不同的行为时，你可以使用枚举来表示不同的错误情况：

```rust
// An error enum to represent the different error cases
// that may occur when parsing a `u32` from a string.
enum U32ParseError {
    NotANumber,
    TooLarge,
    Negative,
}
```

Using an error enum, you're encoding the different error cases in the type system—they become part of the
signature of the fallible function.\
使用错误枚举，你将不同的错误情况编码到类型系统中——它们成为易错函数签名的一部分。\
This simplifies error handling for the caller, as they can use a `match` expression to react to the different
error cases:
这简化了调用者的错误处理，因为他们可以使用 `match` 表达式对不同错误情况做出反应：

```rust
match s.parse_u32() {
    Ok(n) => n,
    Err(U32ParseError::Negative) => 0,
    Err(U32ParseError::TooLarge) => u32::MAX,
    Err(U32ParseError::NotANumber) => {
        panic!("Not a number: {}", s);
    }
}
```
