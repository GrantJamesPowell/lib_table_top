use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (mut socket, info) = listener.accept().await.unwrap();
        println!("Info {:?}", info);
        println!("Socket {:?}", socket);
        socket.write_all(b"hello world!\n").await;
    }
}
