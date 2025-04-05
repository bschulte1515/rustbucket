mod tools;
use tools::Tool;

fn check_tool(value: &str) -> Tool {
    match value {
        "keylogger" => Tool::Keylogger,
        "clipboard" => Tool::Clipboard,
        _ => Tool::Invalid, 
    }
}

pub fn build(mut args: impl Iterator<Item = String>) -> Result<Tool, &'static str> {
    args.next(); 
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-t" | "--tool" => {
                if let Some(value) = args.next() {
                    let tool = check_tool(&value);
                    if matches!(tool, Tool::Invalid) {
                        return Err("Invalid tool specified");
                    }
                    return Ok(tool);
                } else {
                    return Err("Did not supply a tool name");
                }
            }
            _ => return Err("Invalid flag"),
        }
    }
    
    Err("No tool specified. Use -t or --tool to specify a tool.")
}
