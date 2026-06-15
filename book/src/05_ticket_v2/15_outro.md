# Wrapping up
# 总结

When it comes to domain modelling, the devil is in the details.\
在领域建模方面，细节决定成败。\
Rust offers a wide range of tools to help you represent the constraints of your domain directly in the type system,
but it takes some practice to get it right and write code that looks idiomatic.
Rust 提供了广泛的工具来帮助你直接在类型系统中表示领域的约束，但需要一些练习才能正确掌握并编写出地道的代码。

Let's close the chapter with one final refinement of our `Ticket` model.\
让我们通过对 `Ticket` 模型进行最后一次精化来结束本章。\
We'll introduce a new type for each of the fields in `Ticket` to encapsulate the respective constraints.\
我们将为 `Ticket` 中的每个字段引入一个新类型来封装各自的约束。\
Every time someone accesses a `Ticket` field, they'll get back a value that's guaranteed to be valid—i.e. a
`TicketTitle` instead of a `String`. They won't have to worry about the title being empty elsewhere in the code:
as long as they have a `TicketTitle`, they know it's valid **by construction**.
每当有人访问 `Ticket` 字段时，他们都会得到一个保证有效的值——即 `TicketTitle` 而不是 `String`。他们不必担心标题在代码的其他地方为空：只要他们有一个 `TicketTitle`，他们就知道它在**构造上**是有效的。

This is just an example of how you can use Rust's type system to make your code safer and more expressive.
这只是你如何使用 Rust 的类型系统使代码更安全、更具表现力的一个例子。

## Further reading
## 延伸阅读

- [Parse, don't validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
- [解析，不要验证](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
- [Using types to guarantee domain invariants](https://www.lpalmieri.com/posts/2020-12-11-zero-to-production-6-domain-modelling/)
- [使用类型保证领域不变性](https://www.lpalmieri.com/posts/2020-12-11-zero-to-production-6-domain-modelling/)
