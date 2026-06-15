# Generics and associated types
# 泛型和关联类型

Let's re-examine the definition for two of the traits we studied so far, `From` and `Deref`:
让我们重新审视我们目前研究过的两个特征的定义，`From` 和 `Deref`：

```rust
pub trait From<T> {
    fn from(value: T) -> Self;
}

pub trait Deref {
    type Target;
    
    fn deref(&self) -> &Self::Target;
}
```

They both feature type parameters.\
它们都具有类型参数。\
In the case of `From`, it's a generic parameter, `T`.\
对于 `From`，它是一个泛型参数 `T`。\
In the case of `Deref`, it's an associated type, `Target`.
对于 `Deref`，它是一个关联类型 `Target`。

What's the difference? Why use one over the other?
有什么区别？为什么使用一个而不是另一个？

## At most one implementation
## 至多一个实现

Due to how deref coercion works, there can only be one "target" type for a given type. E.g. `String` can
only deref to `str`.
由于解引用强制转换的工作方式，给定类型只能有一个"目标"类型。例如，`String` 只能解引用为 `str`。
It's about avoiding ambiguity: if you could implement `Deref` multiple times for a type,
which `Target` type should the compiler choose when you call a `&self` method?
这是关于避免歧义：如果你可以为某个类型多次实现 `Deref`，那么当你调用 `&self` 方法时，编译器应该选择哪个 `Target` 类型？

That's why `Deref` uses an associated type, `Target`.\
这就是为什么 `Deref` 使用关联类型 `Target`。\
An associated type is uniquely determined **by the trait implementation**.
Since you can't implement `Deref` more than once, you'll only be able to specify one `Target` for a given type
and there won't be any ambiguity.
关联类型是由**特征实现**唯一确定的。由于你不能多次实现 `Deref`，你只能为给定类型指定一个 `Target`，这样就不会有任何歧义。

## Generic traits
## 泛型特征

On the other hand, you can implement `From` multiple times for a type, **as long as the input type `T` is different**.
For example, you can implement `From` for `WrappingU32` using both `u32` and `u16` as input types:
另一方面，你可以为一个类型多次实现 `From`，**只要输入类型 `T` 不同**。例如，你可以使用 `u32` 和 `u16` 作为输入类型为 `WrappingU32` 实现 `From`：

```rust
impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { inner: value }
    }
}

impl From<u16> for WrappingU32 {
    fn from(value: u16) -> Self {
        WrappingU32 { inner: value.into() }
    }
}
```

This works because `From<u16>` and `From<u32>` are considered **different traits**.\
这是可行的，因为 `From<u16>` 和 `From<u32>` 被认为是**不同的特征**。\
There is no ambiguity: the compiler can determine which implementation to use based on type of the value being converted.
没有歧义：编译器可以根据被转换值的类型来确定使用哪个实现。

## Case study: `Add`
## 案例分析：`Add`

As a closing example, consider the `Add` trait from the standard library:
作为结束示例，考虑标准库中的 `Add` 特征：

```rust
pub trait Add<RHS = Self> {
    type Output;
    
    fn add(self, rhs: RHS) -> Self::Output;
}
```

It uses both mechanisms:
它同时使用了两种机制：

- it has a generic parameter, `RHS` (right-hand side), which defaults to `Self`
- 它有一个泛型参数 `RHS`（右侧），默认为 `Self`
- it has an associated type, `Output`, the type of the result of the addition
- 它有一个关联类型 `Output`，即加法的结果类型

### `RHS`
### `RHS`

`RHS` is a generic parameter to allow for different types to be added together.\
`RHS` 是一个泛型参数，允许将不同类型相加。\
For example, you'll find these two implementations in the standard library:
例如，你会在标准库中找到这两个实现：

```rust
impl Add<u32> for u32 {
    type Output = u32;
    
    fn add(self, rhs: u32) -> u32 {
      //                      ^^^
      // This could be written as `Self::Output` instead.
      // The compiler doesn't care, as long as the type you
      // specify here matches the type you assigned to `Output` 
      // right above.
      // [...]
    }
}

impl Add<&u32> for u32 {
    type Output = u32;
    
    fn add(self, rhs: &u32) -> u32 {
        // [...]
    }
}
```

This allows the following code to compile:
这允许以下代码编译：

```rust
let x = 5u32 + &5u32 + 6u32;
```

because `u32` implements `Add<&u32>` _as well as_ `Add<u32>`.
因为 `u32` 实现了 `Add<&u32>` _以及_ `Add<u32>`。

### `Output`
### `Output`

`Output` represents the type of the result of the addition.
`Output` 表示加法结果的类型。

Why do we need `Output` in the first place? Can't we just use `Self` as output, the type implementing `Add`?
We could, but it would limit the flexibility of the trait. In the standard library, for example, you'll find
this implementation:
为什么我们首先需要 `Output`？我们不能直接使用 `Self` 作为输出，即实现 `Add` 的类型吗？我们可以，但这会限制特征的灵活性。例如，在标准库中，你会发现这个实现：

```rust
impl Add<&u32> for &u32 {
    type Output = u32;

    fn add(self, rhs: &u32) -> u32 {
        // [...]
    }
}
```

The type they're implementing the trait for is `&u32`, but the result of the addition is `u32`.\
他们为其实现特征的类型是 `&u32`，但加法的结果是 `u32`。\
It would be impossible[^flexible] to provide this implementation if `add` had to return `Self`, i.e. `&u32` in this case.
如果 `add` 必须返回 `Self`（即在这种情况下是 `&u32`），就不可能提供这个实现了。
`Output` lets `std` decouple the implementor from the return type, thus supporting this case.
`Output` 让 `std` 将实现者与返回类型解耦，从而支持这种情况。

On the other hand, `Output` can't be a generic parameter. The output type of the operation **must** be uniquely determined
once the types of the operands are known. That's why it's an associated type: for a given combination of implementor
and generic parameters, there is only one `Output` type.
另一方面，`Output` 不能是泛型参数。一旦操作数的类型已知，操作的输出类型**必须**被唯一确定。这就是为什么它是一个关联类型：对于给定的实现者和泛型参数的组合，只有一个 `Output` 类型。

## Conclusion
## 结论

To recap:
总结一下：

- Use an **associated type** when the type must be uniquely determined for a given trait implementation.
- 当类型必须为给定的特征实现唯一确定时，使用**关联类型**。
- Use a **generic parameter** when you want to allow multiple implementations of the trait for the same type,
  with different input types.
- 当你想要为同一类型允许该特征的多个实现（具有不同的输入类型）时，使用**泛型参数**。

[^flexible]: Flexibility is rarely free: the trait definition is more complex due to `Output`, and implementors have to reason about
what they want to return. The trade-off is only justified if that flexibility is actually needed. Keep that in mind
when designing your own traits.
[^flexible]: 灵活性很少是免费的：由于 `Output`，特征定义更加复杂，实现者必须考虑他们想要返回什么。只有在实际需要这种灵活性时，这种权衡才是合理的。在设计自己的特征时，请记住这一点。
