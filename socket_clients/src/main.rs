use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;
use std::convert::TryInto;

use serde_json::{Value, Error};

fn main() -> std::io::Result<()> {
    // Create a constant address for the server to listen at
    const ADDRESS: &str = "127.0.0.1:9922";
    // This initilizes this address to listen for data at that port
    let listener = TcpListener::bind(ADDRESS);

    // For incoming connection on the port
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

// This function can be used to craft a response depending on the message recieved on the port
fn create_response(message: &str) -> Vec<u8>
{
    // The data given here must be in json format, so that all json data can be extracted
    let json: Result<Value, Error> = serde_json::from_str(&message);
    match json {
        Ok(json) =>
        {
            // If the json is extracted correctly print the contents to see
            println!("Success: {}",json["value"]);
            let mut response: Vec<u8>;
            if (json["value"].is_null())
            {
                response = b"HTTP/1.1 400 Error\r\n\r\nValue is has not been found".to_vec();
            }
            else {
                response = b"HTTP/1.1 200 OK\r\n\r\nValue is:".to_vec();
            response.extend(json["value"].as_str().unwrap().as_bytes().to_vec());
            }
            // the response must be converted to a 512 byte array, so the vector must be adjusted
            while (response.len() < 512)
            {
                response.extend(b" ".to_vec());
            } 
            return response;
        }
        Err(e) =>
        {
            // Print error if any
            println!("Error: {}",e);
            let mut response: Vec<u8> = b"HTTP/1.1 500 Error\r\n\r\nJson could not be parsed".to_vec();
            // the response must be converted to a 512 byte array, so the vector must be adjusted
            while (response.len() < 512)
            {
                response.extend(b" ".to_vec());
            } 
            return response;
        }
    }
}

// This function handles the connection
fn handle_connection(mut stream: TcpStream)
{
    // This is the maximum size of the message that the function will handle
    // This will need to be synchronized for when bigger data will be sent
    let mut buffer = [0;512];
    // This will try to read the byte data on the port
    match stream.read(&mut buffer)
    {
        Ok(_) => 
        {
            // Recieved bytes converted into text
            let recieved = String::from_utf8_lossy(&buffer[..]);

            // This will find where this line is in the string
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

            // Delete all data from start to "Content-Length:"
            let partial_data: &str  = &recieved[data_index..];

            // This is used to gather the size of the string, which is important as we need to delete all the null bytes which are left over in the buffer
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

            // Response will be a vector, this will be converted to an array of the same size as the buffer
            let response = create_response(data);

            let byte_response: [u8;512] = response.try_into().expect("Incorrect vector length");

            stream.write(&byte_response).unwrap();
            stream.flush().unwrap();

        }
        Err(e) =>
        {
            println!("{}",e);
        }
    }
}


