# Variables
# 变量

In Rust, you can use the `let` keyword to declare **variables**.\
在 Rust 中，你可以使用 `let` 关键字声明**变量**。\
For example:
例如：

```rust
let x = 42;
```

Above we defined a variable `x` and assigned it the value `42`.
上面我们定义了一个变量 `x` 并为其赋值 `42`。

## Type
## 类型

Every variable in Rust must have a type. It can either be inferred by the compiler or explicitly specified by the
developer.
Rust 中的每个变量都必须有一个类型。它要么由编译器推断，要么由开发者显式指定。

### Explicit type annotation
### 显式类型标注

You can specify the variable type by adding a colon `:` followed by the type after the variable name. For example:
你可以通过在变量名后添加冒号 `:` 和类型来指定变量类型。例如：

```rust
// let <variable_name>: <type> = <expression>;
let x: u32 = 42;
```

In the example above, we explicitly constrained the type of `x` to be `u32`.
在上面的例子中，我们显式地将 `x` 的类型约束为 `u32`。

### Type inference
### 类型推断

If we don't specify the type of a variable, the compiler will try to infer it based on the context in which the variable
is used.
如果我们不指定变量的类型，编译器会尝试根据变量使用的上下文来推断它。

```rust
let x = 42;
let y: u32 = x;
```

In the example above, we didn't specify the type of `x`.\
在上面的例子中，我们没有指定 `x` 的类型。\
`x` is later assigned to `y`, which is explicitly typed as `u32`. Since Rust doesn't perform automatic type coercion,
the compiler infers the type of `x` to be `u32`—the same as `y` and the only type that will allow the program to compile
without errors.
`x` 随后被赋给 `y`，而 `y` 被显式类型化为 `u32`。由于 Rust 不执行自动类型转换，编译器推断 `x` 的类型为 `u32`——与 `y` 相同，也是唯一能让程序无错误编译的类型。

### Inference limitations
### 推断的限制

The compiler sometimes needs a little help to infer the correct variable type based on its usage.\
编译器有时需要一些帮助来根据变量的使用情况推断正确的类型。\
In those cases you'll get a compilation error and the compiler will ask you to provide an explicit type hint to
disambiguate the situation.
在这些情况下，你会得到一个编译错误，编译器会要求你提供一个显式的类型提示来澄清情况。

## Function arguments are variables
## 函数参数也是变量

Not all heroes wear capes, not all variables are declared with `let`.\
不是所有英雄都披斗篷，不是所有变量都用 `let` 声明。\
Function arguments are variables too!
函数参数也是变量！

```rust
fn add_one(x: u32) -> u32 {
    x + 1
}
```

In the example above, `x` is a variable of type `u32`.\
在上面的例子中，`x` 是一个 `u32` 类型的变量。\
The only difference between `x` and a variable declared with `let` is that functions arguments **must** have their type
explicitly declared. The compiler won't infer it for you.\
`x` 和用 `let` 声明的变量之间的唯一区别是，函数参数**必须**显式声明其类型。编译器不会为你推断。\
This constraint allows the Rust compiler (and us humans!) to understand the function's signature without having to look
at its implementation. That's a big boost for compilation speed[^speed]!
这个约束让 Rust 编译器（以及我们人类！）无需查看函数实现就能理解其签名。这对编译速度[^speed]是一个很大的提升！

## Initialization
## 初始化

You don't have to initialize a variable when you declare it.\
你不必在声明变量时初始化它。\
For example
例如

```rust
let x: u32;
```

is a valid variable declaration.\
是一个有效的变量声明。\
However, you must initialize the variable before using it. The compiler will throw an error if you don't:
然而，你必须在使用变量之前初始化它。如果你不这样做，编译器会抛出一个错误：

```rust
let x: u32;
let y = x + 1;
```

will throw a compilation error:
会抛出一个编译错误：

```text
error[E0381]: used binding `x` isn't initialized
 --> src/main.rs:3:9
  |
2 | let x: u32;
  |     - binding declared here but left uninitialized
3 | let y = x + 1;
  |         ^ `x` used here but it isn't initialized
  |
help: consider assigning a value
  |
2 | let x: u32 = 0;
  |            +++
```

[^speed]: The Rust compiler needs all the help it can get when it comes to compilation speed.
[^speed]: 在编译速度方面，Rust 编译器需要所有能获得的帮助。
