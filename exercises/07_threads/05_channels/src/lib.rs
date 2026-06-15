use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

pub enum Command {
    Insert(todo!()),
}

// Start the system by spawning the server thread.
// 通过生成服务器线程来启动系统。
// It returns a `Sender` instance which can then be used
// 它返回一个 `Sender` 实例，可用于
// by one or more clients to interact with the server.
// 让一个或多个客户端与服务器进行交互。
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: The server task should **never** stop.
// TODO: 服务器任务**永远**不应该停止。
//  Enter a loop: wait for a command to show up in
//  进入一个循环：等待通道中出现命令，
//  the channel, then execute it, then start waiting
//  然后执行它，然后开始等待下一条命令。
//  for the next command.
pub fn server(receiver: Receiver<Command>) {}
