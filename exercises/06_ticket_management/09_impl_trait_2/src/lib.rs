// TODO: Rework the signature of `TicketStore::add_ticket` to use a generic type parameter rather
// TODO: 重新设计 `TicketStore::add_ticket` 的签名，使其使用泛型类型参数而不是 `impl Trait` 语法。
//  than `impl Trait` syntax.

use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    // Using `Into<Ticket>` as the type parameter for `ticket` allows the method to accept any type
    // 使用 `Into<Ticket>` 作为 `ticket` 的类型参数，允许该方法接受任何可以不可失败地转换为 `Ticket` 的类型。
    // that can be infallibly converted into a `Ticket`.
    // This can make it nicer to use the method, as it removes the syntax noise of `.into()`
    // 这可以使方法使用起来更简洁，因为它消除了调用处 `.into()` 的语法噪声。
    // from the calling site. It can worsen the quality of the compiler error messages, though.
    // 不过，它可能会降低编译器错误信息的质量。
    pub fn add_ticket(&mut self, ticket: impl Into<Ticket>) {
        self.tickets.push(ticket.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    struct TicketDraft {
        pub title: TicketTitle,
        pub description: TicketDescription,
    }

    impl From<TicketDraft> for Ticket {
        fn from(draft: TicketDraft) -> Self {
            Self {
                title: draft.title,
                description: draft.description,
                status: Status::ToDo,
            }
        }
    }

    #[test]
    fn generic_add() {
        let mut store = TicketStore::new();
        // This won't compile if `add_ticket` uses `impl Trait` syntax in argument position.
        // 如果 `add_ticket` 在参数位置使用 `impl Trait` 语法，这将无法编译。
        store.add_ticket::<TicketDraft>(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        });
    }
}
