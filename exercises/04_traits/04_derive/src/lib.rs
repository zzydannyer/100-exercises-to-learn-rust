// TODO: A (derivable) trait implementation is missing for this exercise to compile successfully.
// TODO: 此练习缺少一个（可派生）trait 实现才能成功编译。
//   Fix it!
//   修复它！
//
// # `Debug` primer
// # `Debug` 入门
//
// `Debug` returns a representation of a Rust type that's suitable for debugging (hence the name).
// `Debug` 返回适合调试的 Rust 类型表示形式（因此得名）。
// `assert_eq!` requires `Ticket` to implement `Debug` because, when the assertion fails, it tries to
// `assert_eq!` 要求 `Ticket` 实现 `Debug`，因为当断言失败时，它会尝试将比较的双方输出到终端。
// print both sides of the comparison to the terminal.
// `assert_eq!` 要求 `Ticket` 实现 `Debug`，因为当断言失败时，它会尝试将比较的双方输出到终端。
// If the compared type doesn't implement `Debug`, it doesn't know how to represent them!
// 如果比较的类型没有实现 `Debug`，它不知道如何表示它们！

#[derive(PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq() {
        let title = "title";
        let description = "description";
        let status = "To-Do";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        assert_eq!(ticket1, ticket2);
    }

    #[test]
    fn test_description_not_matching() {
        let title = "title";
        let status = "To-Do";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: "description".to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: "description2".to_string(),
            status: status.to_string(),
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_title_not_matching() {
        let status = "To-Do";
        let description = "description";
        let ticket1 = Ticket {
            title: "title".to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: "title2".to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_status_not_matching() {
        let title = "title";
        let description = "description";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: "status".to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: "status2".to_string(),
        };
        assert_ne!(ticket1, ticket2);
    }
}
