/*
use std::net::{TcpListener};
use std::io::{Read, Write};

pub fn start() {
    let listener = TcpListener::bind("0.0.0.0:5000").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        // Handle connection (send command, read response)
    }
}
*/

use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};

fn get_input() -> Option<String> {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();

    if input.is_empty() {
        None
    } else {
        Some(input.to_string())
    }
}

fn handle_client(mut stream: TcpStream) {
    loop {
        if let Some(cmd) = get_input() {
            stream.write(cmd.as_bytes()).unwrap();
        }
    }
}

pub fn start() {
    let listener = TcpListener::bind("0.0.0.0:5000").expect("Failed to bind listener");
    println!("Listening on port 5000...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection!");
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

