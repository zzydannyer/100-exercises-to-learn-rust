use crate::status::Status;

// We've seen how to declare modules in one of the earliest exercises, but
// 我们在之前的练习中已经看到了如何声明模块，但是
// we haven't seen how to extract them into separate files.
// 我们还没有看到如何将它们提取到单独的文件中。
// Let's fix that now!
// 现在让我们来解决这个问题！
//
// In the simplest case, when the extracted module is a single file, it is enough to
// 在最简单的情况下，当提取的模块是单个文件时，只需
// create a new file with the same name as the module and move the module content there.
// 创建一个与模块同名的文件，并将模块内容移动到那里即可。
// The module file should be placed in the same directory as the file that declares the module.
// 模块文件应放置在与声明该模块的文件相同的目录中。
// In this case, `src/lib.rs`, thus `status.rs` should be placed in the `src` directory.
// 在这种情况下，`src/lib.rs` 声明了模块，因此 `status.rs` 应放置在 `src` 目录中。
mod status;

// TODO: Add a new error variant to `TicketNewError` for when the status string is invalid.
// TODO: 为 `TicketNewError` 添加一个新的错误变体，用于处理状态字符串无效的情况。
//   When calling `source` on an error of that variant, it should return a `ParseStatusError` rather than `None`.
//   当对该变体的错误调用 `source` 时，它应返回 `ParseStatusError` 而不是 `None`。

#[derive(Debug, thiserror::Error)]
pub enum TicketNewError {
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        // TODO: Parse the status string into a `Status` enum.
        // TODO: 将状态字符串解析为 `Status` 枚举。

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use common::{valid_description, valid_title};
    use std::error::Error;

    use super::*;

    #[test]
    fn invalid_status() {
        let err = Ticket::new(valid_title(), valid_description(), "invalid".into()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "`invalid` is not a valid status. Use one of: ToDo, InProgress, Done"
        );
        assert!(err.source().is_some());
    }
}
