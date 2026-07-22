use tokio::net::TcpListener;

// TODO: write an echo server that accepts incoming TCP connections and
// TODO: 编写一个回显服务器，接受传入的 TCP 连接，并将
//  echoes the received data back to the client.
//  接收到的数据回显给客户端。
//  `echo` should not return when it finishes processing a connection, but should
//  `echo` 在处理完一个连接后不应返回，而应
//  continue to accept new connections.
//  继续接受新的连接。
//
// Hint: you should rely on `tokio`'s structs and methods to implement the echo server.
// 提示：你应该依赖 `tokio` 的结构和方法来实现回显服务器。
// In particular:
// 具体来说：
// - `tokio::net::TcpListener::accept` to process the next incoming connection
// - `tokio::net::TcpListener::accept` 处理下一个传入连接
// - `tokio::net::TcpStream::split` to obtain a reader and a writer from the socket
// - `tokio::net::TcpStream::split` 从套接字获取读取器和写入器
// - `tokio::io::copy` to copy data from the reader to the writer
// - `tokio::io::copy` 将数据从读取器复制到写入器
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        let (mut reader, mut writer) = socket.split();
        tokio::io::copy(&mut reader, &mut writer).await?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_echo() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(echo(listener));

        let requests = vec!["hello", "world", "foo", "bar"];

        for request in requests {
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
        }
    }
}
