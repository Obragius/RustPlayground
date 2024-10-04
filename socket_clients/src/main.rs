use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;
use std::convert::TryInto;

use serde_json::{Value, Error};

fn main() -> std::io::Result<()> {
    const ADDRESS: &str = "127.0.0.1:80";
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

fn create_response(message: &str)
{
    //println!("{}",message);
    let json: Result<Value, Error> = serde_json::from_str(&message);
    match json {
        Ok(json) =>
        {
            println!("Success: {}",json["value"]);
        }
        Err(e) =>
        {
            println!("Error: {}",e);
        }
    }
}


fn handle_connection(mut stream: TcpStream)
{
    let mut buffer = [0;512];
    match stream.read(&mut buffer)
    {
        Ok(_) => 
        {

            let recieved = String::from_utf8_lossy(&buffer[..]);

            let index_item = &recieved.find("Content-Length:");

            let mut data_index: usize = 0;

            match index_item {
                Some(index) =>
                {
                    data_index = *index;
                }
                None =>
                {

                }
            }


            let partial_data: &str  = &recieved[data_index..];

            let mut data :&str = "";

            let mut size : usize= 0;

            if let Some(pos) = partial_data.find("\n")
            {
                let index_item = &partial_data.find("Content-Length:");
                match index_item {
                    Some(index) =>
                    {
                        data_index = *index;
                    }
                    None =>
                    {
    
                    }
                }
                match &partial_data[16+data_index..pos-1].parse::<usize>() {
                    Ok(number) => size = *number,
                    Err(e) => println!("Failed to convert to usize: {}", e),
                }
                println!("{}",&partial_data[16+data_index..pos-1]);
            }

            if let Some(pos) = partial_data.find("\n")
            {
                data = &partial_data[pos+3..pos+3+size];
            }

            create_response(data);

            let response = b"HTTP/1.1 200 OK\r\n\r\n";

            stream.write(response).unwrap();
            stream.flush().unwrap();

        }
        Err(e) =>
        {
            println!("{}",e);
        }
    }
}


