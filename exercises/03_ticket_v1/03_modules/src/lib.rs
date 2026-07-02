mod helpers {
    // TODO: Make this code compile, either by adding a `use` statement or by using
    // TODO: 使此代码通过编译，可以通过添加 `use` 语句或使用适当的路径来引用 `Ticket` 结构体。
    //  the appropriate path to refer to the `Ticket` struct.
    // TODO: 使此代码通过编译，可以通过添加 `use` 语句或使用适当的路径来引用 `Ticket` 结构体。
    use super::Ticket;

    fn create_todo_ticket(title: String, description: String) -> Ticket {
        Ticket::new(title, description, "To-Do".into())
    }
}

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    fn new(title: String, description: String, status: String) -> Ticket {
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
}
