use std::io::{self, Write};

pub fn get_input() -> Option<String> {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();

    if input.is_empty() {
        None
    } else {
        Some(input.to_string())
    }
}


