# Encapsulation
# 封装

Now that we have a basic understanding of modules and visibility, let's circle back to **encapsulation**.\
现在我们对模块和可见性有了基本了解，让我们回到**封装**。\
Encapsulation is the practice of hiding the internal representation of an object. It is most commonly
used to enforce some **invariants** on the object's state.
封装是隐藏对象内部表示的做法。它最常用于对对象的状态强制执行一些**不变量**。

Going back to our `Ticket` struct:
回到我们的 `Ticket` 结构体：

```rust
struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

If all fields are made public, there is no encapsulation.\
如果所有字段都设为公开，就没有封装。\
You must assume that the fields can be modified at any time, set to any value that's allowed by
their type. You can't rule out that a ticket might have an empty title or a status
that doesn't make sense.
你必须假设字段可以随时被修改为它们类型允许的任何值。你无法排除工单可能有空标题或无意义状态的可能性。

To enforce stricter rules, we must keep the fields private[^newtype].
为了强制执行更严格的规则，我们必须保持字段私有[^newtype]。
We can then provide public methods to interact with a `Ticket` instance.
然后我们可以提供公开方法来与 `Ticket` 实例交互。
Those public methods will have the responsibility of upholding our invariants (e.g. a title must not be empty).
这些公开方法将负责维护我们的不变量（例如，标题不能为空）。

If at least one field is private it is no longer possible to create a `Ticket` instance directly using the struct
instantiation syntax:
如果至少有一个字段是私有的，就无法再直接使用结构体实例化语法创建 `Ticket` 实例：

```rust
// This won't work!
let ticket = Ticket {
    title: "Build a ticket system".into(),
    description: "A Kanban board".into(),
    status: "Open".into()
};
```

You've seen this in action in the previous exercise on visibility.\
你在上一个关于可见性的练习中已经看到了这一点。\
We now need to provide one or more public **constructors**—i.e. static methods or functions that can be used
from outside the module to create a new instance of the struct.\
我们现在需要提供一个或多个公开的**构造函数**——即可以从模块外部使用的静态方法或函数来创建结构体的新实例。\
Luckily enough we already have one: `Ticket::new`, as implemented in [a previous exercise](02_validation.md).
幸运的是，我们已经有一个：`Ticket::new`，正如在[上一个练习](02_validation.md)中实现的那样。

## Accessor methods
## 访问器方法

In summary:
总结来说：

- All `Ticket` fields are private
- 所有 `Ticket` 字段都是私有的
- We provide a public constructor, `Ticket::new`, that enforces our validation rules on creation
- 我们提供了一个公开构造函数 `Ticket::new`，在创建时强制执行我们的验证规则

That's a good start, but it's not enough: apart from creating a `Ticket`, we also need to interact with it.
这是一个好的开始，但这还不够：除了创建 `Ticket`，我们还需要与它交互。
But how can we access the fields if they're private?
但是如果字段是私有的，我们如何访问它们呢？

We need to provide **accessor methods**.\
我们需要提供**访问器方法**。\
Accessor methods are public methods that allow you to read the value of a private field (or fields) of a struct.
访问器方法是可以让你读取结构体私有字段值的公开方法。

Rust doesn't have a built-in way to generate accessor methods for you, like some other languages do.
Rust 没有像其他一些语言那样内置为你生成访问器方法的方式。
You have to write them yourself—they're just regular methods.
你必须自己编写——它们只是普通的方法。

[^newtype]: Or refine their type, a technique we'll explore [later on](../05_ticket_v2/15_outro.md).
[^newtype]: 或者优化它们类型，这是我们在[后面](../05_ticket_v2/15_outro.md)将探索的一种技术。
