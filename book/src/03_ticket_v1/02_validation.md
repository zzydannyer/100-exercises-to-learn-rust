# Validation
# 验证

Let's go back to our ticket definition:
让我们回到我们的工单定义：

```rust
struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

We are using "raw" types for the fields of our `Ticket` struct.
我们对 `Ticket` 结构体的字段使用了"原始"类型。
This means that users can create a ticket with an empty title, a suuuuuuuper long description or
a nonsensical status (e.g. "Funny").\
这意味着用户可以创建标题为空、描述超长或状态无意义（如"Funny"）的工单。\
We can do better than that!
我们可以做得更好！

## Further reading
## 扩展阅读

- Check out [`String`'s documentation](https://doc.rust-lang.org/std/string/struct.String.html)
  for a thorough overview of the methods it provides. You'll need it for the exercise!
- 查看 [`String` 的文档](https://doc.rust-lang.org/std/string/struct.String.html) 了解其提供的方法的全面概述。练习中会用到！
