# `HashMap`
# `HashMap`

Our implementation of `Index`/`IndexMut` is not ideal: we need to iterate over the entire
`Vec` to retrieve a ticket by id; the algorithmic complexity is `O(n)`, where
`n` is the number of tickets in the store.
我们的 `Index`/`IndexMut` 实现并不理想：我们需要遍历整个 `Vec` 来按 ID 检索票据；算法复杂度是 `O(n)`，其中 `n` 是存储中票据的数量。

We can do better by using a different data structure for storing tickets: a `HashMap<K, V>`.
我们可以通过使用不同的数据结构来存储票据：`HashMap<K, V>`。

```rust
use std::collections::HashMap;

// Type inference lets us omit an explicit type signature (which
// would be `HashMap<String, String>` in this example).
let mut book_reviews = HashMap::new();

book_reviews.insert(
    "Adventures of Huckleberry Finn".to_string(),
    "My favorite book.".to_string(),
);
```

`HashMap` works with key-value pairs. It's generic over both: `K` is the generic
parameter for the key type, while `V` is the one for the value type.
`HashMap` 使用键值对。它对两者都是泛型的：`K` 是键类型的泛型参数，而 `V` 是值类型的泛型参数。

The expected cost of insertions, retrievals and removals is **constant**, `O(1)`.
That sounds perfect for our usecase, doesn't it?
插入、检索和删除的预期成本是**常数** `O(1)`。这听起来很适合我们的用例，不是吗？

## Key requirements
## 键的要求

There are no trait bounds on `HashMap`'s struct definition, but you'll find some
on its methods. Let's look at `insert`, for example:
`HashMap` 的结构体定义上没有特征约束，但你会在其方法上找到一些。让我们看一下 `insert`，例如：

```rust
// Slightly simplified
impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        // [...]
    }
}
```

The key type must implement the `Eq` and `Hash` traits.\
键类型必须实现 `Eq` 和 `Hash` 特征。\
Let's dig into those two.
让我们深入了解这两个特征。

## `Hash`
## `Hash`

A hashing function (or hasher) maps a potentially infinite set of a values (e.g.
all possible strings) to a bounded range (e.g. a `u64` value).\
哈希函数（或哈希器）将可能无限的值集（例如所有可能的字符串）映射到有限的范围（例如 `u64` 值）。\
There are many different hashing functions around, each with different properties
(speed, collision risk, reversibility, etc.).
有许多不同的哈希函数，每种都有不同的属性（速度、碰撞风险、可逆性等）。

A `HashMap`, as the name suggests, uses a hashing function behind the scene.
It hashes your key and then uses that hash to store/retrieve the associated value.
This strategy requires the key type must be hashable, hence the `Hash` trait bound on `K`.
顾名思义，`HashMap` 在幕后使用哈希函数。它对你的键进行哈希，然后使用该哈希来存储/检索关联的值。这种策略要求键类型必须是可哈希的，因此对 `K` 有 `Hash` 特征约束。

You can find the `Hash` trait in the `std::hash` module:
你可以在 `std::hash` 模块中找到 `Hash` 特征：

```rust
pub trait Hash {
    // Required method
    fn hash<H>(&self, state: &mut H)
       where H: Hasher;
}
```

You will rarely implement `Hash` manually. Most of the times you'll derive it:
你很少会手动实现 `Hash`。大多数时候你会派生它：

```rust
#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
}
```

## `Eq`
## `Eq`

`HashMap` must be able to compare keys for equality. This is particularly important
when dealing with hash collisions—i.e. when two different keys hash to the same value.
`HashMap` 必须能够比较键的相等性。这在处理哈希冲突时特别重要——即当两个不同的键哈希到相同的值时。

You may wonder: isn't that what the `PartialEq` trait is for? Almost!\
你可能想知道：这不就是 `PartialEq` 特征的用途吗？差不多！\
`PartialEq` is not enough for `HashMap` because it doesn't guarantee reflexivity, i.e. `a == a` is always `true`.\
`PartialEq` 对于 `HashMap` 来说是不够的，因为它不保证自反性，即 `a == a` 总是 `true`。\
For example, floating point numbers (`f32` and `f64`) implement `PartialEq`,
but they don't satisfy the reflexivity property: `f32::NAN == f32::NAN` is `false`.\
例如，浮点数（`f32` 和 `f64`）实现了 `PartialEq`，但它们不满足自反性属性：`f32::NAN == f32::NAN` 是 `false`。\
Reflexivity is crucial for `HashMap` to work correctly: without it, you wouldn't be able to retrieve a value
from the map using the same key you used to insert it.
自反性对于 `HashMap` 正确工作至关重要：没有它，你将无法使用插入时使用的相同键从映射中检索值。

The `Eq` trait extends `PartialEq` with the reflexivity property:
`Eq` 特征用自反性属性扩展了 `PartialEq`：

```rust
pub trait Eq: PartialEq {
    // No additional methods
}
```

It's a marker trait: it doesn't add any new methods, it's just a way for you to say to the compiler
that the equality logic implemented in `PartialEq` is reflexive.
它是一个标记特征：它不添加任何新方法，它只是你向编译器声明 `PartialEq` 中实现的相等逻辑是自反的一种方式。

You can derive `Eq` automatically when you derive `PartialEq`:
当你派生 `PartialEq` 时，你可以自动派生 `Eq`：

```rust
#[derive(PartialEq, Eq)]
struct Person {
    id: u32,
    name: String,
}
```

## `Eq` and `Hash` are linked
## `Eq` 和 `Hash` 是关联的

There is an implicit contract between `Eq` and `Hash`: if two keys are equal, their hashes must be equal too.
This is crucial for `HashMap` to work correctly. If you break this contract, you'll get nonsensical results
when using `HashMap`.
`Eq` 和 `Hash` 之间有一个隐式约定：如果两个键相等，那么它们的哈希也必须相等。这对于 `HashMap` 正确工作至关重要。如果你破坏了这个约定，使用 `HashMap` 时会得到无意义的结果。
