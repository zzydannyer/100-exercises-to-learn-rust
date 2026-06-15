# Structs
# 结构体

We need to keep track of three pieces of information for each ticket:
我们需要为每个工单跟踪三条信息：

- A title
- 标题
- A description
- 描述
- A status
- 状态

We can start by using a [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
to represent them. `String` is the type defined in Rust's standard library to represent
[UTF-8 encoded](https://en.wikipedia.org/wiki/UTF-8) text.
我们可以先使用 [`String`](https://doc.rust-lang.org/std/string/struct.String.html) 来表示它们。`String` 是 Rust 标准库中定义的类型，用于表示 [UTF-8 编码](https://en.wikipedia.org/wiki/UTF-8)的文本。

But how do we **combine** these three pieces of information into a single entity?
但是我们如何将这三条信息**组合**成一个单独的实体呢？

## Defining a `struct`
## 定义 `struct`

A `struct` defines a **new Rust type**.
`struct` 定义了一个**新的 Rust 类型**。

```rust
struct Ticket {
    title: String,
    description: String,
    status: String
}
```

A struct is quite similar to what you would call a class or an object in other programming languages.
结构体与其他编程语言中称为类或对象的东西非常相似。

## Defining fields
## 定义字段

The new type is built by combining other types as **fields**.\
新类型通过将其他类型作为**字段**组合而成。\
Each field must have a name and a type, separated by a colon, `:`. If there are multiple fields, they are separated by a comma, `,`.
每个字段必须有一个名称和类型，用冒号 `:` 分隔。如果有多个字段，它们用逗号 `,` 分隔。

Fields don't have to be of the same type, as you can see in the `Configuration` struct below:
字段不必是同一类型，正如你在下面的 `Configuration` 结构体中看到的：

```rust
struct Configuration {
   version: u32,
   active: bool
}
```

## Instantiation
## 实例化

You can create an instance of a struct by specifying the values for each field:
你可以通过为每个字段指定值来创建结构体的实例：

```rust
// Syntax: <StructName> { <field_name>: <value>, ... }
let ticket = Ticket {
    title: "Build a ticket system".into(),
    description: "A Kanban board".into(),
    status: "Open".into()
};
```

## Accessing fields
## 访问字段

You can access the fields of a struct using the `.` operator:
你可以使用 `.` 运算符访问结构体的字段：

```rust
// Field access
let x = ticket.description;
```

## Methods
## 方法

We can attach behaviour to our structs by defining **methods**.\
我们可以通过定义**方法**来为结构体附加行为。\
Using the `Ticket` struct as an example:
以 `Ticket` 结构体为例：

```rust
impl Ticket {
    fn is_open(self) -> bool {
        self.status == "Open"
    }
}

// Syntax:
// impl <StructName> {
//    fn <method_name>(<parameters>) -> <return_type> {
//        // Method body
//    }
// }
```

Methods are pretty similar to functions, with two key differences:
方法与函数非常相似，有两个关键区别：

1. methods must be defined inside an **`impl` block**
1. 方法必须在 **`impl` 块**内定义
2. methods may use `self` as their first parameter.
   `self` is a keyword and represents the instance of the struct the method is being called on.
2. 方法可以使用 `self` 作为它们的第一个参数。
   `self` 是一个关键字，代表方法被调用的结构体实例。

### `self`
### `self`

If a method takes `self` as its first parameter, it can be called using the **method call syntax**:
如果方法将 `self` 作为第一个参数，则可以使用**方法调用语法**来调用：

```rust
// Method call syntax: <instance>.<method_name>(<parameters>)
let is_open = ticket.is_open();
```

This is the same calling syntax you used to perform saturating arithmetic operations on `u32` values
in [the previous chapter](../02_basic_calculator/09_saturating.md).
这与你在[前一章](../02_basic_calculator/09_saturating.md)中对 `u32` 值执行饱和算术运算时使用的调用语法相同。

### Static methods
### 静态方法

If a method doesn't take `self` as its first parameter, it's a **static method**.
如果方法不将 `self` 作为第一个参数，则它是**静态方法**。

```rust
struct Configuration {
    version: u32,
    active: bool
}

impl Configuration {
    // `default` is a static method on `Configuration`
    fn default() -> Configuration {
        Configuration { version: 0, active: false }
    }
}
```

The only way to call a static method is by using the **function call syntax**:
调用静态方法的唯一方式是使用**函数调用语法**：

```rust
// Function call syntax: <StructName>::<method_name>(<parameters>)
let default_config = Configuration::default();
```

### Equivalence
### 等价性

You can use the function call syntax even for methods that take `self` as their first parameter:
即使是那些将 `self` 作为第一个参数的方法，你也可以使用函数调用语法：

```rust
// Function call syntax:
//   <StructName>::<method_name>(<instance>, <parameters>)
let is_open = Ticket::is_open(ticket);
```

The function call syntax makes it quite clear that `ticket` is being used as `self`, the first parameter of the method,
but it's definitely more verbose. Prefer the method call syntax when possible.
函数调用语法清楚地表明 `ticket` 被用作 `self`（方法的第一个参数），但它确实更冗长。尽可能优先使用方法调用语法。
