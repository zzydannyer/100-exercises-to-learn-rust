// TODO: Use two variants, one for a title error and one for a description error.
// TODO: 使用两个变体，一个用于标题错误，一个用于描述错误。
//   Each variant should contain a string with the explanation of what went wrong exactly.
//   每个变体应包含一个字符串，精确说明出了什么问题。
//   You'll have to update the implementation of `Ticket::new` as well.
//   你还必须更新 `Ticket::new` 的实现。
#[derive(Debug)]
enum TicketNewError {
    TitleError(String),
    DescriptionError(String),
}

// TODO: `easy_ticket` should panic when the title is invalid, using the error message
// TODO: `easy_ticket` 在标题无效时应使用 `TicketNewError` 枚举中相关变体存储的错误信息触发 panic
//   stored inside the relevant variant of the `TicketNewError` enum.
//   使用 `TicketNewError` 枚举中相关变体存储的错误信息触发 panic。
//   When the description is invalid, instead, it should use a default description:
//   当描述无效时，它应使用默认描述：
//   "Description not provided".
//   "Description not provided"。
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(), description, status.clone()) {
        Ok(ticket) => ticket,
        Err(TicketNewError::TitleError(msg)) => panic!("{}", msg),
        Err(TicketNewError::DescriptionError(_)) => Ticket {
            title,
            description: "Description not provided".to_string(),
            status,
        },
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleError(
                "Title cannot be empty".to_string(),
            ));
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleError(
                "Title cannot be longer than 50 bytes".to_string(),
            ));
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionError(
                "Description cannot be empty".to_string(),
            ));
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionError(
                "Description cannot be longer than 500 bytes".to_string(),
            ));
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
}
