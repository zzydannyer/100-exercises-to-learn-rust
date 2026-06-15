# Combinators
# 组合器

Iterators can do so much more than `for` loops!\
迭代器能做的远不止 `for` 循环！\
If you look at the documentation for the `Iterator` trait, you'll find a **vast** collection of
methods that you can leverage to transform, filter, and combine iterators in various ways.
如果你查看 `Iterator` 特征的文档，你会发现大量的方法，你可以利用它们以各种方式转换、过滤和组合迭代器。

Let's mention the most common ones:
让我们提一下最常见的几种：

- `map` applies a function to each element of the iterator.
- `map` 将一个函数应用于迭代器的每个元素。
- `filter` keeps only the elements that satisfy a predicate.
- `filter` 只保留满足谓词的元素。
- `filter_map` combines `filter` and `map` in one step.
- `filter_map` 将 `filter` 和 `map` 结合在一个步骤中。
- `cloned` converts an iterator of references into an iterator of values, cloning each element.
- `cloned` 将引用迭代器转换为值迭代器，克隆每个元素。
- `enumerate` returns a new iterator that yields `(index, value)` pairs.
- `enumerate` 返回一个生成 `(index, value)` 对的新迭代器。
- `skip` skips the first `n` elements of the iterator.
- `skip` 跳过迭代器的前 `n` 个元素。
- `take` stops the iterator after `n` elements.
- `take` 在 `n` 个元素后停止迭代器。
- `chain` combines two iterators into one.
- `chain` 将两个迭代器组合成一个。

These methods are called **combinators**.\
这些方法被称为**组合器**。\
They are usually **chained** together to create complex transformations in a concise and readable way:
它们通常被**链式**组合在一起，以简洁且可读的方式创建复杂的转换：

```rust
let numbers = vec![1, 2, 3, 4, 5];
// The sum of the squares of the even numbers
let outcome: u32 = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    .sum();
```

## Closures
## 闭包

What's going on with the `filter` and `map` methods above?\
上面的 `filter` 和 `map` 方法是怎么回事？\
They take **closures** as arguments.
它们接受**闭包**作为参数。

Closures are **anonymous functions**, i.e. functions that are not defined using the `fn` syntax we are used to.\
闭包是**匿名函数**，即不是用我们习惯的 `fn` 语法定义的函数。\
They are defined using the `|args| body` syntax, where `args` are the arguments and `body` is the function body.
`body` can be a block of code or a single expression.
它们使用 `|args| body` 语法定义，其中 `args` 是参数，`body` 是函数体。`body` 可以是一个代码块或一个单一表达式。
For example:
例如：

```rust
// An anonymous function that adds 1 to its argument
let add_one = |x| x + 1;
// Could be written with a block too:
let add_one = |x| { x + 1 };
```

Closures can take more than one argument:
闭包可以接受多个参数：

```rust
let add = |x, y| x + y;
let sum = add(1, 2);
```

They can also capture variables from their environment:
它们也可以从其环境中捕获变量：

```rust
let x = 42;
let add_x = |y| x + y;
let sum = add_x(1);
```

If necessary, you can specify the types of the arguments and/or the return type:
如果有必要，你可以指定参数的类型和/或返回类型：

```rust
// Just the input type
let add_one = |x: i32| x + 1;
// Or both input and output types, using the `fn` syntax
let add_one: fn(i32) -> i32 = |x| x + 1;
```

## `collect`
## `collect`

What happens when you're done transforming an iterator using combinators?\
当你使用组合器完成对迭代器的转换后，会发生什么？\
You either iterate over the transformed values using a `for` loop, or you collect them into a collection.
你要么使用 `for` 循环迭代转换后的值，要么将它们收集到一个集合中。

The latter is done using the `collect` method.\
后者使用 `collect` 方法完成。\
`collect` consumes the iterator and collects its elements into a collection of your choice.
`collect` 消耗迭代器并将其元素收集到你选择的集合中。

For example, you can collect the squares of the even numbers into a `Vec`:
例如，你可以将偶数的平方收集到一个 `Vec` 中：

```rust
let numbers = vec![1, 2, 3, 4, 5];
let squares_of_evens: Vec<u32> = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    .collect();
```

`collect` is generic over its **return type**.\
`collect` 对其**返回类型**是泛型的。\
Therefore you usually need to provide a type hint to help the compiler infer the correct type.
因此，你通常需要提供一个类型提示来帮助编译器推断正确的类型。
In the example above, we annotated the type of `squares_of_evens` to be `Vec<u32>`.
Alternatively, you can use the **turbofish syntax** to specify the type:
在上面的例子中，我们将 `squares_of_evens` 的类型注解为 `Vec<u32>`。或者，你可以使用**turbofish 语法**来指定类型：

```rust
let squares_of_evens = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    // Turbofish syntax: `<method_name>::<type>()`
    // It's called turbofish because `::<>` looks like a fish
    .collect::<Vec<u32>>();
```

## Further reading
## 延伸阅读

- [`Iterator`'s documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html) gives you an
  overview of the methods available for iterators in `std`.
- [`Iterator` 的文档](https://doc.rust-lang.org/std/iter/trait.Iterator.html) 为你概述了 `std` 中可用于迭代器的方法。
- [The `itertools` crate](https://docs.rs/itertools/) defines even **more** combinators for iterators.
- [`itertools` crate](https://docs.rs/itertools/) 为迭代器定义了**更多**的组合器。
