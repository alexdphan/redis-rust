// use anyhow::{Error, Result};
// use bytes::BytesMut;
// use tokio::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpStream};

// const CARRIAGE_RETURN: u8 = '\r' as u8;
// const NEWLINE: u8 = '\n' as u8;

// #[derive(Debug, PartialEq, Clone, Debug)]
// pub enum Value {
//     // for simple strings, the first byte of the RESP array is "+"
//     SimpleString(String),
//     // for errors, the first byte of the RESP array is "-"
//     Error(String),
//     // for bulk strings, the first byte of the RESP array is "$"
//     BulkString(String),
//     // for Arrays, the first byte of the RESP array is "*"
//     Arrau(Vec<Value>),
// }

// // impl Value is for defining the methods of the Value enum
// // the fucntion to_command() is for converting a Value::Array into a command and arguments
// impl Value {
//     // to_command is a function that returns a Result<(String, Vec<Value>)>
//     pub fn to_command(&self) -> Result<(String, Vec<Value>)> {
//         // match self is for matching the Value enum above
//         match self {
//             Value::Array(items) => {
//                 return Ok((
//                     // items.first() returns the first element of the array if it exists
//                     items.first().unwrap().unwrap_bulk(),
//                     // items.clone() returns a copy of the array
//                     items.clone().into_iter().skip(1).collect(),
//                 ));
//             }
//             // if the Value enum is not an Array, return an error
//             _ => Err(Error::msg("not an array")),
//         }
//     }

//     // function that returns a String
//     // Match self is for matching the Value enum above
//     fn unwrap_bulk(&self) -> String {
//         match self {
//             // if the Value enum is a BulkString, return a copy (clone) of the string, otherwise panic
//             Value::BulkString(s) => s.clone(),
//             _ => panic!("not a bulk string"),
//         }
//     }

//     // function to encode the Value enum into a string
//     pub fn encode(self) -> String {
//         match &self {
//             // if the Value enum is a SimpleString, format as a string with a "+" in front of it
//             Value::SimpleString(s) => format!("+{}\r\n", s.as_str()),
//             // if the Value enum is an Error, format as a string with a "-" in front of it
//             Value::Error(msg) => format!("-{}\r\n", msg.as_str()),
//             // if the Value enum is a BulkString, format as a string with a "$" in front of it
//             Value::BulkString(s) => format!("${}\r\n{}\r\n", s.chars().count(), s),
//             // the other use cases are not required for this project
//             _ => panic!("value encode not implemented for: {:?}", self),
//         }
//     }
// }

// // a struct for the RespConnection, which contains a TcpStream and a BytesMut
// pub struct RespConnection {
//     stream: TcpStream,
//     buffer: BytesMut,
// }

// // impl RespConnection is for defining the methods of the RespConnection struct
// impl RespConnection {
//     // function new is for creating a new RespConnection
//     pub fn new(stream: TcpStream) -> Self {
//         return RespConnection {
//             // the stream is the TcpStream that is passed into the function
//             stream,
//             // the buffer is a BytesMut with a capacity of 512 because the RESP protocol is limited to 512 MB
//             buffer: BytesMut::with_capacity(512),
//         };
//     }

//     // async function read_value is for reading the value from the stream
//     pub async fn read_value(&mut self) -> Result<Option<Value>> {
//         // loop is for looping through the stream, reading the value
//         loop {
//             let bytes_read = self.stream.read_buf(&mut self.buffer).await?;

//             // connection closes if bytes_read is 0
//             if bytes_read == 0 {
//                 return Ok(None);
//             }

//             // if the buffer is not empty, parse the message
//             if let Some((value, _)) = parse_message(self.buffer.split())? {
//                 return Ok(Some(value));
//             }
//         }
//     }

//     // async function write_value is for writing the value to the stream, and returning a Result
//     pub async fn write_value(&mut self, value: Value) -> Result<()> {
//         // uses self.stream.write_all to write the value to the stream
//         self.stream.write_all(value.encode().as_bytes()).await?;
//         Ok(())
//     }
// }

// // function parse_message is for parsing the message from the buffer and returning a Result of an Option of a Value and a usize
// fn parse_message(buffer: BytesMut) -> Result<Option<(Value, usize)>> {
//     // match buffer[0] is for matching the first byte of the buffer
//     match buffer[0] as char {
//         // if the first byte is a "+", parse the simple string with the buffer
//         '+' => decode_simple_string(buffer),
//         // if the first byte is a "$", parse the error with the buffer
//         '$' => decode_bulk_string(buffer),
//         // if the first byte is a "*", parse the array with the buffer
//         '*' => decode_array(buffer),
//         // otherwise, return an error
//         _ => Err(Error::msg("unrecognised message type")),
//     }
// }
// // "Parse" typically refers to the process of taking a string or byte sequence and converting it into a more structured format, such as a JSON object, an XML document, or a program written in a specific programming language. For example, a JSON parser would take a string of JSON text as input and convert it into a Rust data structure that can be easily manipulated in code.

