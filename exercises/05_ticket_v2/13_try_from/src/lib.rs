// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
// TODO: 为 `Status` 实现 `TryFrom<String>` 和 `TryFrom<&str>`。
//  The parsing should be case-insensitive.
//  解析应不区分大小写。

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        match value.as_str() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err("Invalid status".to_string()),
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        match value.as_str() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err("Invalid status".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);

        let empty = String::new();
        assert!(Status::try_from(empty).is_err());

        let invalid = "InValid".to_string();
        assert!(Status::try_from(invalid).is_err());
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);

        assert!(Status::try_from("").is_err());
        assert!(Status::try_from("invalid").is_err());
    }
}
