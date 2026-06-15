# `match`
# `match`

You may be wondering—what can you actually **do** with an enum?\
你可能想知道——你实际上能用枚举**做什么**？\
The most common operation is to **match** on it.
最常见的操作是**匹配**它。

```rust
enum Status {
    ToDo,
    InProgress,
    Done
}

impl Status {
    fn is_done(&self) -> bool {
        match self {
            Status::Done => true,
            // The `|` operator lets you match multiple patterns.
            // It reads as "either `Status::ToDo` or `Status::InProgress`".
            Status::InProgress | Status::ToDo => false
        }
    }
}
```

A `match` statement that lets you compare a Rust value against a series of **patterns**.\
`match` 语句让你将 Rust 值与一系列**模式**进行比较。\
You can think of it as a type-level `if`. If `status` is a `Done` variant, execute the first block;
if it's a `InProgress` or `ToDo` variant, execute the second block.
你可以把它看作类型层面的 `if`。如果 `status` 是 `Done` 变体，执行第一个代码块；如果是 `InProgress` 或 `ToDo` 变体，执行第二个代码块。

## Exhaustiveness
## 穷尽性

There's one key detail here: `match` is **exhaustive**. You must handle all enum variants.\
这里有一个关键细节：`match` 是**穷尽的**。你必须处理所有枚举变体。\
If you forget to handle a variant, Rust will stop you **at compile-time** with an error.
如果你忘记处理某个变体，Rust 会在**编译时**阻止你并报错。

E.g. if we forget to handle the `ToDo` variant:
例如，如果我们忘记处理 `ToDo` 变体：

```rust
match self {
    Status::Done => true,
    Status::InProgress => false,
}
```

the compiler will complain:
编译器会报错：

```text
error[E0004]: non-exhaustive patterns: `ToDo` not covered
 --> src/main.rs:5:9
  |
5 |     match status {
  |     ^^^^^^^^^^^^ pattern `ToDo` not covered
```

This is a big deal!\
这很重要！\
Codebases evolve over time—you might add a new status down the line, e.g. `Blocked`. The Rust compiler
will emit an error for every single `match` statement that's missing logic for the new variant.
代码库会随着时间演变——你可能会在后续添加一个新状态，例如 `Blocked`。Rust 编译器会为每个缺少新变体逻辑的 `match` 语句发出错误。
That's why Rust developers often sing the praises of "compiler-driven refactoring"—the compiler tells you
what to do next, you just have to fix what it reports.
这就是为什么 Rust 开发者经常称赞"编译器驱动的重构"——编译器告诉你下一步要做什么，你只需修复它报告的问题。

## Catch-all
## 通配匹配

If you don't care about one or more variants, you can use the `_` pattern as a catch-all:
如果你不关心一个或多个变体，你可以使用 `_` 模式作为通配匹配：

```rust
match status {
    Status::Done => true,
    _ => false
}
```

The `_` pattern matches anything that wasn't matched by the previous patterns.
`_` 模式匹配任何未被前面模式匹配到的内容。

<div class="warning">
By using this catch-all pattern, you _won't_ get the benefits of compiler-driven refactoring.
If you add a new enum variant, the compiler _won't_ tell you that you're not handling it.
如果你使用这种通配匹配模式，你_不会_获得编译器驱动重构的好处。
如果你添加一个新的枚举变体，编译器_不会_告诉你没有处理它。

If you're keen on correctness, avoid using catch-alls. Leverage the compiler to re-examine all matching sites and determine how new enum variants should be handled.
如果你追求正确性，避免使用通配匹配。利用编译器重新检查所有匹配位置，并确定如何处理新的枚举变体。

</div>
