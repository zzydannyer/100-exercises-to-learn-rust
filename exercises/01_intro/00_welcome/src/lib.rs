// This is a Rust file. It is a plain text file with a `.rs` extension.
// 这是一个 Rust 文件。它是一个扩展名为 `.rs` 的纯文本文件。
//
// Like most modern programming languages, Rust supports comments. You're looking at one right now!
// 像大多数现代编程语言一样，Rust 支持注释。你现在看到的就是一个！
// Comments are ignored by the compiler; you can leverage them to annotate code with notes and
// 注释会被编译器忽略；你可以利用它们为代码添加注释和说明。
// explanations.
// 注释会被编译器忽略；你可以利用它们为代码添加注释和说明。
// There are various ways to write comments in Rust, each with its own purpose.
// Rust 中有多种编写注释的方式，每种都有其特定用途。
// For now we'll stick to the most common one: the line comment.
// 现在我们只使用最常见的一种：行注释。
// Everything from `//` to the end of the line is considered a comment.
// 从 `//` 到行尾的所有内容都被视为注释。

// Exercises will include `TODO`, `todo!()` or `__` markers to draw your attention to the lines
// 练习中会包含 `TODO`、`todo!()` 或 `__` 标记，以引起你对需要编写代码的行的注意。
// where you need to write code.
// 练习中会包含 `TODO`、`todo!()` 或 `__` 标记，以引起你对需要编写代码的行的注意。
// You'll need to replace these markers with your own code to complete the exercise.
// 你需要用你自己的代码替换这些标记来完成练习。
// Sometimes it'll be enough to write a single line of code, other times you'll have to write
// 有时只需编写一行代码就足够了，其他时候则需要编写更长的代码段。
// longer sections.
// 有时只需编写一行代码就足够了，其他时候则需要编写更长的代码段。
//
// If you get stuck for more than 10 minutes on an exercise, grab a trainer! We're here to help!
// 如果你在某道练习上卡住超过 10 分钟，去找教练！我们随时准备提供帮助！
// You can also find solutions to all exercises in the `solutions` git branch.
// 你也可以在 `solutions` git 分支中找到所有练习的解答。
fn greeting() -> &'static str {
    // TODO: fix me 👇
    // TODO: 修复我 👇
    "I'm ready to learn Rust!"
}

// Your solutions will be automatically verified by a set of tests.
// 你的解答将由一组测试自动验证。
// You can run these tests directly by invoking the `cargo test` command in your terminal,
// 你可以通过在终端中从练习目录的根目录运行 `cargo test` 命令来直接执行这些测试。
// from the root of this exercise's directory. That's what the `wr` command does for you
// 这就是 `wr` 命令在底层为你做的事情。
// under the hood.
// 这就是 `wr` 命令在底层为你做的事情。
//
// Rust lets you write tests alongside your code.
// Rust 允许你在代码旁边编写测试。
// The `#[cfg(test)]` attribute tells the compiler to only compile the code below when
// `#[cfg(test)]` 属性告诉编译器仅在运行测试时编译以下代码。
// running tests (i.e. when you run `cargo test`).
// `#[cfg(test)]` 属性告诉编译器仅在运行测试时编译以下代码。
// You'll learn more about attributes and testing later in the course.
// 你将在课程后面学到更多关于属性和测试的知识。
// For now, just know that you need to look for the `#[cfg(test)]` attribute to find the tests
// 现在，你只需要知道你需要寻找 `#[cfg(test)]` 属性来找到将验证你解答正确性的测试！
// that will be verifying the correctness of your solutions!
// 现在，你只需要知道你需要寻找 `#[cfg(test)]` 属性来找到将验证你解答正确性的测试！
//
// ⚠️ **DO NOT MODIFY THE TESTS** ⚠️
// ⚠️ **不要修改测试** ⚠️
// They are there to help you validate your solutions. You should only change the code that's being
// 它们在那里是为了帮助你验证你的解答。你应该只修改被测试的代码，而不是测试本身。
// tested, not the tests themselves.
// 它们在那里是为了帮助你验证你的解答。你应该只修改被测试的代码，而不是测试本身。
#[cfg(test)]
mod tests {
    use crate::greeting;

    #[test]
    fn test_welcome() {
        assert_eq!(greeting(), "I'm ready to learn Rust!");
    }
}
