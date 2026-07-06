// TODO: use `Status` as type for `Ticket::status`
// TODO: 使用 `Status` 作为 `Ticket::status` 的类型
//   Adjust the signature and implementation of all other methods as necessary.
//   根据需要调整所有其他方法的签名和实现。

#[derive(Debug, PartialEq)]
// `derive`s are recursive: it can only derive `PartialEq` if all fields also implement `PartialEq`.
// `derive` 是递归的：只有在所有字段都实现了 `PartialEq` 时，它才能推导出 `PartialEq`。
// Same holds for `Debug`. Do what you must with `Status` to make this work.
// `Debug` 同理。请对 `Status` 做必要处理以使其正常工作。
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Status {
    // TODO: add the missing variants
    // TODO: 添加缺少的变体
    ToDo,
    InProgress,
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Ticket {
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
        if status != Status::ToDo && status != Status::InProgress && status != Status::Done {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }

        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{valid_description, valid_title};

    #[test]
    fn test_partial_eq() {
        let title = valid_title();
        let description = valid_description();
        let ticket1 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::ToDo,
        };
        let ticket2 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::ToDo,
        };
        assert_eq!(ticket1, ticket2);
    }

    #[test]
    fn test_description_not_matching() {
        let title = valid_title();
        let status = Status::ToDo;
        let ticket1 = Ticket {
            title: title.clone(),
            description: "description".to_string(),
            status,
        };
        let ticket2 = Ticket {
            title: title.clone(),
            description: "description2".to_string(),
            status,
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_title_not_matching() {
        let description = valid_description();
        let status = Status::InProgress;
        let ticket1 = Ticket {
            title: "title".to_string(),
            description: description.clone(),
            status,
        };
        let ticket2 = Ticket {
            title: "title2".to_string(),
            description: description.clone(),
            status,
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_status_not_matching() {
        let title = valid_title();
        let description = valid_description();
        let ticket1 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::InProgress,
        };
        let ticket2 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::Done,
        };
        assert_ne!(ticket1, ticket2);
    }
}
