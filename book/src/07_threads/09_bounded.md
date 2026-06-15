# Bounded vs unbounded channels
# 有界通道 vs 无界通道

So far we've been using unbounded channels.\
到目前为止我们一直在使用无界通道。\
You can send as many messages as you want, and the channel will grow to accommodate them.\
你可以发送任意数量的消息，通道会增长以适应它们。\
In a multi-producer single-consumer scenario, this can be problematic: if the producers
enqueue messages at a faster rate than the consumer can process them, the channel will
keep growing, potentially consuming all available memory.
在多生产者单消费者场景中，这可能会成为问题：如果生产者入队消息的速度快于消费者处理它们的速度，通道会不断增长，可能消耗所有可用内存。

Our recommendation is to **never** use an unbounded channel in a production system.\
我们的建议是**永远不要**在生产系统中使用无界通道。\
You should always enforce an upper limit on the number of messages that can be enqueued using a
**bounded channel**.
你应该始终使用**有界通道**来强制执行可入队消息数量的上限。

## Bounded channels
## 有界通道

A bounded channel has a fixed capacity.\
有界通道具有固定的容量。\
You can create one by calling `sync_channel` with a capacity greater than zero:
你可以通过调用 `sync_channel` 并传入大于零的容量来创建一个：

```rust
use std::sync::mpsc::sync_channel;

let (sender, receiver) = sync_channel(10);
```

`receiver` has the same type as before, `Receiver<T>`.\
`receiver` 与之前具有相同的类型，`Receiver<T>`。\
`sender`, instead, is an instance of `SyncSender<T>`.
而 `sender` 则是 `SyncSender<T>` 的一个实例。

### Sending messages
### 发送消息

You have two different methods to send messages through a `SyncSender`:
你有两种不同的方法通过 `SyncSender` 发送消息：

- `send`: if there is space in the channel, it will enqueue the message and return `Ok(())`.\
  If the channel is full, it will block and wait until there is space available.
- `send`：如果通道中有空间，它会将消息入队并返回 `Ok(())`。如果通道已满，它会阻塞并等待直到有空间可用。
- `try_send`: if there is space in the channel, it will enqueue the message and return `Ok(())`.\
  If the channel is full, it will return `Err(TrySendError::Full(value))`, where `value` is the message that couldn't be sent.
- `try_send`：如果通道中有空间，它会将消息入队并返回 `Ok(())`。如果通道已满，它会返回 `Err(TrySendError::Full(value))`，其中 `value` 是无法发送的消息。

Depending on your use case, you might want to use one or the other.
根据你的使用场景，你可能希望使用其中之一。

### Backpressure
### 背压

The main advantage of using bounded channels is that they provide a form of **backpressure**.\
使用有界通道的主要优势是它们提供了一种**背压**机制。\
They force the producers to slow down if the consumer can't keep up.
如果消费者跟不上，它们会强制生产者放慢速度。
The backpressure can then propagate through the system, potentially affecting the whole architecture and
preventing end users from overwhelming the system with requests.
背压然后可以在系统中传播，可能影响整个架构，并防止最终用户用请求压垮系统。