// // "Decode" generally refers to the process of taking an encoded representation of data and converting it back into its original format. For example, a base64 decoder would take a base64 encoded string and convert it back into the original bytes.

// // function decode_simple_string is for decoding the simple string from the buffer and returning a Result of an Option of a Value and a usize
// fn decode_simple_string(buffer: BytesMut) -> Result<Option<(Value, usize)>> {
//     // if let Some((line, len)) is for checking if the buffer contains a line and a length
//     // read_until_crlf takes the buffer and returns a Result of an Option (the Option is Some if the buffer contains a line and a length, and None if it does not)
//     if let Some((line, len)) = read_until_crlf(&buffer[1..]) {
//         // if the buffer contains a line and a length, parse the string from the line
//         let str = parse_string(line)?;

//         // return the Value::SimpleString(str) and the length of the string
//         // len + 1 is for the first byte of the buffer (from the match statement called in parse_message function)
//         // len means the length of the string, not the length of the array
//         Ok(Some((Value::SimpleString(str), len + 1)))
//     } else {
//         Ok(None)
//     }
// }

// // function decode_array is for decoding the array from the buffer and returning a Result of an Option of a Value and a usize
// fn decode_array(buffer: BytesMut) -> Result<Option<(Value, usize)>> {    
//     let (array_length, mut bytes_consumed) =
//         if let Some((line, len)) = read_until_crlf(&buffer[1..]) {
//             let array_length = parse_integer(line)?;

//             (array_length, len + 1)
//         } else {
//             return Ok(None);
//         };

//     let mut items: Vec<Value> = Vec::new();
//     for _ in 0..array_length {
//         if let Some((v, len)) = parse_message(BytesMut::from(&buffer[bytes_consumed..]))? {
//             items.push(v);
//             bytes_consumed += len
//         } else {
//             return Ok(None);
//         }
//     }

//     return Ok(Some((Value::Array(items), bytes_consumed)));
// }


// // function decode_bulk_string is for decoding the bulk string from the buffer and returning a Result of an Option of a Value and a usize
// fn decode_bulk_string(buffer: BytesMut) -> Result<Option<(Value, usize)>> {
//     // assigning the bulk_length and bytes_consumed to the result of the if let Some((line, len)) statement
//     let (bulk_length, bytes_consumed) = if let Some((line, len)) = read_until_crlf(&buffer[1..]) {
//         let bulk_length = parse_integer(line)?;

//         (bulk_length, len + 1)
//     } else {
//         return Ok(None);
//     };

//     let end_of_bulk = bytes_consumed + (bulk_length as usize);
//     let end_of_bulk_line = end_of_bulk + 2;

//     return if end_of_bulk_line <= buffer.len() {
//         Ok(Some((
//             Value::BulkString(buffer.slice(bytes_consumed..end_of_bulk)?),
//             end_of_bulk_line,
//         )))
//     } else {
//         Ok(None)
//     };
// }
//     // function read_until_crlf is for reading until the carriage return line feed and returning an Option of a slice of the buffer and a usize
//     // the carriage return line feed is the end of the line
//     fn read_until_crlf(buffer: &[u8]) -> Option<(&[u8], usize)> {
//         // this for loop is for iterating through the buffer with the index i, starting at 1 (i comes from the match statement called in parse_message function)
//         for i in 1..buffer.len() {
//             // if the byte is a carriage return and the next byte is a line feed, return the buffer and the length of the buffer
//             if buffer[i - 1] == CARRIAGE_RETURN && buffer[i] == NEWLINE {
//                 return Some((&buffer[0..(i - 1)], i + 1));
//             }
//         }
//         // else return None
//         return None;
//     }

//     // function parse_string is for parsing the string from the bytes and returning a Result of a String
//     fn parse_string(bytes: &u8) -> Result<String> {
//         String::from_utf8(bytes.to_vec()).map_err(|_| Error::msg("invalid utf8"))
//     }

//     // function parse_integer is for parsing the integer from the bytes and returning a Result of an i64
//     fn parse_integer(bytes: &u8) -> Result<i64> {
//         let str_integer = parse_string(bytes)?;
//         (str_integer.parse::<i64>()).map_err(|_| Error::msg("Could not parse integer"))
//     }

use anyhow::{Error, Result};
use bytes::BytesMut;
use tokio::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpStream};

const CARRIAGE_RETURN: u8 = '\r' as u8;
const NEWLINE: u8 = '\n' as u8;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Value {
    SimpleString(String),
    Error(String),
    BulkString(String),
    Array(Vec<Value>),
}

