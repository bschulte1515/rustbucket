mod tools;
use tools::{clipboard, replaceboard, Tool};
use std::{
    error::Error,
};

fn check_tool(input: &str) -> Tool {
    match input {
        "keylogger" => return Tool::Keylogger,
        "clipboard" => return Tool::Clipboard,
        "replaceboard" => return Tool::ReplaceBoard,
        _ => return Tool::Invalid, 
    }
}

pub fn build(mut args: impl Iterator<Item = String>) -> Result<Tool, &'static str> {
    args.next(); // Skip the first argument (usually the program name)
    
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-t" | "--tool" => {
                if let Some(value) = args.next() {
                    return Ok(check_tool(&value));
                } else {
                    return Err("Did not supply a tool name");
                }
            }
            "--list" => {
                return Err("\n\nAvailable tools:
- clipboard         Outputs the current clipboard
- replaceboard      Replaces the clipboard text
- keylogger         Listens for keypresses for 10 seconds\n");
            }
            _ => return Err("Invalid flag"),
        }
    }
    
    Err("\
No tool specified. Use -t or --tool to specify a tool
Use --list to see a list of available tools")
}

pub fn run(tool: Tool) -> Result<(), Box<dyn Error>> {
    match tool {
        Tool::Keylogger => {
            // You can implement the logic for Keylogger here
            todo!("Please implement me!");
        }
        Tool::Clipboard => {
            // You can implement the logic for Clipboard here
            let _ = clipboard::run()?;
            return Ok(());
        }
        Tool::ReplaceBoard => {
            let _ = replaceboard::run()?;
            return Ok(());

        }
        Tool::Ghost => {
            todo!("Please implement me!");
        }
        Tool::Invalid => {
            // Return an error if the tool is invalid
            return Err("Invalid tool".into());
        }
    }
}
