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
                let mut buffer = [0];
                stream.read(&mut buffer).unwrap();
                stream.write(buffer.as_ref()).unwrap();
                println!("{}", String::from_utf8_lossy(&buffer));
                // convert the buffer to a string

            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
