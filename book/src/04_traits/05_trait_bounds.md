# Trait bounds
# 特征约束

We've seen two use cases for traits so far:
到目前为止，我们已经看到了特征的两种用途：

- Unlocking "built-in" behaviour (e.g. operator overloading)
- 解锁"内置"行为（例如运算符重载）
- Adding new behaviour to existing types (i.e. extension traits)
- 向现有类型添加新行为（即扩展特征）

There's a third use case: **generic programming**.
还有第三种用途：**泛型编程**。

## The problem
## 问题

All our functions and methods, so far, have been working with **concrete types**.\
到目前为止，我们所有的函数和方法都在处理**具体类型**。\
Code that operates on concrete types is usually straightforward to write and understand. But it's also
limited in its reusability.\
对具体类型进行操作的代码通常编写和理解起来都很直接。但它的可重用性也有限。\
Let's imagine, for example, that we want to write a function that returns `true` if an integer is even.
Working with concrete types, we'd have to write a separate function for each integer type we want to
support:
例如，假设我们想编写一个函数，如果整数是偶数则返回 `true`。使用具体类型，我们必须为每个要支持的整数类型编写一个单独的函数：

```rust
fn is_even_i32(n: i32) -> bool {
    n % 2 == 0
}

fn is_even_i64(n: i64) -> bool {
    n % 2 == 0
}

// Etc.
```

Alternatively, we could write a single extension trait and then different implementations for each integer type:
或者，我们可以编写一个扩展特征，然后为每个整数类型编写不同的实现：

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

impl IsEven for i64 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

// Etc.
```

The duplication remains.
重复仍然存在。

## Generic programming
## 泛型编程

We can do better using **generics**.\
我们可以使用**泛型**做得更好。\
Generics allow us to write code that works with a **type parameter** instead of a concrete type:
泛型允许我们编写使用**类型参数**而不是具体类型的代码：

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
{
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```

`print_if_even` is a **generic function**.\
`print_if_even` 是一个**泛型函数**。\
It isn't tied to a specific input type. Instead, it works with any type `T` that:
它不绑定到特定的输入类型。相反，它适用于任何满足以下条件的类型 `T`：

- Implements the `IsEven` trait.
- 实现 `IsEven` 特征。
- Implements the `Debug` trait.
- 实现 `Debug` 特征。

This contract is expressed with a **trait bound**: `T: IsEven + Debug`.\
这个约定通过**特征约束**来表达：`T: IsEven + Debug`。\
The `+` symbol is used to require that `T` implements multiple traits. `T: IsEven + Debug` is equivalent to
"where `T` implements `IsEven` **and** `Debug`".
`+` 符号用于要求 `T` 实现多个特征。`T: IsEven + Debug` 等价于"其中 `T` 实现 `IsEven` **和** `Debug`"。

## Trait bounds
## 特征约束

What purpose do trait bounds serve in `print_if_even`?\
特征约束在 `print_if_even` 中起什么作用？\
To find out, let's try to remove them:
为了找出答案，让我们尝试移除它们：

```rust
fn print_if_even<T>(n: T) {
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```

This code won't compile:
这段代码无法编译：

```text
error[E0599]: no method named `is_even` found for type parameter `T` 
              in the current scope
 --> src/lib.rs:2:10
  |
1 | fn print_if_even<T>(n: T) {
  |                  - method `is_even` not found 
  |                    for this type parameter
2 |     if n.is_even() {
  |          ^^^^^^^ method not found in `T`

error[E0277]: `T` doesn't implement `Debug`
 --> src/lib.rs:3:19
  |
3 |         println!("{n:?} is even");
  |                   ^^^^^ 
  |   `T` cannot be formatted using `{:?}` because 
  |         it doesn't implement `Debug`
  |
help: consider restricting type parameter `T`
  |
1 | fn print_if_even<T: std::fmt::Debug>(n: T) {
  |                   +++++++++++++++++
```

Without trait bounds, the compiler doesn't know what `T` **can do**.\
没有特征约束，编译器不知道 `T` **能做什么**。\
It doesn't know that `T` has an `is_even` method, and it doesn't know how to format `T` for printing.
From the compiler point of view, a bare `T` has no behaviour at all.\
它不知道 `T` 有 `is_even` 方法，也不知道如何格式化 `T` 进行打印。从编译器的角度来看，一个裸的 `T` 没有任何行为。\
Trait bounds restrict the set of types that can be used by ensuring that the behaviour required by the function
body is present.
特征约束通过确保函数体所需的行为存在来限制可以使用的类型集合。

## Syntax: inlining trait bounds
## 语法：内联特征约束

All the examples above used a **`where` clause** to specify trait bounds:
上面的所有示例都使用了 **`where` 子句**来指定特征约束：

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
//  ^^^^^^^^^^^^^^^^^
//  This is a `where` clause
{
    // [...]
}
```

If the trait bounds are simple, you can **inline** them directly next to the type parameter:
如果特征约束很简单，你可以直接将它们**内联**在类型参数旁边：

```rust
fn print_if_even<T: IsEven + Debug>(n: T) {
    //           ^^^^^^^^^^^^^^^^^
    //           This is an inline trait bound
    // [...]
}
```

## Syntax: meaningful names
## 语法：有意义的名称

In the examples above, we used `T` as the type parameter name. This is a common convention when a function has
only one type parameter.\
在上面的示例中，我们使用 `T` 作为类型参数的名称。这是当函数只有一个类型参数时的常见约定。\
Nothing stops you from using a more meaningful name, though:
不过，没有任何东西阻止你使用更有意义的名称：

```rust
fn print_if_even<Number: IsEven + Debug>(n: Number) {
    // [...]
}
```

It is actually **desirable** to use meaningful names when there are multiple type parameters at play or when the name
`T` doesn't convey enough information about the type's role in the function.
当有多个类型参数或名称 `T` 不能传达足够的信息来说明该类型在函数中的作用时，使用有意义的名称实际上是**可取的**。
Maximize clarity and readability when naming type parameters, just as you would with variables or function parameters.
Follow Rust's conventions, though: use [upper camel case for type parameter names](https://rust-lang.github.io/api-guidelines/naming.html#casing-conforms-to-rfc-430-c-case).
在命名类型参数时，应像对待变量或函数参数一样，最大化清晰度和可读性。不过要遵循 Rust 的约定：使用[大驼峰命名法作为类型参数名称](https://rust-lang.github.io/api-guidelines/naming.html#casing-conforms-to-rfc-430-c-case)。

## The function signature is king
## 函数签名是王道

You may wonder why we need trait bounds at all. Can't the compiler infer the required traits from the function's body?\
你可能想知道为什么我们还需要特征约束。编译器不能从函数体中推断出所需特征吗？\
It could, but it won't.\
它可以，但它不会。\
The rationale is the same as for [explicit type annotations on function parameters](../02_basic_calculator/02_variables.md#function-arguments-are-variables):
each function signature is a contract between the caller and the callee, and the terms must be explicitly stated.
This allows for better error messages, better documentation, less unintentional breakages across versions,
and faster compilation times.
其原理与[函数参数上的显式类型注解](../02_basic_calculator/02_variables.md#function-arguments-are-variables)相同：每个函数签名都是调用者和被调用者之间的契约，条款必须明确说明。这样可以提供更好的错误信息、更好的文档、更少的跨版本意外破坏以及更快的编译时间。
