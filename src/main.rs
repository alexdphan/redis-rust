// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::{io::Read, io::Write};


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New client connected");
                let mut buffer = [0; 1024];

                loop {
                    let bytes_read = stream.read(&mut buffer).unwrap();
                    if bytes_read == 0 {
                        break;
                    }
                    println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
                }
                // // Wait for the client to send us a message but ignore the content for now
                // stream.read(&mut buffer).unwrap();
                // stream.write("+PONG\r\n".as_bytes()).unwrap();

            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
