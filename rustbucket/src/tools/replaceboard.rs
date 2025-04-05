use arboard::Clipboard;
use std::io::{self, Write};

fn get_input() -> Option<String> {
    println!("Enter text to paste to clipboard: ");
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

pub fn run() -> Result<(), &'static str> {
    let mut clipboard = Clipboard::new().expect("Failed to open clipboard");

    let text_to_copy = match get_input() {
        Some(text) => text,
        None => "1337".to_string(),
    };

    clipboard.set_text(&text_to_copy).expect("Failed to set text of clipboard");
    Ok(())
}
