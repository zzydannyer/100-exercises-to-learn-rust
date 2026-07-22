use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::model::Ticket;

#[derive(Clone, Default)]
pub struct AppState {
    pub tickets: Arc<Mutex<HashMap<u64, Ticket>>>,
    pub next_id: Arc<Mutex<u64>>,
}