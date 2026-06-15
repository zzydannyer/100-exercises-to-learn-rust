# Variants can hold data
# 变体可以包含数据

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

Our `Status` enum is what's usually called a **C-style enum**.\
我们的 `Status` 枚举就是通常所说的 **C 风格枚举**。\
Each variant is a simple label, a bit like a named constant. You can find this kind of enum in many programming
languages, like C, C++, Java, C#, Python, etc.
每个变体都是一个简单的标签，有点像命名常量。你可以在许多编程语言中找到这种枚举，如 C、C++、Java、C#、Python 等。

Rust enums can go further though. We can **attach data to each variant**.
不过，Rust 枚举可以更进一步。我们可以**将数据附加到每个变体**。

## Variants
## 变体

Let's say that we want to store the name of the person who's currently working on a ticket.\
假设我们想要存储当前正在处理票据的人的名字。\
We would only have this information if the ticket is in progress. It wouldn't be there for a to-do ticket or
a done ticket.
只有当票据正在进行中时，我们才会有这个信息。对于待办或已完成的票据，它不应该存在。
We can model this by attaching a `String` field to the `InProgress` variant:
我们可以通过给 `InProgress` 变体附加一个 `String` 字段来建模：

```rust
enum Status {
    ToDo,
    InProgress {
        assigned_to: String,
    },
    Done,
}
```

`InProgress` is now a **struct-like variant**.\
`InProgress` 现在是一个**类结构体变体**。\
The syntax mirrors, in fact, the one we used to define a struct—it's just "inlined" inside the enum, as a variant.
实际上，其语法反映了我们用来定义结构体的语法——它只是作为变体"内联"在枚举内部。

## Accessing variant data
## 访问变体数据

If we try to access `assigned_to` on a `Status` instance,
如果我们尝试在 `Status` 实例上访问 `assigned_to`：

```rust
let status: Status = /* */;

// This won't compile
println!("Assigned to: {}", status.assigned_to);
```

the compiler will stop us:
编译器会阻止我们：

```text
error[E0609]: no field `assigned_to` on type `Status`
 --> src/main.rs:5:40
  |
5 |     println!("Assigned to: {}", status.assigned_to);
  |                                        ^^^^^^^^^^^ unknown field
```

`assigned_to` is **variant-specific**, it's not available on all `Status` instances.\
`assigned_to` 是**特定于变体的**，它在所有 `Status` 实例上并不可用。\
To access `assigned_to`, we need to use **pattern matching**:
要访问 `assigned_to`，我们需要使用**模式匹配**：

```rust
match status {
    Status::InProgress { assigned_to } => {
        println!("Assigned to: {}", assigned_to);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```

## Bindings
## 绑定

In the match pattern `Status::InProgress { assigned_to }`, `assigned_to` is a **binding**.\
在匹配模式 `Status::InProgress { assigned_to }` 中，`assigned_to` 是一个**绑定**。\
We're **destructuring** the `Status::InProgress` variant and binding the `assigned_to` field to
a new variable, also named `assigned_to`.\
我们正在**解构** `Status::InProgress` 变体，并将 `assigned_to` 字段绑定到一个新的变量，也命名为 `assigned_to`。\
If we wanted, we could bind the field to a different variable name:
如果我们愿意，我们可以将该字段绑定到不同的变量名：

```rust
match status {
    Status::InProgress { assigned_to: person } => {
        println!("Assigned to: {}", person);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```
