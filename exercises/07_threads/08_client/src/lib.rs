use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
// TODO: flesh out the client implementation.
// TODO: 完善客户端实现。
pub struct TicketStoreClient {
    sender: Sender<Command>,
}

impl TicketStoreClient {
    // Feel free to panic on all errors, for simplicity.
    // 为简单起见，可以在所有错误上 panic。
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        let (response_channel, receiver) = std::sync::mpsc::channel();
        self.sender
            .send(Command::Insert {
                draft,
                response_channel,
            })
            .unwrap();
        receiver.recv().unwrap()
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        let (response_channel, receiver) = std::sync::mpsc::channel();
        self.sender
            .send(Command::Get {
                id,
                response_channel,
            })
            .unwrap();
        receiver.recv().unwrap()
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

// No longer public! This becomes an internal detail of the library now.
// 不再是公共的！这现在成为库的内部细节。
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
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
