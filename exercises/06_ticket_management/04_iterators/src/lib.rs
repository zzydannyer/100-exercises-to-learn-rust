use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Let's start sketching our ticket store!
// TODO: 让我们开始勾勒我们的工单存储！
//  First task: implement `IntoIterator` on `TicketStore` to allow iterating over all the tickets
//  第一个任务：在 `TicketStore` 上实现 `IntoIterator`，以允许使用 `for` 循环遍历其中包含的所有工单。
//  it contains using a `for` loop.
//
// Hint: you shouldn't have to implement the `Iterator` trait in this case.
// 提示：在这种情况下，你不必实现 `Iterator` trait。
//   You want to *delegate* the iteration to the `Vec<Ticket>` field in `TicketStore`.
//   你应该将迭代*委托*给 `TicketStore` 中的 `Vec<Ticket>` 字段。
//   Look at the standard library documentation for `Vec` to find the right type
//   查看标准库中 `Vec` 的文档，以找到
//   to return from `into_iter`.
//   从 `into_iter` 返回的正确类型。
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

impl IntoIterator for TicketStore {
    type Item = Ticket;
    type IntoIter = std::vec::IntoIter<Ticket>;

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.into_iter()
    }
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

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<_> = store.clone().into_iter().collect();
        assert_eq!(tickets, store.tickets);
    }
}
