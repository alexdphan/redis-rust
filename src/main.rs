// Uncomment this block to pass the first stage
use anyhow::Result;
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

// to let Tokio start a runtime before our main function does any work.
#[tokio::main]
// async function that returns a Result
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    // assigns listener to TcpListener::bind()
    // bind() returns a Result<TcpListener, io::Error>
    loop {
        let incoming = listener.accept().await;
        match incoming {
            Ok((stream, _)) => {
                println!("accepted new connection");

                tokio::spawn(async move {
                    handle_connection(stream).await.unwrap();
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

// TcpStream is a wrapper around a socket, used to read and write data (argument)
// async function that returns a Result
async fn handle_connection(stream: TcpStream) -> Result<()> {
    // create a buffer to store the data we read from the socket
    // BytesMut is a type from the bytes crate that provides a mutable buffer of bytes
    // represent a unique viet into a potentially shared memory region
    // owners of BytesMut handles can mutate the buffer without affecting other owners
    let mut buf = BytesMut::with_capacity(512);
    // It is similar to a Vec<u8> but with less copies and allocations.

    // read and write calls are await ed
    // These changes allow Tokio to suspend and resume our connection handler at the right times, and do work on tasks for other clients while ours is suspended.
    loop {
        // wait for client to send us a message but ignore the content for now
        let bytes_read = stream.read(&mut buf).await?;
        if bytes_read == 0 {
            println!("client closed the connection");
            break;
        }

        stream.write("+PONG\r\n".as_bytes()).await?;
    }
    Ok(())
}
