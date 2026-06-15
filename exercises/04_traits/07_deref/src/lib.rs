// TODO: whenever `title` and `description` are returned via their accessor methods, they
// TODO: 当通过访问器方法返回 `title` 和 `description` 时，它们应该被规范化——即去除首尾空白。
//   should be normalized—i.e. leading and trailing whitespace should be removed.
// TODO: 当通过访问器方法返回 `title` 和 `description` 时，它们应该被规范化——即去除首尾空白。
//   There is a method in Rust's standard library that can help with this, but you won't
//   Rust 标准库中有一个方法可以帮助实现这一点，但你不会在 `String` 的文档中找到它。
//   find it in the documentation for `String`.
//   Rust 标准库中有一个方法可以帮助实现这一点，但你不会在 `String` 的文档中找到它。
//   Can you figure out where it is defined and how to use it?
//   你能弄清楚它在哪里定义以及如何使用它吗？

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn title(&self) -> &str {
        todo!()
    }

    pub fn description(&self) -> &str {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let ticket = Ticket {
            title: "   A title ".to_string(),
            description: " A description   ".to_string(),
            status: "To-Do".to_string(),
        };

        assert_eq!("A title", ticket.title());
        assert_eq!("A description", ticket.description());
    }
}
