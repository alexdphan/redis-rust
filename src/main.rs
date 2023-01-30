use anyhow::Result;
// use bytes::BytesMut;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use resp::Value::{BulkString, Error, Null, SimpleString};
use std::sync::{Arc, Mutex};
use store::Store;
use tokio::net::{TcpListener, TcpStream};

mod resp;
mod store;

// to let Tokio start a runtime before our main function does any work.
// async function that returns a Result
#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    // assigns listener to TcpListener::bind()
    // bind() returns a Result<TcpListener, io::Error>

    // main_store is a shared reference to a mutex
    // mutex is a thread-safe way to share data between threads
    // Each mutex has a type parameter which represents the data that it is protecting
    let main_store = Arc::new(Mutex::new(Store::new()));
    // The type Arc<T> provides shared ownership of a value of type T, allocated in the heap, thread safety
    // https://doc.rust-lang.org/beta/rust-by-example/std/arc.html
    loop {
        // accept() returns a future that will resolve to a Result<TcpStream, io::Error>
        let incoming = listener.accept().await;
        // clone the main_store, so that each connection has its own store
        let client_store = main_store.clone();

        match incoming {
            Ok((stream, _)) => {
                println!("accepted new connection");

                tokio::spawn(async move {
                    handle_connection(stream, client_store).await.unwrap();
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
// Arc<Mutex<Store>> is a shared reference to a mutex, which is a thread-safe way to share data between threads
async fn handle_connection(stream: TcpStream, client_store: Arc<Mutex<Store>>) -> Result<()> {
    let mut conn = resp::RespConnection::new(stream);

    loop {
        let value = conn.read_value().await?;

        if let Some(value) = value {
            let (command, args) = value.to_command()?;
            let response = match command.to_ascii_lowercase().as_ref() {
                "ping" => SimpleString("PONG".to_string()),
                "echo" => args.first().unwrap().clone(),
                // get would return a value, so we need to wrap it in an Option before we can call unwrap()
                "get" => {
                    // if Some BulkString(key) is in args of index 0, then if let Some(value) is in client_store, you can get the key by using lock() and unwrap()
                    // if Some BulkString(key) is in args of index 0, then if let Some(value) is not in client_store, then return Null
                    // if Some BulkString(key) is not in args of index 0, then return Error
                    if let Some(BulkString(key)) = args.get(0) {
                        if let Some(value) = client_store.lock().unwrap().get(key.clone()) {
                            SimpleString(value)
                        } else {
                            Null
                        }
                    } else {
                        Error("Get requires one argument".to_string())
                    }
                }
                "set" => {
                    // if the first and second arguments are Some BulkString(key) and Some BulkString(value), then set the key and value in client_store to lock() and unwrap(), and return SimpleString("OK")
                    // if the first and second arguments are not Some BulkString(key) and Some BulkString(value), then return Error with "Set requires two arguments" or "command not implemented: {}"
                    if let (Some(BulkString(key)), Some(BulkString(value))) =
                        (args.get(0), args.get(1))
                    {
                        // if the third and fourth arguments are Some BulkString(_), and Some BulkString(amount), then set the key and value in client_store to lock() and unwrap(), and return SimpleString("OK")
                        if let (Some(BulkString(_)), Some(BulkString(amount))) =
                            (args.get(2), args.get(3))
                        {
                            // set_with_expiry() returns a future that will resolve to a Result<(), io::Error>, taking in key, value, and amount
                            // amount is the time in seconds that the key should be stored for, which is parsed to u64, then it will be set in client_store and return SimpleString("OK"). Once used, the key will be deleted
                            client_store.lock().unwrap().set_with_expiry(
                                key.clone(),
                                value.clone(),
                                amount.parse::<u64>()?,
                            );
                        } else {
                            // set() returns a future that will resolve to a Result<(), io::Error>, taking in key and value
                            client_store.lock().unwrap().set(key.clone(), value.clone());
                        }
                        SimpleString("OK".to_string())
                    } else {
                        Error("Set requires two arguments".to_string());
                        Error("Set requires two or four arguments".to_string())
                    }
                }
                _ => Error(format!("command not implemented: {}", command)),
            };
            // write_value() returns a future that will resolve to a Result<(), io::Error>
            // this would write the response to the client, and then wait for the next command
            conn.write_value(response).await?;
        } else {
            break;
        }
    }
    Ok(())
}
// overall,
