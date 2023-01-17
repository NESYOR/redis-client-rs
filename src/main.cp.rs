// use bincode::{deserialize, serialize};
// use serde::{Deserialize, Serialize};
// use std::io::prelude::*;
// use std::net::TcpStream;

// #[derive(Serialize, Deserialize, Debug)]
// enum Value {
//     SimpleString(String),
//     Error(String),
//     BulkString(String),
//     Array(Vec<Value>),
//     Null,
// }

// impl Value {
//     fn unwrap_bulk(&self) -> String {
//         match self {
//             Value::BulkString(s) => s.to_string(),
//             _ => "".to_string(),
//         }
//     }
// }

// fn send_command(stream: &mut TcpStream, command: &str, args: Vec<&str>) -> Value {
//     let mut values = vec![Value::SimpleString(command.to_string())];
//     for arg in args {
//         values.push(Value::BulkString(arg.to_string()));
//     }
//     let array = Value::Array(values);
//     let bytes = serialize(&array).unwrap();
//     stream.write_all(&bytes).unwrap();
//     stream.flush().unwrap();

//     let mut buffer = [0; 1024];
//     let size = stream.read(&mut buffer).unwrap();
//     let bytes = &buffer[..size];
//     deserialize(bytes).unwrap()
// }

// fn main() {
//     let mut stream = TcpStream::connect("127.0.0.1:6379").unwrap();

//     let ping = send_command(&mut stream, "ping\r\n", vec![]);
//     match ping {
//         Value::SimpleString(s) => println!("PING: {}", s),
//         _ => println!("Unexpected response"),
//     }

//     // let set = send_command(&mut stream, "SET", vec!["key", "value"]);
//     // match set {
//     //     Value::SimpleString(s) => println!("SET: {}", s),
//     //     _ => println!("Unexpected response"),
//     // }

//     // let get = send_command(&mut stream, "GET", vec!["key"]);
//     // match get {
//     //     Value::BulkString(s) => println!("GET: {}", s),
//     //     _ => println!("Unexpected response"),
//     // }
// }

use std::io::{self, Write};
use tokio::net::TcpStream;
mod resp;
async fn run_client() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379").await?;
    let mut conn = resp::RespConnection::new(stream);

    loop {
        print!("Enter command: ");
        io::stdout().flush()?;

        let mut command = String::new();
        io::stdin().read_line(&mut command)?;

        let command_string = command.trim().to_string();
        let command_vec: Vec<&str> = command_string.split_whitespace().collect();

        let mut value = Vec::new();
        for cmd in command_vec {
            value.push(resp::Value::SimpleString(cmd.to_string()))
        }
        let message = resp::Value::Array(value);
        //conn.write_value(message).await?;
        conn.write_value(message).await.unwrap();
        let response = conn.read_value().await.unwrap();
        match response {
            Some(resp) => println!("Response: {:?}", resp),
            None => println!("No response"),
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    run_client().await
}
