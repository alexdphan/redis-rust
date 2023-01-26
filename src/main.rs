// Result is a type alias for Result<T, E> where E is always anyhow::Error
use anyhow::Result;
// BytesMut is a type from the bytes crate that provides a mutable buffer of bytes
use bytes::BytesMut;
// AsyncReadExt and AsyncWriteExt are traits that provide read_buf() and write() methods
use tokio::io::{AsyncReadExt, AsyncWriteExt};
// TcpListener and TcpStream are types from the tokio crate that provide methods for listening for and accepting connections, and reading and writing data
use tokio::net::{TcpListener, TcpStream};
// provides RespConnection, a type that wraps a TcpStream and provides methods for reading and writing RESP values
use resp::Value::{Error, SimpleString};
// mod resp is a module that provides a type for reading and writing RESP values
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
    // resp provides a simple RespConnection, which wraps a TcpStream and provides methods for reading and writing RESP values
    let mut conn = resp::RespConnection::new(stream);

    // we assume the incoming message will be represented as a RESP array (Value::Array) and use a utility function to get the first element of the array (head element), the command, the tail of the array (tail elements), and the arguments
    // check command to see if it was a PING or an ECHO. If it was a PING, we respond with a simple string, "PONG". If it was an ECHO, we respond with the first argument
    // if the command was not a PING or an ECHO (otherwise), we respond with an error
    loop {
        let value = conn.read_value().await?;

        // if there is a value, we unwrap it and call to_command() to get the command and arguments
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
    }

    Ok(())
}
