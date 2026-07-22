// Not much to be exercised on `Sync`, just a thing to remember.
// 关于 `Sync` 没有太多要练习的，只是一件需要记住的事情。
fn outro() -> &'static str {
    "I have a good understanding of Send and Sync!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a good understanding of Send and Sync!");
    }
}
