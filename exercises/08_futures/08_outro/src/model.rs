use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, Deserialize)]
pub struct CreateTicket {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct PatchTicket {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<Status>,
}
