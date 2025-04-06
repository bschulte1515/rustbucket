use arboard::Clipboard;

pub fn run() -> Result<(), &'static str> {
    let mut clipboard = Clipboard::new().expect("Failed to open clipboard");
    
    let text_to_copy = "1337";

    clipboard.set_text(text_to_copy).expect("Failed to set text of clipboard");
    Ok(())
}
