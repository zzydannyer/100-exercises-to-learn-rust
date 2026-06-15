# Outro
# 结语

Rust's asynchronous model is quite powerful, but it does introduce additional
complexity. Take time to know your tools: dive deep into `tokio`'s documentation
and get familiar with its primitives to make the most out of it.
Rust 的异步模型相当强大，但确实引入了额外的复杂性。花时间了解你的工具：深入阅读 `tokio` 的文档并熟悉其原语，以充分利用它。

Keep in mind, as well, that there is ongoing work at the language and `std` level
to streamline and "complete" Rust's asynchronous story. You may experience some
rough edges in your day-to-day work due to some of these missing pieces.
同时请记住，语言和 `std` 层面有持续的工作来简化和"完善" Rust 的异步故事。由于这些缺失的部分，你可能会在日常工作中遇到一些粗糙之处。

A few recommendations for a mostly-pain-free async experience:
一些关于获得基本无痛异步体验的建议：

- **Pick a runtime and stick to it.**\
  Some primitives (e.g. timers, I/O) are not portable across runtimes. Trying to
  mix runtimes is likely to cause you pain. Trying to write code that's runtime
  agnostic can significantly increase the complexity of your codebase. Avoid it
  if you can.
- **选择一个运行时并坚持使用它。** 一些原语（例如定时器、I/O）在运行时之间不可移植。尝试混合运行时可能会给你带来痛苦。尝试编写与运行时无关的代码会显著增加代码库的复杂性。如果可以，请避免这样做。
- **There is no stable `Stream`/`AsyncIterator` interface yet.**\
  An `AsyncIterator` is, conceptually, an iterator that yields new items
  asynchronously. There is ongoing design work, but no consensus (yet).
  If you're using `tokio`, refer to [`tokio_stream`](https://docs.rs/tokio-stream/latest/tokio_stream/)
  as your go-to interface.
- **还没有稳定的 `Stream`/`AsyncIterator` 接口。** 概念上，`AsyncIterator` 是一个异步产生新项的迭代器。有持续的设计工作，但还没有达成共识。如果你在使用 `tokio`，请参考 [`tokio_stream`](https://docs.rs/tokio-stream/latest/tokio_stream/) 作为你的首选接口。
- **Be careful with buffering.**\
  It is often the cause of subtle bugs. Check out
  ["Barbara battles buffered streams"](https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/barbara_battles_buffered_streams.html)
  for more details.
- **小心缓冲。** 它往往是微妙错误的根源。查看 ["Barbara battles buffered streams"](https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/barbara_battles_buffered_streams.html) 了解更多细节。
- **There is no equivalent of scoped threads for asynchronous tasks**.\
  Check out ["The scoped task trilemma"](https://without.boats/blog/the-scoped-task-trilemma/)
  for more details.
- **没有与作用域线程等效的异步任务。** 查看 ["The scoped task trilemma"](https://without.boats/blog/the-scoped-task-trilemma/) 了解更多细节。

Don't let these caveats scare you: asynchronous Rust is being used effectively
at _massive_ scale (e.g. AWS, Meta) to power foundational services.\
不要让这些警告吓到你：异步 Rust 正在以_大规模_（例如 AWS、Meta）有效用于支撑基础服务。\
You will have to master it if you're planning building networked applications
in Rust.
如果你计划用 Rust 构建网络应用程序，你将需要掌握它。
