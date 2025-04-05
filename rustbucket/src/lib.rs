mod tools;

pub struct Config {
    tool: tools::Tool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> None { // -> Result<Config, &'static str) {
        args.next(); // Skip name of the program 
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-t" => {
                    if let Some(value) = arg.next() {
                        println!("Using tool {value}");
                        return Config(  )
                    } else {
                        println!("Did not supply a tool name");
                    }
                }
                _ => println!("Invalid flag: {arg}");
            }
        }
    }
}

pub fn run() -> 
