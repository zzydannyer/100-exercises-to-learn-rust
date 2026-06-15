// TODO: Fill in the missing methods for `TicketStore`.
// TODO: 填写 `TicketStore` 缺失的方法。
//  Notice how we no longer need a separate update command: `Get` now returns a handle to the ticket
//  注意，我们不再需要单独的更新命令：`Get` 现在返回一个票据的句柄，
//  which allows the caller to both modify and read the ticket.
//  允许调用者既可以修改也可以读取票据。
use std::sync::mpsc::{Receiver, SyncSender, TrySendError, sync_channel};
use std::sync::{Arc, Mutex};

use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, OverloadedError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::Insert {
                draft,
                response_channel: response_sender,
            })
            .map_err(|_| OverloadedError)?;
        Ok(response_receiver.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Arc<Mutex<Ticket>>>, OverloadedError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::Get {
                id,
                response_channel: response_sender,
            })
            .map_err(|_| OverloadedError)?;
        Ok(response_receiver.recv().unwrap())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("The store is overloaded")]
pub struct OverloadedError;

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Arc<Mutex<Ticket>>>>,
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
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket);
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
