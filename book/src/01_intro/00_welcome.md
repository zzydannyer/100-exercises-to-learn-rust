# Welcome
# 欢迎

Welcome to **"100 Exercises To Learn Rust"**!
欢迎来到 **"100 个练习学会 Rust"**！

This course will teach you Rust's core concepts, one exercise at a time.\
本课程将通过一次一个练习的方式，教你 Rust 的核心概念。\
You'll learn about Rust's syntax, its type system, its standard library, and its ecosystem.
你将学习 Rust 的语法、类型系统、标准库及其生态系统。

We don't assume any prior knowledge of Rust, but we assume you know at least
another programming language.
我们不假设你之前了解 Rust，但我们假设你至少熟悉另一种编程语言。
We also don't assume any prior knowledge of systems programming or memory management. Those
topics will be covered in the course.
我们也不假设你有系统编程或内存管理方面的知识。这些话题将在课程中讲解。

In other words, we'll be starting from scratch!\
换句话说，我们将从零开始！\
You'll build up your Rust knowledge in small, manageable steps.
你将通过小而可行的步骤逐步建立你的 Rust 知识。
By the end of the course, you will have solved ~100 exercises, enough to
feel comfortable working on small to medium-sized Rust projects.
到课程结束时，你将完成约 100 个练习，足以让你在处理中小型 Rust 项目时感到得心应手。

## Methodology
## 方法论

This course is based on the "learn by doing" principle.\
本课程基于"在实践中学习"的原则。\
It has been designed to be interactive and hands-on.
课程设计为互动式和动手实践。

