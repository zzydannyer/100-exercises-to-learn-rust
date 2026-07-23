use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::model::{CreateTicket, PatchTicket, Status, Ticket};
use crate::state::AppState;

pub async fn create_ticket(
    State(state): State<AppState>,
    Json(body): Json<CreateTicket>,
) -> (StatusCode, Json<Ticket>) {
    let mut id_guard = state.next_id.lock().unwrap();
    let id = *id_guard;
    *id_guard += 1;
    drop(id_guard);

    let ticket = Ticket {
        id,
        title: body.title,
        description: body.description,
        status: Status::ToDo,
    };

    state.tickets.lock().unwrap().insert(id, ticket.clone());
    (StatusCode::CREATED, Json(ticket))
}

pub async fn get_ticket(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>, StatusCode> {
    state
        .tickets
        .lock()
        .unwrap()
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn patch_ticket(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(body): Json<PatchTicket>,
) -> Result<Json<Ticket>, StatusCode> {
    let mut tickets = state.tickets.lock().unwrap();
    let ticket = tickets.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;
    if let Some(title) = body.title {
        ticket.title = title;
    }
    if let Some(description) = body.description {
        ticket.description = description;
    }
    if let Some(status) = body.status {
        ticket.status = status;
    }
    Ok(Json(ticket.clone()))
}

pub async fn list_tickets(State(state): State<AppState>) -> Json<Vec<Ticket>> {
    let tickets = state.tickets.lock().unwrap();
    let mut list: Vec<Ticket> = tickets.values().cloned().collect();
    list.sort_by_key(|ticket| ticket.id);
    Json(list)
}

pub async fn delete_ticket(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> StatusCode {
    if state.tickets.lock().unwrap().remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
