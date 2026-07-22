// TODO: Implement the `fixed_reply` function. It should accept two `TcpListener` instances,
// TODO: 实现 `fixed_reply` 函数。它应接受两个 `TcpListener` 实例，
//  accept connections on both of them concurrently, and always reply to clients by sending
//  同时接受两者的连接，并通过发送 `reply` 参数的 `Display` 表示形式
//  the `Display` representation of the `reply` argument as a response.
//  始终回复客户端。
use std::fmt::Display;
use std::future::pending;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

async fn handle_connection<T: Display>(mut socket: TcpStream, reply: Arc<T>) {
    let _ = socket.write_all(reply.to_string().as_bytes()).await;
    let _ = socket.shutdown().await;
}

async fn accept_loop<T>(listener: TcpListener, reply: Arc<T>)
where
    T: Display + Send + Sync + 'static,
{
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let reply = reply.clone();
        tokio::spawn(handle_connection(socket, reply));
    }
}

pub async fn fixed_reply<T>(first: TcpListener, second: TcpListener, reply: T)
where
    // `T` cannot be cloned. How do you share it between the two server tasks?
    // `T` 不能被克隆。如何在两个服务器任务之间共享它？
    T: Display + Send + Sync + 'static,
{
    let reply = Arc::new(reply);
    tokio::spawn(accept_loop(first, reply.clone()));
    tokio::spawn(accept_loop(second, reply));
    pending::<()>().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::AsyncReadExt;
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        let reply = "Yo";
        tokio::spawn(fixed_reply(first_listener, second_listener, reply));

        let mut join_set = JoinSet::new();

        for _ in 0..3 {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, _) = socket.split();

                    // Read the response
                    // 读取响应
                    let mut buf = Vec::new();
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, reply.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
