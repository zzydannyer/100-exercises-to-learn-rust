// TODO: based on what we just learned about ownership, it sounds like immutable references
// TODO: 根据我们刚刚学到的关于所有权的知识，不可变引用似乎很适合我们的访问器方法。
//   are a good fit for our accessor methods.
// TODO: 根据我们刚刚学到的关于所有权的知识，不可变引用似乎很适合我们的访问器方法。
//   Change the existing implementation of `Ticket`'s accessor methods to take a reference
//   修改 `Ticket` 访问器方法的现有实现，使其接受对 `self` 的引用作为参数，而不是获取其所有权。
//   to `self` as an argument, rather than taking ownership of it.
//   修改 `Ticket` 访问器方法的现有实现，使其接受对 `self` 的引用作为参数，而不是获取其所有权。

pub struct Ticket {
    title: String,
    description: String,
    status: String,
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

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;

    #[test]
    fn works() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        // If you change the signatures as requested, this should compile:
        // 如果你按要求更改了签名，这应该可以编译：
        // we can call these methods one after the other because they borrow `self`
        // 我们可以依次调用这些方法，因为它们借用 `self` 而不是获取其所有权。
        // rather than taking ownership of it.
        // 我们可以依次调用这些方法，因为它们借用 `self` 而不是获取其所有权。
        assert_eq!(ticket.title(), "A title");
        assert_eq!(ticket.description(), "A description");
        assert_eq!(ticket.status(), "To-Do");
    }
}
