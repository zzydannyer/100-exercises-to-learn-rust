mod ticket {
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
    }
}

// TODO: **Exceptionally**, you'll be modifying both the `ticket` module and the `tests` module
// TODO: **特别地**，你将在此练习中同时修改 `ticket` 模块和 `tests` 模块。
//  in this exercise.
// TODO: **特别地**，你将在此练习中同时修改 `ticket` 模块和 `tests` 模块。
#[cfg(test)]
mod tests {
    // TODO: Add the necessary `pub` modifiers in the parent module to remove the compiler
    // TODO: 在父模块中添加必要的 `pub` 修饰符，以消除关于下面 use 语句的编译器错误。
    //  errors about the use statement below.
    // TODO: 在父模块中添加必要的 `pub` 修饰符，以消除关于下面 use 语句的编译器错误。
    use super::ticket::Ticket;

    // Be careful though! We don't want this function to compile after you have changed
    // 但是要小心！在你更改了可见性以使 use 语句编译通过后，我们不希望这个函数能够编译通过！
    // visibility to make the use statement compile!
    // 但是要小心！在你更改了可见性以使 use 语句编译通过后，我们不希望这个函数能够编译通过！
    // Once you have verified that it indeed doesn't compile, comment it out.
    // 一旦你验证了它确实无法编译，就将其注释掉。
    fn should_not_be_possible() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());

        // You should be seeing this error when trying to run this exercise:
        // 你在尝试运行此练习时应该会看到此错误：
        //
        // error[E0616]: field `description` of struct `Ticket` is private
        //
        //    |              assert_eq!(ticket.description, "A description");
        //    |                         ^^^^^^^^^^^^^^^^^^
        //
        // TODO: Once you have verified that the below does not compile,
        // TODO: 一旦你验证了下面的代码无法编译，就注释掉这一行，继续下一个练习！
        //   comment the line out to move on to the next exercise!
        // TODO: 一旦你验证了下面的代码无法编译，就注释掉这一行，继续下一个练习！
        assert_eq!(ticket.description, "A description");
    }

    fn encapsulation_cannot_be_violated() {
        // This should be impossible as well, with a similar error as the one encountered above.
        // 这同样应该是不可能的，会出现与上面遇到的类似错误。
        // (It will throw a compilation error only after you have commented the faulty line
        // （只有在你注释掉了前一个测试中的错误行之后，它才会抛出编译错误——下一个编译阶段！）
        // in the previous test - next compilation stage!)
        // （只有在你注释掉了前一个测试中的错误行之后，它才会抛出编译错误——下一个编译阶段！）
        //
        // This proves that `Ticket::new` is now the only way to get a `Ticket` instance.
        // 这证明了 `Ticket::new` 现在是获取 `Ticket` 实例的唯一方式。
        // It's impossible to create a ticket with an illegal title or description!
        // 创建一个具有非法标题或描述的工单是不可能的！
        //
        // TODO: Once you have verified that the below does not compile,
        // TODO: 一旦你验证了下面的代码无法编译，就注释掉这些行，继续下一个练习！
        //   comment the lines out to move on to the next exercise!
        // TODO: 一旦你验证了下面的代码无法编译，就注释掉这些行，继续下一个练习！
        let ticket = Ticket {
            title: "A title".into(),
            description: "A description".into(),
            status: "To-Do".into(),
        };
    }
}
