# Two-way communication
# 双向通信

In our current client-server implementation, communication flows in one direction: from the client to the server.\
在我们当前的客户端-服务器实现中，通信是单向流动的：从客户端到服务器。\
The client has no way of knowing if the server received the message, executed it successfully, or failed.
客户端无法知道服务器是否收到了消息、是否成功执行或失败。
That's not ideal.
这并不理想。

To solve this issue, we can introduce a two-way communication system.
为了解决这个问题，我们可以引入一个双向通信系统。

## Response channel
## 响应通道

We need a way for the server to send a response back to the client.\
我们需要一种方式让服务器向客户端发送响应。\
There are various ways to do this, but the simplest option is to include a `Sender` channel in
the message that the client sends to the server. After processing the message, the server can use
this channel to send a response back to the client.
有多种方法可以实现这一点，但最简单的选项是在客户端发送给服务器的消息中包含一个 `Sender` 通道。处理完消息后，服务器可以使用这个通道向客户端发送响应。

This is a fairly common pattern in Rust applications built on top of message-passing primitives.
这是基于消息传递原语构建的 Rust 应用程序中相当常见的模式。
