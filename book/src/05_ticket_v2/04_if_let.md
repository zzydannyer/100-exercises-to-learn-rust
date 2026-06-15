# Concise branching
# 简洁的分支

Your solution to the previous exercise probably looks like this:
你对上一个练习的解决方案可能看起来像这样：

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        match &self.status {
            Status::InProgress { assigned_to } => assigned_to,
            Status::Done | Status::ToDo => {
                panic!(
                    "Only `In-Progress` tickets can be \
                    assigned to someone"
                )
            }
        }
    }
}
```

You only care about the `Status::InProgress` variant.
你只关心 `Status::InProgress` 变体。
Do you really need to match on all the other variants?
你真的需要匹配所有其他变体吗？

New constructs to the rescue!
新的构造来救场了！

## `if let`
## `if let`

The `if let` construct allows you to match on a single variant of an enum,
without having to handle all the other variants.
`if let` 构造允许你匹配枚举的单个变体，而无需处理所有其他变体。

Here's how you can use `if let` to simplify the `assigned_to` method:
以下是你如何使用 `if let` 来简化 `assigned_to` 方法：

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        if let Status::InProgress { assigned_to } = &self.status {
            assigned_to
        } else {
            panic!(
                "Only `In-Progress` tickets can be assigned to someone"
            );
        }
    }
}
```

## `let/else`
## `let/else`

If the `else` branch is meant to return early (a panic counts as returning early!),
you can use the `let/else` construct:
如果 `else` 分支是要提前返回（恐慌也算提前返回！），你可以使用 `let/else` 构造：

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        let Status::InProgress { assigned_to } = &self.status else {
            panic!(
                "Only `In-Progress` tickets can be assigned to someone"
            );
        };
        assigned_to
    }
}
```

It allows you to assign the destructured variable without incurring
any "right drift", i.e. the variable is assigned at the same indentation level
as the code that precedes it.
它允许你赋值解构后的变量，而不会产生任何"向右偏移"，即变量的赋值与前面的代码处于相同的缩进级别。

## Style
## 风格

Both `if let` and `let/else` are idiomatic Rust constructs.\
`if let` 和 `let/else` 都是地道的 Rust 构造。\
Use them as you see fit to improve the readability of your code,
but don't overdo it: `match` is always there when you need it.
根据需要适当使用它们来改善代码的可读性，但不要过度：`match` 在需要时随时可用。
