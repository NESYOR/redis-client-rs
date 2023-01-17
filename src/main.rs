// use std::io::{self, Write};
// use std::str;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpStream;

const CR: u8 = b'\r';
// const LF: u8 = b'\n';

// async fn run_client() -> io::Result<()> {
//     let mut stream = TcpStream::connect("127.0.0.1:6379").await?;

//     loop {
//         print!("Enter command: ");
//         io::stdout().flush()?;

//         let mut command = String::new();
//         io::stdin().read_line(&mut command)?;
//         let command = command.trim();

//         // Send the command to the server
//         stream
//             .write(format!("*{}\r\n", command.split_whitespace().count()).as_bytes())
//             .await?;
//         for cmd in command.split_whitespace() {
//             stream
//                 .write(format!("${}\r\n", cmd.as_bytes().len()).as_bytes())
//                 .await?;
//             stream.write(cmd.as_bytes()).await?;
//             stream.write(&[CR, LF]).await?;
//         }

//         // Read the response from the server
//         let mut buffer = [0; 1];
//         stream.read_exact(&mut buffer).await?;
//         let _message_type = buffer[0];

//         //     match message_type {
//         //         b'+' => {
//         //             let mut line = Vec::new();
//         //             stream.read_until(CR, &mut line).await?;
//         //             let line = str::from_utf8(&line)?.trim_end_matches("\r");
//         //             println!("Response: {}", line);
//         //         }
//         //         b'-' => {
//         //             let mut line = Vec::new();
//         //             stream.read_until(CR, &mut line).await?;
//         //             let line = str::from_utf8(&line)?.trim_end_matches("\r");
//         //             println!("Error: {}", line);
//         //         }
//         //         b'$' => {
//         //             let mut len_bytes = [0; 32];
//         //             stream.read_exact(&mut len_bytes).await?;
//         //             let len = str::from_utf8(&len_bytes)?
//         //                 .trim_end_matches("\r")
//         //                 .parse::<usize>()?;
//         //             let mut value = vec![0; len];
//         //             stream.read_exact(&mut value).await?;
//         //             let value = str::from_utf8(&value)?;
//         //             println!("Response: {}", value);
//         //             stream.read_exact(&mut buffer).await?;
//         //             if buffer[0] != CR {
//         //                 return Err(io::Error::new(io::ErrorKind::Other, "Invalid response"));
//         //             }
//         //             stream.read_exact(&mut buffer).await?;
//         //             if buffer[0] != LF {
//         //                 return Err(io::Error::new(io::ErrorKind::Other, "Invalid response"));
//         //             }
//         //         }
//         //         _ => {
//         //             return Err(io::Error::new(io::ErrorKind::Other, "Invalid response"));
//         //         }
//         //     }
//     }
// }

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     run_client().await
// }

use std::io::{stdin, BufRead, Write};
use std::str;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379").await?;
    let (r, w) = stream.split();
    let stdin = stdin();
    let mut command = String::new();

    loop {
        print!("Enter command: ");
        io::stdout().flush().await?;
        stdin.lock().read_line(&mut command)?;

        let command_t = command.trim();

        if command_t == "PING" {
            println!("writing ping");
            stream.write(b"*1\r\n$4\r\nPING\r\n").await?;
            let mut buffer = [0; 1];
            stream.read_exact(&mut buffer).await?;
            let message_type = buffer[0];
            println!("message_type:{}", message_type);
            match message_type {
                b'+' => {
                    let mut buffer = [0; 4];
                    stream.read_exact(&mut buffer).await?;
                    println!("Response: {:?}", std::str::from_utf8(&buffer));
                }
                _ => println!("{} first byte not read", message_type),
            }
        } else if command_t.starts_with("SET") {
            let args: Vec<&str> = command_t.split_whitespace().collect();
            if args.len() != 3 {
                println!("Invalid SET command_t, format should be SET key value");
                continue;
            }
            let key = args[1];
            let value = args[2];
            let command_t = format!(
                "*3\r\n$3\r\nSET\r\n${}\r\n{}\r\n${}\r\n{}\r\n",
                key.len(),
                key,
                value.len(),
                value
            );
            let mut buffer = [0; 1];
            stream.read_exact(&mut buffer).await?;
            stream.write(command_t.as_bytes()).await?;
            let mut buffer = [0; 2];
            stream.read_exact(&mut buffer).await?;
            let message_type = buffer[1];
            println!("message_type:{},{}", buffer[0], buffer[1]);
            match message_type {
                b'+' => {
                    let mut buffer: [u8; 2] = [0; 2];
                    stream.read_exact(&mut buffer).await?;
                    println!("Response: {:?}", std::str::from_utf8(&buffer));
                }
                _ => println!("{} first byte not read", message_type),
            }
        } else if command_t.starts_with("GET") {
            let args: Vec<&str> = command_t.split_whitespace().collect();
            if args.len() != 2 {
                println!("Invalid GET command_t, format should be GET key");
                continue;
            }
            let key = args[1];
            let command_t = format!("*2\r\n$3\r\nGET\r\n${}\r\n{}\r\n", key.len(), key);
            stream.write(command_t.as_bytes()).await?;

            let mut buffer = [0; 1];
            stream.read_exact(&mut buffer).await?;
            stream.write(command_t.as_bytes()).await?;
            let mut buffer = [0; 2];
            stream.read_exact(&mut buffer).await?;
            let message_type = buffer[0];
            println!("message_type:{},{}", buffer[0], buffer[1]);
            match message_type {
                b'+' => {
                    let mut buffer = vec![0; 1024];
                    let bytes_read = stream.read(&mut buffer).await?;
                    buffer.truncate(bytes_read);
                    println!("Response: {:?}", std::str::from_utf8(&buffer));
                }
                _ => println!("{} first byte not read", message_type),
            }
        } else {
            println!("command_t IS:{}", command);
            println!("Invalid command, please enter PING, SET key value, or GET key");
        }
        command.clear();
    }
}
