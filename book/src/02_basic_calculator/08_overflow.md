# Overflow
# 溢出

The factorial of a number grows quite fast.\
一个数的阶乘增长得相当快。\
For example, the factorial of 20 is 2,432,902,008,176,640,000. That's already bigger than the maximum value for a
32-bit integer, 2,147,483,647.
例如，20 的阶乘是 2,432,902,008,176,640,000。这已经大于 32 位整数的最大值 2,147,483,647。

When the result of an arithmetic operation is bigger than the maximum value for a given integer type,
we are talking about **an integer overflow**.
当算术运算的结果大于给定整数类型的最大值时，我们称之为**整数溢出**。

Integer overflows are an issue because they violate the contract for arithmetic operations.\
整数溢出是一个问题，因为它们违反了算术运算的约定。\
The result of an arithmetic operation between two integers of a given type should be another integer of the same type.
But the _mathematically correct result_ doesn't fit into that integer type!
给定类型的两个整数之间的算术运算的结果应该是同一类型的另一个整数。但_数学上正确的结果_却放不进那个整数类型！

> If the result is smaller than the minimum value for a given integer type, we refer to the event as **an integer
> underflow**.\
> 如果结果小于给定整数类型的最小值，我们称这种情况为**整数下溢**。\
> For brevity, we'll only talk about integer overflows for the rest of this section, but keep in mind that
> everything we say applies to integer underflows as well.
> 为简洁起见，在本节剩余部分我们将只讨论整数溢出，但请记住，我们所说的一切同样适用于整数下溢。
>
> The `speed` function you wrote in the ["Variables" section](02_variables.md) underflowed for some input
> combinations.
> 你在["变量"部分](02_variables.md)编写的 `speed` 函数在某些输入组合下发生了下溢。
> E.g. if `end` is smaller than `start`, `end - start` will underflow the `u32` type since the result is supposed
> to be negative but `u32` can't represent negative numbers.
> 例如，如果 `end` 小于 `start`，`end - start` 会导致 `u32` 类型下溢，因为结果应该是负数，但 `u32` 不能表示负数。

## No automatic promotion
## 无自动提升

One possible approach would be automatically promote the result to a bigger integer type.
一种可能的方案是自动将结果提升到更大的整数类型。
E.g. if you're summing two `u8` integers and the result is 256 (`u8::MAX + 1`), Rust could choose to interpret the
result as `u16`, the next integer type that's big enough to hold 256.
例如，如果你对两个 `u8` 整数求和，结果是 256（`u8::MAX + 1`），Rust 可以选择将结果解释为 `u16`，即下一个足够容纳 256 的整数类型。

But, as we've discussed before, Rust is quite picky about type conversions. Automatic integer promotion
is not Rust's solution to the integer overflow problem.
但是，正如我们之前讨论过的，Rust 对类型转换相当挑剔。自动整数提升不是 Rust 解决整数溢出问题的方法。

## Alternatives
## 替代方案

Since we ruled out automatic promotion, what can we do when an integer overflow occurs?\
既然我们已经排除了自动提升，那么当整数溢出发生时我们能做什么呢？\
It boils down to two different approaches:
它归结为两种不同的方法：

- Reject the operation
- 拒绝操作
- Come up with a "sensible" result that fits into the expected integer type
- 得出一个适合预期整数类型的"合理"结果

### Reject the operation
### 拒绝操作

This is the most conservative approach: we stop the program when an integer overflow occurs.\
这是最保守的方法：当整数溢出发生时，我们停止程序。\
That's done via a panic, the mechanism we've already seen in the ["Panics" section](04_panics.md).
这是通过恐慌来完成的，我们在["恐慌"部分](04_panics.md)已经看到过这种机制。

### Come up with a "sensible" result
### 得出一个"合理"的结果

When the result of an arithmetic operation is bigger than the maximum value for a given integer type, you can
choose to **wrap around**.\
当算术运算的结果大于给定整数类型的最大值时，你可以选择**回绕(wrap around)**。\
If you think of all the possible values for a given integer type as a circle, wrapping around means that when you
reach the maximum value, you start again from the minimum value.
如果你将给定整数类型的所有可能值想象成一个圆圈，回绕意味着当你达到最大值时，你会从最小值重新开始。

For example, if you do a **wrapping addition** between 1 and 255 (=`u8::MAX`), the result is 0 (=`u8::MIN`).
例如，如果你在 1 和 255（=`u8::MAX`）之间进行**回绕加法**，结果是 0（=`u8::MIN`）。
If you're working with signed integers, the same principle applies. E.g. adding 1 to 127 (=`i8::MAX`) with wrapping
will give you -128 (=`i8::MIN`).
如果你在处理有符号整数，同样的原理也适用。例如，对 127（=`i8::MAX`）加 1 并回绕将得到 -128（=`i8::MIN`）。

## `overflow-checks`
## `overflow-checks`

Rust lets you, the developer, choose which approach to use when an integer overflow occurs.
Rust 让你（开发者）选择当整数溢出发生时使用哪种方法。
The behaviour is controlled by the `overflow-checks` profile setting.
行为由 `overflow-checks` 配置文件设置控制。

