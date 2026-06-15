# Wrapping up
# 总结

We've covered quite a few different traits in this chapter—and we've only scratched the surface!
我们在本章中涵盖了不少不同的特征——而且我们仅仅触及了表面！
It may feel like you have a lot to remember, but don't worry: you'll bump into these traits
so often when writing Rust code that they'll soon become second nature.
你可能会觉得有很多东西要记，但别担心：你在编写 Rust 代码时会经常遇到这些特征，它们很快就会变成你的第二天性。

## Closing thoughts
## 结束语

Traits are powerful, but don't overuse them.\
特征很强大，但不要过度使用它们。\
A few guidelines to keep in mind:
一些需要记住的指导原则：

- Don't make a function generic if it is always invoked with a single type. It introduces indirection in your
  codebase, making it harder to understand and maintain.
- 如果函数总是使用单一类型调用，不要将其泛型化。这会在你的代码库中引入间接性，使其更难理解和维护。
- Don't create a trait if you only have one implementation. It's a sign that the trait is not needed.
- 如果你只有一个实现，不要创建特征。这表明并不需要该特征。
- Implement standard traits for your types (`Debug`, `PartialEq`, etc.) whenever it makes sense.
  It will make your types more idiomatic and easier to work with, unlocking a lot of functionality provided
  by the standard library and ecosystem crates.
- 在合理的情况下，为你的类型实现标准特征（`Debug`、`PartialEq` 等）。它会使你的类型更地道、更易于使用，解锁标准库和生态系统 crate 提供的许多功能。
- Implement traits from third-party crates if you need the functionality they unlock within their ecosystem.
- 如果你需要第三方 crate 在其生态系统中解锁的功能，请实现来自这些 crate 的特征。
- Beware of making code generic solely to use mocks in your tests. The maintainability cost of this approach
  can be high, and it's often better to use a different testing strategy. Check out the
  [testing masterclass](https://github.com/mainmatter/rust-advanced-testing-workshop)
  for details on high-fidelity testing.
- 警惕仅仅为了在测试中使用模拟而将代码泛型化。这种方法的维护成本可能很高，通常最好使用不同的测试策略。查看[测试大师班](https://github.com/mainmatter/rust-advanced-testing-workshop)了解高保真测试的细节。

## Testing your knowledge
## 测试你的知识

Before moving on, let's go through one last exercise to consolidate what we've learned.
You'll have minimal guidance this time—just the exercise description and the tests to guide you.
在继续之前，让我们通过最后一个练习来巩固我们所学的知识。这次你只会得到最少的指导——只有练习描述和测试来引导你。
