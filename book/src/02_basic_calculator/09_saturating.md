# Case-by-case behavior
# 逐情况处理

`overflow-checks` is a blunt tool: it's a global setting that affects the whole program.\
`overflow-checks` 是一个粗糙的工具：它是一个影响整个程序的全局设置。\
It often happens that you want to handle integer overflows differently depending on the context: sometimes
wrapping is the right choice, other times panicking is preferable.
你经常会希望根据上下文以不同方式处理整数溢出：有时回绕是正确的选择，有时恐慌更可取。

## `wrapping_` methods
## `wrapping_` 方法

You can opt into wrapping arithmetic on a per-operation basis by using the `wrapping_` methods[^method].\
你可以通过使用 `wrapping_` 方法[^method]在每个操作的基础上选择回绕算术。\
For example, you can use `wrapping_add` to add two integers with wrapping:
例如，你可以使用 `wrapping_add` 进行两个整数的回绕加法：

```rust
let x = 255u8;
let y = 1u8;
let sum = x.wrapping_add(y);
assert_eq!(sum, 0);
```

## `saturating_` methods
## `saturating_` 方法

Alternatively, you can opt into **saturating arithmetic** by using the `saturating_` methods.\
或者，你可以通过使用 `saturating_` 方法来选择**饱和算术**。\
Instead of wrapping around, saturating arithmetic will return the maximum or minimum value for the integer type.
饱和算术会返回整数类型的最大值或最小值，而不是回绕。
For example:
例如：

```rust
let x = 255u8;
let y = 1u8;
let sum = x.saturating_add(y);
assert_eq!(sum, 255);
```

Since `255 + 1` is `256`, which is bigger than `u8::MAX`, the result is `u8::MAX` (255).\
由于 `255 + 1` 是 `256`，大于 `u8::MAX`，所以结果是 `u8::MAX`（255）。\
The opposite happens for underflows: `0 - 1` is `-1`, which is smaller than `u8::MIN`, so the result is `u8::MIN` (0).
下溢的情况相反：`0 - 1` 是 `-1`，小于 `u8::MIN`，所以结果是 `u8::MIN`（0）。

You can't get saturating arithmetic via the `overflow-checks` profile setting—you have to explicitly opt into it
when performing the arithmetic operation.
你不能通过 `overflow-checks` 配置文件设置来获得饱和算术——你必须在执行算术运算时显式选择它。

[^method]: You can think of methods as functions that are "attached" to a specific type.
We'll cover methods (and how to define them) in the next chapter.
[^method]: 你可以把方法看作是"附加"到特定类型上的函数。我们将在下一章介绍方法（以及如何定义它们）。
