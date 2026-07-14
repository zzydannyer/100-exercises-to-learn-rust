// TODO: Set `move_forward` to `true` in `ready` when you think you're done with this exercise.
// TODO: 当你认为完成了这个练习时，在 `ready` 中将 `move_forward` 设置为 `true`。
//  Feel free to call an instructor to verify your solution!
//  随时可以叫导师来验证你的解决方案！
use channels::data::TicketDraft;
use channels::{Command, launch};
use std::time::Duration;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn a_thread_is_spawned() {
    let sender = launch();
    std::thread::sleep(Duration::from_millis(200));

    sender
        .send(Command::Insert(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        }))
        // If the thread is no longer running, this will panic
        // 如果线程不再运行，这将引发 panic
        // because the channel will be closed.
        // 因为通道将被关闭。
        .expect("Did you actually spawn a thread? The channel is closed!");
}

#[test]
fn ready() {
    // There's very little that we can check automatically in this exercise,
    // 在这个练习中，我们能够自动检查的内容非常有限，
    // since our server doesn't expose any **read** actions.
    // 因为我们的服务器没有暴露任何**读取**操作。
    // We have no way to know if the inserts are actually happening and if they
    // 我们无法知道插入是否真的发生了，以及它们是否正确发生。
    // are happening correctly.
    let move_forward = true;

    assert!(move_forward);
}