impl Value {
    pub fn to_command(&self) -> Result<(String, Vec<Value>)> {
        match self {
            Value::Array(items) => {
                return Ok((
                    items.first().unwrap().unwrap_bulk(),
                    items.clone().into_iter().skip(1).collect(),
                ));
            }
            _ => Err(Error::msg("not an array")),
        }
    }

    fn unwrap_bulk(&self) -> String {
        match self {
            Value::BulkString(str) => str.clone(),
            _ => panic!("not a bulk string"),
        }
    }
    pub fn encode(self) -> String {
        match &self {
            Value::SimpleString(s) => format!("+{}\r\n", s.as_str()),
            Value::Error(msg) => format!("-{}\r\n", msg.as_str()),
            Value::BulkString(s) => format!("${}\r\n{}\r\n", s.len(), s.as_str()),
            // other cases are not required
            _ => panic!("value encode not implemented for : {:?}, ", self),
        }
    }
}

pub struct RespConnection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl RespConnection {
    pub fn new(stream: TcpStream) -> Self {
        return RespConnection {
            stream, 
            buffer: BytesMut::with_capacity(512),
        };
    }

    pub async fn read_value(&mut self) -> Result<Option<Value>> {
        loop {
            let bytes_read = self.stream.read_buf(&mut self.buffer).await?;

            if bytes_read == 0 {
                return Ok(None);
            }

            if let Some((value, _)) = parse_message(self.buffer.split())? {
                return Ok(Some(value));
            }
        }
    }
    // async function write_value is for writing the value to the stream and returning a Result of ()
    // The type of the empty tuple is written as (). It is not the same as the empty type (), which is a type with no values. The empty tuple is written as (), and it is a value with no fields.
    pub async fn write_value(&mut self, value: Value) -> Result<()> {
        self.stream.write(value.encode().as_bytes()).await?;

        Ok(())
    }
}

fn parse_message(buffer: BytesMut) -> Result<Option<(Value, usize)>> {
    match buffer[0] as char {
        '+' => decode_simple_string(buffer),
        '-' => decode_array(buffer),
        '$' => decode_bulk_string(buffer),
        _ => Err(Error::msg("unrecognised message type"))
    }
}

fn decode_simple_string(buffer: BytesMut) -> Result<Option<(Value, usize)>> {
if let Some((line, len)) = read_until_crlf(&buffer[1..]) {
            let str = parse_string(line)?;

        Ok(Some((Value::SimpleString(str), len + 1)))
    } else {
        Ok(None)
    }
}

fn decode_array(buffer: BytesMut) -> Result<Option<(Value, usize)>> {
    let (array_length, mut bytes_consumed) = if let Some((line, len)) = read_until_crlf(&buffer[1..]) {
        let array_length = parse_integer(line)?;

        (array_length, len + 1)
    } else {
        return Ok(None);
    };

    let mut items: Vec<Value> = Vec::new();
    for _ in 0..array_length {
        if let Some((v, len)) = parse_message(BytesMut::from(&buffer[bytes_consumed..]))? {
            items.push(v);
            bytes_consumed += len;
        } else {
            return Ok(None);
        }
    }
    return Ok(Some((Value::Array(items), bytes_consumed)));
}

fn decode_bulk_string(buffer: BytesMut) -> Result<Option<(Value, usize)>> {
    let (bulk_length, bytes_consumed) = if let Some((line, len)) = read_until_crlf(&buffer[1..]) {
            let bulk_length = parse_integer(line)?;

        (bulk_length, len + 1)
    } else {
        return Ok(None);
    };

    let end_of_bulk = bytes_consumed + (bulk_length as usize);
    let end_of_bulk_line = end_of_bulk + 2;

    return if end_of_bulk_line <= buffer.len() {
        Ok(Some((
            Value::BulkString(parse_string(&buffer[bytes_consumed..end_of_bulk])?),
            end_of_bulk_line,
        )))
    } else {
        Ok(None)
    };
}
fn read_until_crlf(buffer: &[u8]) -> Option<(&[u8], usize)> {
    for i in 1..buffer.len() {
        if buffer[i - 1] == CARRIAGE_RETURN && buffer[i] == NEWLINE {
            return Some((&buffer[0..(i - 1)], i + 1));
        }
    }
    return None;        
    }

fn parse_string(bytes: &[u8]) -> Result<String> {
    String::from_utf8(bytes.to_vec()).map_err(|_| Error::msg("Could not parse string"))
}

fn parse_integer(bytes: &[u8]) -> Result<i64> {
    let str_integer = parse_string(bytes)?;
    (str_integer.parse::<i64>()).map_err(|_| Error::msg("Could not parse integer"))
}