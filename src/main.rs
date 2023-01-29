use anyhow::Result;
// use bytes::BytesMut;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::resp::Value::{Error, SimpleString};
use tokio::net::{TcpListener, TcpStream};

mod resp;

// to let Tokio start a runtime before our main function does any work.
// async function that returns a Result
#[tokio::main]
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
    let mut conn = resp::RespConnection::new(stream);

    loop {
        // Wait for the client to send us a message but ignore the content for now
        // let bytes_read = stream.read_buf(&mut buf).await?;
        // if bytes_read == 0 {
        //     println!("client closed the connection");
        let value = conn.read_value().await?;

        if let Some(value) = value {
            let (command, args) = value.to_command()?;
            let response = match command.to_ascii_lowercase().as_ref() {
                "ping" => SimpleString("PONG".to_string()),
                "echo" => args.first().unwrap().clone(),
                _ => Error(format!("command not implemented: {}", command)),
            };

            conn.write_value(response).await?;
        } else {
            break;
        }

        // stream.write("+PONG\r\n".as_bytes()).await?;
    }

    Ok(())
}
