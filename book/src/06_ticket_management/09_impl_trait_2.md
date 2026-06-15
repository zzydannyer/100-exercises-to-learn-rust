# `impl Trait` in argument position
# `impl Trait` 在参数位置

In the previous section, we saw how `impl Trait` can be used to return a type without specifying its name.\
在上一节中，我们看到了如何使用 `impl Trait` 返回一个类型而不指定其名称。\
The same syntax can also be used in **argument position**:
相同的语法也可以用于**参数位置**：

```rust
fn print_iter(iter: impl Iterator<Item = i32>) {
    for i in iter {
        println!("{}", i);
    }
}
```

`print_iter` takes an iterator of `i32`s and prints each element.\
`print_iter` 接受一个 `i32` 的迭代器并打印每个元素。\
When used in **argument position**, `impl Trait` is equivalent to a generic parameter with a trait bound:
当在**参数位置**使用时，`impl Trait` 等价于带有特征约束的泛型参数：

```rust
fn print_iter<T>(iter: T) 
where
    T: Iterator<Item = i32>
{
    for i in iter {
        println!("{}", i);
    }
}
```

## Downsides
## 缺点

As a rule of thumb, prefer generics over `impl Trait` in argument position.\
根据经验法则，在参数位置优先选择泛型而不是 `impl Trait`。\
Generics allow the caller to explicitly specify the type of the argument, using the turbofish syntax (`::<>`),
which can be useful for disambiguation. That's not the case with `impl Trait`.
泛型允许调用者使用 turbofish 语法（`::<>`）显式指定参数的类型，这对于消除歧义很有用。`impl Trait` 则不是这样。
