use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    const ADDRESS: &str = "127.0.0.1:9922";
    let listener = TcpListener::bind(ADDRESS);

    for stream in listener.unwrap().incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection established!");
                // Spawn a new thread for each connection to handle multiple clients
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("Failed to establish a connection: {}", e);
            }
        }
    }

    Ok(())
}


fn handle_connection(mut stream: TcpStream)
{
    let mut buffer = [0;512];
    match stream.read(&mut buffer)
    {
        Ok(_) => 
        {
            println!("Recieved: {}",String::from_utf8_lossy(&buffer[..]));

            let response = b"HTTP/1.1 200 OK\r\n\r\nHello";

            stream.write(response).unwrap();
            stream.flush().unwrap();

        }
        Err(e) =>
        {
            println!("{}",e);
        }
    }
}


