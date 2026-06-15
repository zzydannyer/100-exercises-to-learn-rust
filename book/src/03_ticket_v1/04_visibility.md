# Visibility
# 可见性

When you start breaking down your code into multiple modules, you need to start thinking about **visibility**.
当你开始将代码分解到多个模块中时，你需要开始考虑**可见性**。
Visibility determines which regions of your code (or other people's code) can access a given entity,
be it a struct, a function, a field, etc.
可见性决定了你的代码（或其他人的代码）的哪些区域可以访问给定的实体，无论是结构体、函数、字段等。

## Private by default
## 默认私有

By default, everything in Rust is **private**.\
默认情况下，Rust 中的所有内容都是**私有的**。\
A private entity can only be accessed:
私有实体只能在以下情况下被访问：

1. within the same module where it's defined, or
1. 在定义它的同一模块内，或
2. by one of its submodules
2. 由它的一个子模块访问

We've used this extensively in the previous exercises:
我们在前面的练习中广泛使用了这一点：

- `create_todo_ticket` worked (once you added a `use` statement) because `helpers` is a submodule of the crate root,
  where `Ticket` is defined. Therefore, `create_todo_ticket` can access `Ticket` without any issues even
  though `Ticket` is private.
- `create_todo_ticket` 可以工作（一旦你添加了 `use` 语句），因为 `helpers` 是 crate 根（`Ticket` 定义的地方）的一个子模块。因此，即使 `Ticket` 是私有的，`create_todo_ticket` 也可以毫无问题地访问 `Ticket`。
- All our unit tests are defined in a submodule of the code they're testing, so they can access everything without
  restrictions.
- 我们所有的单元测试都定义在被测试代码的子模块中，因此它们可以不受限制地访问所有内容。

## Visibility modifiers
## 可见性修饰符

You can modify the default visibility of an entity using a **visibility modifier**.\
你可以使用**可见性修饰符**修改实体的默认可见性。\
Some common visibility modifiers are:
一些常见的可见性修饰符有：

- `pub`: makes the entity **public**, i.e. accessible from outside the module where it's defined, potentially from
  other crates.
- `pub`：使实体**公开**，即可以从定义它的模块外部访问，可能来自其他 crate。
- `pub(crate)`: makes the entity public within the same **crate**, but not outside of it.
- `pub(crate)`：使实体在同一 **crate** 内公开，但在其外部不可访问。
- `pub(super)`: makes the entity public within the parent module.
- `pub(super)`：使实体在父模块内公开。
- `pub(in path::to::module)`: makes the entity public within the specified module.
- `pub(in path::to::module)`：使实体在指定模块内公开。

You can use these modifiers on modules, structs, functions, fields, etc.
你可以在模块、结构体、函数、字段等上使用这些修饰符。
For example:
例如：

```rust
pub struct Configuration {
    pub(crate) version: u32,
    active: bool,
}
```

`Configuration` is public, but you can only access the `version` field from within the same crate.
`Configuration` 是公开的，但你只能从同一个 crate 内访问 `version` 字段。
The `active` field, instead, is private and can only be accessed from within the same module or one of its submodules.
而 `active` 字段是私有的，只能从同一个模块或其子模块内访问。