If `overflow-checks` is set to `true`, Rust will **panic at runtime** when an integer operation overflows.
如果将 `overflow-checks` 设置为 `true`，当整数运算溢出时，Rust **会在运行时恐慌**。
If `overflow-checks` is set to `false`, Rust will **wrap around** when an integer operation overflows.
如果将 `overflow-checks` 设置为 `false`，当整数运算溢出时，Rust **会回绕**。

You may be wondering—what is a profile setting? Let's get into that!
你可能想知道——什么是配置文件设置？让我们来了解一下！

## Profiles
## 配置文件

A [**profile**](https://doc.rust-lang.org/cargo/reference/profiles.html) is a set of configuration options that can be
used to customize the way Rust code is compiled.
[**配置文件(profile)**](https://doc.rust-lang.org/cargo/reference/profiles.html)是一组可用于自定义 Rust 代码编译方式的配置选项。

Cargo provides 4 built-in profiles: `dev`, `release`, `test`, and `bench`.\
Cargo 提供 4 个内置配置文件：`dev`、`release`、`test` 和 `bench`。\
The `dev` profile is used every time you run `cargo build`, `cargo run` or `cargo test`. It's aimed at local
development,
therefore it sacrifices runtime performance in favor of faster compilation times and a better debugging experience.\
`dev` 配置文件在你每次运行 `cargo build`、`cargo run` 或 `cargo test` 时使用。它针对本地开发，因此牺牲运行时性能以换取更快的编译时间和更好的调试体验。\
The `release` profile, instead, is optimized for runtime performance but incurs longer compilation times. You need
to explicitly request via the `--release` flag—e.g. `cargo build --release` or `cargo run --release`.
而 `release` 配置文件则针对运行时性能进行了优化，但编译时间更长。你需要通过 `--release` 标志显式请求——例如 `cargo build --release` 或 `cargo run --release`。
The `test` profile is the default profile used by `cargo test`. The `test` profile inherits the settings from the `dev` profile.
`test` 配置文件是 `cargo test` 使用的默认配置文件。`test` 配置文件继承 `dev` 配置文件的设置。
The `bench` profile is the default profile used by `cargo bench`. The `bench` profile inherits from the `release` profile.
`bench` 配置文件是 `cargo bench` 使用的默认配置文件。`bench` 配置文件继承 `release` 配置文件的设置。
Use `dev` for iterative development and debugging, `release` for optimized production builds,\
`test` for correctness testing, and `bench` for performance benchmarking.
使用 `dev` 进行迭代开发和调试，`release` 用于优化的生产构建，`test` 用于正确性测试，`bench` 用于性能基准测试。

> "Have you built your project in release mode?" is almost a meme in the Rust community.\
> "你在 release 模式下构建你的项目了吗？"这几乎是 Rust 社区中的一个梗。\
> It refers to developers who are not familiar with Rust and complain about its performance on
> social media (e.g. Reddit, Twitter) before realizing they haven't built their project in
> release mode.
> 它指的是那些不熟悉 Rust 的开发者，他们在社交媒体（如 Reddit、Twitter）上抱怨其性能，然后才意识到他们没有在 release 模式下构建项目。

You can also define custom profiles or customize the built-in ones.
你也可以定义自定义配置文件或自定义内置配置文件。

### `overflow-check`
### `overflow-check`

By default, `overflow-checks` is set to:
默认情况下，`overflow-checks` 设置为：

- `true` for the `dev` profile
- `true` 用于 `dev` 配置文件
- `false` for the `release` profile
- `false` 用于 `release` 配置文件

This is in line with the goals of the two profiles.\
这与两个配置文件的目标一致。\
`dev` is aimed at local development, so it panics in order to highlight potential issues as early as possible.\
`dev` 针对本地开发，因此它通过恐慌来尽早突出潜在问题。\
`release`, instead, is tuned for runtime performance: checking for overflows would slow down the program, so it
prefers to wrap around.
而 `release` 则针对运行时性能进行了调优：检查溢出会减慢程序速度，因此它更倾向于回绕。

At the same time, having different behaviours for the two profiles can lead to subtle bugs.\
同时，两个配置文件的不同行为可能导致微妙的错误。\
Our recommendation is to enable `overflow-checks` for both profiles: it's better to crash than to silently produce
incorrect results. The runtime performance hit is negligible in most cases; if you're working on a performance-critical
application, you can run benchmarks to decide if it's something you can afford.
我们的建议是为两个配置文件都启用 `overflow-checks`：崩溃比默默产生错误结果要好。在大多数情况下，运行时性能影响可以忽略不计；如果你在处理性能关键的应用程序，可以运行基准测试来决定是否能承受这个开销。

## Further reading
## 扩展阅读

- Check out ["Myths and legends about integer overflow in Rust"](https://huonw.github.io/blog/2016/04/myths-and-legends-about-integer-overflow-in-rust/)
  for an in-depth discussion about integer overflow in Rust.
- 查看 ["Myths and legends about integer overflow in Rust"](https://huonw.github.io/blog/2016/04/myths-and-legends-about-integer-overflow-in-rust/) 以深入了解 Rust 中的整数溢出问题。
