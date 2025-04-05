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
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) if n == 0 => break, // Connection closed
            Ok(n) => {
                let cmd = String::from_utf8_lossy(&buffer[..n]);
                println!("Received command: {}", cmd);

                // Example response
                let response = format!("Command received: {}", cmd);
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                break;
            }
        }
    }
}

pub fn start() {
    let listener_thread = thread::spawn(|| {
        let listener = TcpListener::bind("0.0.0.0:5000").expect("Failed to bind listener");
        println!("Listening on port 5000...");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection!");
                    thread::spawn(|| handle_client(stream));
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
    });

    // Do other stuff in the main thread here if you want

    listener_thread.join().unwrap(); // Optionally wait for the listener to finish
}

