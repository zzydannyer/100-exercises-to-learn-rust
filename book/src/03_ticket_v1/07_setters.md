# Mutable references
# 可变引用

Your accessor methods should look like this now:
你的访问器方法现在应该看起来像这样：

```rust
impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}
```

A sprinkle of `&` here and there did the trick!\
这里那里加一点 `&` 就搞定了！\
We now have a way to access the fields of a `Ticket` instance without consuming it in the process.
我们现在有了一种访问 `Ticket` 实例字段而不在此过程中消耗它的方法。
Let's see how we can enhance our `Ticket` struct with **setter methods** next.
接下来，让我们看看如何用**设置器方法**来增强我们的 `Ticket` 结构体。

## Setters
## 设置器

Setter methods allow users to change the values of `Ticket`'s private fields while making sure that its invariants
are respected (i.e. you can't set a `Ticket`'s title to an empty string).
设置器方法允许用户更改 `Ticket` 的私有字段值，同时确保其不变量得到维护（即，你不能将 `Ticket` 的标题设置为空字符串）。

There are two common ways to implement setters in Rust:
在 Rust 中有两种常见的实现设置器的方法：

- Taking `self` as input.
- 将 `self` 作为输入。
- Taking `&mut self` as input.
- 将 `&mut self` 作为输入。

### Taking `self` as input
### 将 `self` 作为输入

The first approach looks like this:
第一种方法看起来像这样：

```rust
impl Ticket {
    pub fn set_title(mut self, new_title: String) -> Self {
        // Validate the new title [...]
        self.title = new_title;
        self
    }
}
```

It takes ownership of `self`, changes the title, and returns the modified `Ticket` instance.\
它取得 `self` 的所有权，更改标题，然后返回修改后的 `Ticket` 实例。\
This is how you'd use it:
这样使用：

```rust
let ticket = Ticket::new(
    "Title".into(), 
    "Description".into(), 
    "To-Do".into()
);
let ticket = ticket.set_title("New title".into());
```

Since `set_title` takes ownership of `self` (i.e. it **consumes it**), we need to reassign the result to a variable.
由于 `set_title` 取得 `self` 的所有权（即它**消耗了它**），我们需要将结果重新赋给一个变量。
In the example above we take advantage of **variable shadowing** to reuse the same variable name: when
you declare a new variable with the same name as an existing one, the new variable **shadows** the old one. This
is a common pattern in Rust code.
在上面的例子中，我们利用**变量遮蔽(variable shadowing)**来重用相同的变量名：当你用与现有变量相同的名称声明新变量时，新变量**遮蔽**旧变量。这是 Rust 代码中的常见模式。

`self`-setters work quite nicely when you need to change multiple fields at once: you can chain multiple calls together!
当你需要一次更改多个字段时，`self` 设置器效果很好：你可以将多个调用链接在一起！

```rust
let ticket = ticket
    .set_title("New title".into())
    .set_description("New description".into())
    .set_status("In Progress".into());
```

### Taking `&mut self` as input
### 将 `&mut self` 作为输入

The second approach to setters, using `&mut self`, looks like this instead:
第二种设置器方法，使用 `&mut self`，看起来像这样：

```rust
impl Ticket {
    pub fn set_title(&mut self, new_title: String) {
        // Validate the new title [...]
        
        self.title = new_title;
    }
}
```

This time the method takes a mutable reference to `self` as input, changes the title, and that's it.
这次该方法将 `self` 的可变引用作为输入，更改标题，就这样。
Nothing is returned.
不返回任何内容。

You'd use it like this:
你这样使用它：

```rust
let mut ticket = Ticket::new(
    "Title".into(),
    "Description".into(),
    "To-Do".into()
);
ticket.set_title("New title".into());

// Use the modified ticket
```

Ownership stays with the caller, so the original `ticket` variable is still valid. We don't need to reassign the result.
所有权保留在调用者手中，所以原始的 `ticket` 变量仍然有效。我们不需要重新赋值结果。
We need to mark `ticket` as mutable though, because we're taking a mutable reference to it.
不过我们需要将 `ticket` 标记为可变的，因为我们要获取它的可变引用。

`&mut`-setters have a downside: you can't chain multiple calls together.
`&mut` 设置器有一个缺点：你不能将多个调用链接在一起。
Since they don't return the modified `Ticket` instance, you can't call another setter on the result of the first one.
由于它们不返回修改后的 `Ticket` 实例，你不能在第一个设置器的结果上调用另一个设置器。
You have to call each setter separately:
你必须分别调用每个设置器：

```rust
ticket.set_title("New title".into());
ticket.set_description("New description".into());
ticket.set_status("In Progress".into());
```
