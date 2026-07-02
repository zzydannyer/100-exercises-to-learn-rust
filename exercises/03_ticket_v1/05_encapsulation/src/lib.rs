pub mod ticket {
    pub struct Ticket {
        pub title: String,
        pub description: String,
        pub status: String,
    }

    impl Ticket {
        pub fn new(title: String, description: String, status: String) -> Ticket {
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
            if status != "To-Do" && status != "In Progress" && status != "Done" {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
            }

            Ticket {
                title,
                description,
                status,
            }
        }

        // TODO: Add three public methods to the `Ticket` struct:
        // TODO: 为 `Ticket` 结构体添加三个公有方法：
        //  - `title` that returns the `title` field.
        //  - `title` 返回 `title` 字段。
        pub fn title(&self) -> &str {
            &self.title
        }
        //  - `description` that returns the `description` field.
        //  - `description` 返回 `description` 字段。
        pub fn description(&self) -> &str {
            &self.description
        }
        //  - `status` that returns the `status` field.
        //  - `status` 返回 `status` 字段。
        pub fn status(&self) -> &str {
            &self.status
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ticket::Ticket;

    #[test]
    fn description() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        assert_eq!(ticket.description(), "A description");
    }

    #[test]
    fn title() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        assert_eq!(ticket.title(), "A title");
    }

    #[test]
    fn status() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        assert_eq!(ticket.status(), "To-Do");
    }
}
