struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    // TODO: implement the `new` function.
    // TODO: 实现 `new` 函数。
    //  The following requirements should be met:
    //  应满足以下要求：
    //   - Only `To-Do`, `In Progress`, and `Done` statuses are allowed.
    //   - 只允许 `To-Do`、`In Progress` 和 `Done` 这三种状态。
    //   - The `title` and `description` fields should not be empty.
    //   - `title` 和 `description` 字段不应为空。
    //   - the `title` should be at most 50 bytes long.
    //   - `title` 的长度最多为 50 字节。
    //   - the `description` should be at most 500 bytes long.
    //   - `description` 的长度最多为 500 字节。
    //  The method should panic if any of the requirements are not met.
    //  如果任何要求未满足，该方法应触发 panic。
    //  You can find the needed panic messages in the tests.
    //  你可以在测试中找到所需的 panic 消息。
    //
    // You'll have to use what you learned in the previous exercises,
    // 你需要运用在前面的练习中学到的知识，
    // as well as some `String` methods. Use the documentation of Rust's standard library
    // 以及一些 `String` 方法。使用 Rust 标准库的文档来找到最合适的选项 -> https://doc.rust-lang.org/std/string/struct.String.html
    // to find the most appropriate options -> https://doc.rust-lang.org/std/string/struct.String.html
    // 以及一些 `String` 方法。使用 Rust 标准库的文档来找到最合适的选项 -> https://doc.rust-lang.org/std/string/struct.String.html
    fn new(title: String, description: String, status: String) -> Self {
        if status != "To-Do" && status != "In Progress" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }

        if title.is_empty() {
            panic!("Title cannot be empty");
        }

        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }

        if description.is_empty() {
            panic!("Description cannot be empty");
        }

        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }

        Self {
            title,
            description,
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }
}
