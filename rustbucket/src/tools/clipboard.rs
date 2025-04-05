use arboard::Clipboard;

fn run() -> Result<String, String> {
    let mut clipboard = Clipboard::new().expect("Failed to open clipboard");

    match clipboard.get_text() {
        Ok(text) => return Ok(text),
        Err(err) => return Err("Error getting clipboard text"),
    }
}
