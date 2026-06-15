/// TODO: the code below will deadlock because it's using std's channels,
/// TODO: 下面的代码会因为使用 std 的通道而死锁，
///  which are not async-aware.
///  std 通道不支持异步。
///  Rewrite it to use `tokio`'s channels primitive (you'll have to touch
///  重写它以使用 `tokio` 的通道原语（你也需要修改测试代码）。
///  the testing code too, yes).
///
/// Can you understand the sequence of events that can lead to a deadlock?
/// 你能理解导致死锁的事件序列吗？
use std::sync::mpsc;

pub struct Message {
    payload: String,
    response_channel: mpsc::Sender<Message>,
}

/// Replies with `pong` to any message it receives, setting up a new
/// 用 `pong` 回复收到的任何消息，设置一个新的通道以继续与调用者通信。
/// channel to continue communicating with the caller.
pub async fn pong(mut receiver: mpsc::Receiver<Message>) {
    loop {
        if let Ok(msg) = receiver.recv() {
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = mpsc::channel();
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                .unwrap();
            receiver = new_receiver;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Message, pong};
    use std::sync::mpsc;

    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = mpsc::channel();
        let (response_sender, response_receiver) = mpsc::channel();
        sender
            .send(Message {
                payload: "pong".into(),
                response_channel: response_sender,
            })
            .unwrap();

        tokio::spawn(pong(receiver));

        let answer = response_receiver.recv().unwrap().payload;
        assert_eq!(answer, "pong");
    }
}
