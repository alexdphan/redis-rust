// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::{io::Read, io::Write};

#[tokio::main]
fn main() {
   async fn main() -> Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let incoming = listener.accept().await;
        match incoming {
            Ok((mut stream, _)) => {
                println!("accepted new connection");
                tokio::spawn(async move {
                    handle_connection(&mut stream).await.unwrap();
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buf = BytesMut::with_capacity(512);

    loop {
        // Wait for the client to send us a message but ignore the content for now
        let bytes_read = stream.read_buf(&mut buf).await?;
        if bytes_read == 0 {
            println!("client closed the connection");
            break;
        }

        stream.write("+PONG\r\n".as_bytes()).await?;
    }

    Ok(())
}