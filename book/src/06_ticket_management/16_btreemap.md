# Ordering
# 排序

By moving from a `Vec` to a `HashMap` we have improved the performance of our ticket management system,
and simplified our code in the process.\
通过从 `Vec` 迁移到 `HashMap`，我们提高了票据管理系统的性能，并在此过程中简化了代码。\
It's not all roses, though. When iterating over a `Vec`-backed store, we could be sure that the tickets
would be returned in the order they were added.\
不过，并非一切都是美好的。当迭代基于 `Vec` 的存储时，我们可以确定票据会按照添加的顺序返回。\
That's not the case with a `HashMap`: you can iterate over the tickets, but the order is random.
而对于 `HashMap` 则不是这样：你可以迭代票据，但顺序是随机的。

We can recover a consistent ordering by switching from a `HashMap` to a `BTreeMap`.
我们可以通过从 `HashMap` 切换到 `BTreeMap` 来恢复一致的排序。

## `BTreeMap`
## `BTreeMap`

A `BTreeMap` guarantees that entries are sorted by their keys.\
`BTreeMap` 保证条目按键排序。\
This is useful when you need to iterate over the entries in a specific order, or if you need to
perform range queries (e.g. "give me all tickets with an id between 10 and 20").
当你需要按特定顺序迭代条目，或者需要执行范围查询（例如"给我 ID 在 10 到 20 之间的所有票据"）时，这很有用。

Just like `HashMap`, you won't find trait bounds on the definition of `BTreeMap`.
But you'll find trait bounds on its methods. Let's look at `insert`:
就像 `HashMap` 一样，你不会在 `BTreeMap` 的定义上找到特征约束，但你会在其方法上找到。让我们看一下 `insert`：

```rust
// `K` and `V` stand for the key and value types, respectively,
// just like in `HashMap`.
impl<K, V> BTreeMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        // implementation
    }
}
```

`Hash` is no longer required. Instead, the key type must implement the `Ord` trait.
`Hash` 不再是必需的。相反，键类型必须实现 `Ord` 特征。

## `Ord`
## `Ord`

The `Ord` trait is used to compare values.\
`Ord` 特征用于比较值。\
While `PartialEq` is used to compare for equality, `Ord` is used to compare for ordering.
`PartialEq` 用于比较相等性，而 `Ord` 用于比较顺序。

It's defined in `std::cmp`:
它定义在 `std::cmp` 中：

```rust
pub trait Ord: Eq + PartialOrd {
    fn cmp(&self, other: &Self) -> Ordering;
}
```

The `cmp` method returns an `Ordering` enum, which can be one
of `Less`, `Equal`, or `Greater`.\
`cmp` 方法返回一个 `Ordering` 枚举，可以是 `Less`、`Equal` 或 `Greater` 之一。\
`Ord` requires that two other traits are implemented: `Eq` and `PartialOrd`.
`Ord` 要求实现其他两个特征：`Eq` 和 `PartialOrd`。

## `PartialOrd`
## `PartialOrd`

`PartialOrd` is a weaker version of `Ord`, just like `PartialEq` is a weaker version of `Eq`.
You can see why by looking at its definition:
`PartialOrd` 是 `Ord` 的较弱版本，就像 `PartialEq` 是 `Eq` 的较弱版本一样。通过查看其定义，你可以看出原因：

```rust
pub trait PartialOrd: PartialEq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
}
```

`PartialOrd::partial_cmp` returns an `Option`—it is not guaranteed that two values can
be compared.\
`PartialOrd::partial_cmp` 返回一个 `Option`——它不保证两个值可以进行比较。\
For example, `f32` doesn't implement `Ord` because `NaN` values are not comparable,
the same reason why `f32` doesn't implement `Eq`.
例如，`f32` 没有实现 `Ord`，因为 `NaN` 值不可比较，这与 `f32` 没有实现 `Eq` 的原因相同。

## Implementing `Ord` and `PartialOrd`
## 实现 `Ord` 和 `PartialOrd`

Both `Ord` and `PartialOrd` can be derived for your types:
`Ord` 和 `PartialOrd` 都可以为你的类型派生：

```rust
// You need to add `Eq` and `PartialEq` too,
// since `Ord` requires them.
#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct TicketId(u64);
```

If you choose (or need) to implement them manually, be careful:
如果你选择（或需要）手动实现它们，请小心：

- `Ord` and `PartialOrd` must be consistent with `Eq` and `PartialEq`.
- `Ord` 和 `PartialOrd` 必须与 `Eq` 和 `PartialEq` 一致。
- `Ord` and `PartialOrd` must be consistent with each other.
- `Ord` 和 `PartialOrd` 必须彼此一致。
