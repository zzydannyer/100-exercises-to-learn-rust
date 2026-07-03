// We need some more machinery to write a proper exercise for destructors.
// 我们需要更多机制来为析构函数编写一个合适的练习。
// We'll pick the concept up again in a later chapter after covering traits and
// 我们将在涵盖 trait 和内部可变性之后，在后面的章节中再次讨论这个概念。
// interior mutability.
// 我们将在涵盖 trait 和内部可变性之后，在后面的章节中再次讨论这个概念。
fn outro() -> &'static str {
    "I have a basic understanding of destructors!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a basic understanding of destructors!");
    }
}
