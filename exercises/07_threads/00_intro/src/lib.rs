fn intro() -> &'static str {
    // TODO: fix me 👇
    // TODO: 修复我 👇
    "I'm ready to build a concurrent ticket management system!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(
            intro(),
            "I'm ready to build a concurrent ticket management system!"
        );
    }
}
