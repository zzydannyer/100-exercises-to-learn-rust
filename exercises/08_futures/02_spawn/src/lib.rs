use tokio::net::TcpListener;

// TODO: write an echo server that accepts TCP connections on two listeners, concurrently.
// TODO: 编写一个回显服务器，同时接受两个监听器上的 TCP 连接。
//  Multiple connections (on the same listeners) should be processed concurrently.
//  多个连接（在同一个监听器上）应并发处理。
//  The received data should be echoed back to the client.
//  接收到的数据应回显给客户端。

async fn echo_connection(mut socket: tokio::net::TcpStream) {
    let (mut reader, mut writer) = socket.split();
    let _ = tokio::io::copy(&mut reader, &mut writer).await;
}

async fn accept_loop(listener: TcpListener) {
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(echo_connection(socket));
    }
}

pub async fn echoes(first: TcpListener, second: TcpListener) -> Result<(), anyhow::Error> {
    tokio::spawn(accept_loop(first));
    tokio::spawn(accept_loop(second));
    std::future::pending::<Result<(), anyhow::Error>>().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
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
        tokio::spawn(echoes(first_listener, second_listener));

        let requests = vec!["hello", "world", "foo", "bar"];
        let mut join_set = JoinSet::new();

        for request in requests.clone() {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, mut writer) = socket.split();

                    // Send the request
                    // 发送请求
                    writer.write_all(request.as_bytes()).await.unwrap();
                    // Close the write side of the socket
                    // 关闭套接字的写入端
                    writer.shutdown().await.unwrap();

                    // Read the response
                    // 读取响应
                    let mut buf = Vec::with_capacity(request.len());
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, request.as_bytes());
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
