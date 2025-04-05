mod tools;
use tools::Tool;

fn check_tool(&input: str) -> Tool {
    match value.as_str() {
        "keylogger" => return Tool::Keylogger,
        "clipboard" => return Tool::Clipboard,
        _ => return Tool::Invalid, 
    }
}

pub fn build(mut args: impl Iterator<Item = String>) -> Result<Tool, &'static str> {
    args.next(); 
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-t" | "--tool" => {
                if let Some(value) = args.next() {
                    if value == Tool::Invalid {
                        return Err("Invalid tool: {value}");
                    }
                    return Ok(check_tool(value));
                } else {
                    return Err("Did not supply a tool name");
                }
            }
            _ => return Err("Invalid flag: {arg}"),
        }
    }
    
    Err("No tool specified. Use -t or --tool to specify a tool.")
}