[Mainmatter](https://mainmatter.com/rust-consulting/) developed this course
to be delivered in a classroom setting, over 4 days: each attendee advances
through the lessons at their own pace, with an experienced instructor providing
guidance, answering questions and diving deeper into the topics as needed.\
[Mainmatter](https://mainmatter.com/rust-consulting/) 开发了这门课程，在课堂环境中授课，为期四天：每位学员按照自己的进度学习课程，由经验丰富的讲师提供指导、回答问题并根据需要深入讲解相关主题。\
You can sign up for the next tutored session on [our website](https://ti.to/mainmatter/rust-from-scratch-jan-2025).
你可以在[我们的网站](https://ti.to/mainmatter/rust-from-scratch-jan-2025)上报名参加下一期辅导课程。
If you'd like to organise a private session for your company, please [get in touch](https://mainmatter.com/contact/).
如果你想为你的公司组织私人课程，请[联系我们](https://mainmatter.com/contact/)。

You can also take the courses on your own, but we recommend you find a friend or
a mentor to help you along the way should you get stuck. You can
find solutions for all exercises in the
[`solutions` branch of the GitHub repository](https://github.com/mainmatter/100-exercises-to-learn-rust/tree/solutions).
你也可以自学这门课程，但我们建议你找一个朋友或导师在你遇到困难时帮助你。你可以在
[GitHub 仓库的 `solutions` 分支](https://github.com/mainmatter/100-exercises-to-learn-rust/tree/solutions)中找到所有练习的解答。

## Formats
## 格式

You can go through the course material [in the browser](https://rust-exercises.com/100-exercises/) or [download it as a PDF file](https://rust-exercises.com/100-exercises-to-learn-rust.pdf), for offline reading.\
你可以[在浏览器中](https://rust-exercises.com/100-exercises/)浏览课程材料，或[下载为 PDF 文件](https://rust-exercises.com/100-exercises-to-learn-rust.pdf)进行离线阅读。\
If you prefer to have the course material printed out, [buy a paperback copy on Amazon](https://www.amazon.com/dp/B0DJ14KQQG/).
如果你更喜欢打印版的课程材料，可以在[亚马逊上购买平装版](https://www.amazon.com/dp/B0DJ14KQQG/)。

## Structure
## 结构

On the left side of the screen, you can see that the course is divided into sections.
在屏幕左侧，你可以看到课程分为多个部分。
Each section introduces a new concept or feature of the Rust language.\
每个部分介绍 Rust 语言的一个新概念或特性。\
To verify your understanding, each section is paired with an exercise that you need to solve.
为了验证你的理解，每个部分都有一个需要你解决的配套练习。

You can find the exercises in the
[companion GitHub repository](https://github.com/mainmatter/100-exercises-to-learn-rust).\
你可以在[配套的 GitHub 仓库](https://github.com/mainmatter/100-exercises-to-learn-rust)中找到这些练习。\
Before starting the course, make sure to clone the repository to your local machine:
在开始课程之前，请确保将该仓库克隆到你的本地机器上：

```bash
# If you have an SSH key set up with GitHub
git clone git@github.com:mainmatter/100-exercises-to-learn-rust.git
# Otherwise, use the HTTPS URL:
#   https://github.com/mainmatter/100-exercises-to-learn-rust.git
```

We also recommend you work on a branch, so you can easily track your progress and pull
in updates from the main repository, if needed:
我们还建议你在一个分支上工作，这样你可以轻松跟踪进度，并在需要时从主仓库拉取更新：

```bash
cd 100-exercises-to-learn-rust
git checkout -b my-solutions
```

All exercises are located in the `exercises` folder.
所有练习都位于 `exercises` 文件夹中。
Each exercise is structured as a Rust package.
每个练习都组织为一个 Rust 包。
The package contains the exercise itself, instructions on what to do (in `src/lib.rs`), and a test suite to
automatically verify your solution.
包中包含练习本身、操作说明（在 `src/lib.rs` 中），以及一个用于自动验证你的解决方案的测试套件。

### Tools
### 工具

To work through this course, you'll need:
要完成本课程，你需要：

- [**Rust**](https://www.rust-lang.org/tools/install).
  If `rustup` is already installed on your system, run `rustup update` (or another appropriate command depending on how you installed Rust on your system) to ensure you're running on the latest stable version.
  如果你的系统上已经安装了 `rustup`，请运行 `rustup update`（或根据你在系统上安装 Rust 的方式使用其他适当命令）以确保你运行的是最新的稳定版本。
- _(Optional but recommended)_ An IDE with Rust autocompletion support.
  _(可选但推荐)_ 一个支持 Rust 自动完成的 IDE。
  We recommend one of the following:
  我们推荐以下之一：
  - [RustRover](https://www.jetbrains.com/rust/);
  - [Visual Studio Code](https://code.visualstudio.com) with the [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension.
  - 安装了 [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) 扩展的 [Visual Studio Code](https://code.visualstudio.com)。

### Workshop runner, `wr`
### 工作坊运行器，`wr`

To verify your solutions, we've also provided a tool to guide you through the course: the `wr` CLI, short for "workshop runner".
为了验证你的解决方案，我们还提供了一个工具来引导你完成课程：`wr` 命令行工具，即"workshop runner"的缩写。
Install `wr` by following the instructions on [its website](https://mainmatter.github.io/rust-workshop-runner/).
按照[其网站](https://mainmatter.github.io/rust-workshop-runner/)上的说明安装 `wr`。

Once you have `wr` installed, open a new terminal and navigate to the top-level folder of the repository.
安装好 `wr` 后，打开一个新的终端并导航到仓库的顶层文件夹。
Run the `wr` command to start the course:
运行 `wr` 命令开始课程：

```bash
wr
```

`wr` will verify the solution to the current exercise.\
`wr` 会验证当前练习的解决方案。\
Don't move on to the next section until you've solved the exercise for the current one.
在完成当前部分的练习之前，不要进入下一部分。

> We recommend committing your solutions to Git as you progress through the course,
> so you can easily track your progress and "restart" from a known point if needed.
> 我们建议你在学习过程中将解决方案提交到 Git，这样你可以轻松跟踪进度，并在需要时从已知点"重新开始"。

Enjoy the course!
祝你在课程中玩得开心！

## Author
## 作者

This course was written by [Luca Palmieri](https://www.lpalmieri.com/), Principal Engineering
Consultant at [Mainmatter](https://mainmatter.com/rust-consulting/).\
本课程由 [Luca Palmieri](https://www.lpalmieri.com/) 编写，他是 [Mainmatter](https://mainmatter.com/rust-consulting/) 的首席工程顾问。\
Luca has been working with Rust since 2018, initially at TrueLayer and then at AWS.\
Luca 自 2018 年以来一直从事 Rust 开发，最初在 TrueLayer，后来在 AWS。\
Luca is the author of ["Zero to Production in Rust"](https://zero2prod.com),
the go-to resource for learning how to build backend applications in Rust.\
Luca 是 ["Zero to Production in Rust"](https://zero2prod.com) 的作者，这是学习如何使用 Rust 构建后端应用程序的首选资源。\
He is also the author and maintainer of a variety of open-source Rust projects, including
[`cargo-chef`](https://github.com/LukeMathWalker/cargo-chef),
[Pavex](https://pavex.dev) and [`wiremock`](https://github.com/LukeMathWalker/wiremock-rs).
他还是多个开源 Rust 项目的作者和维护者，包括 [`cargo-chef`](https://github.com/LukeMathWalker/cargo-chef)、[Pavex](https://pavex.dev) 和 [`wiremock`](https://github.com/LukeMathWalker/wiremock-rs)。
