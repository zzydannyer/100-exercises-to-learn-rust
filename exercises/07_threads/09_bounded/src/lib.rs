// TODO: Convert the implementation to use bounded channels.
// TODO: 将实现转换为使用有界通道。
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: todo!(),
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, todo!()> {
        todo!()
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, todo!()> {
        todo!()
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    todo!();
    std::thread::spawn(move || server(receiver));
    todo!()
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: todo!(),
    },
    Get {
        id: TicketId,
        response_channel: todo!(),
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                todo!()
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                todo!()
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // 没有更多的发送者了，所以我们可以安全地中断
                // and shut down the server.
                // 并关闭服务器。
                break;
            }
        }
    }
}
