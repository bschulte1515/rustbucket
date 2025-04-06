use arboard::Clipboard;

pub fn run() -> Result<String, &'static str> {
    let mut clipboard = Clipboard::new().expect("Failed to open clipboard");

    match clipboard.get_text() {
        Ok(text) => { return Ok(text); }
        Err(_) => { return Err("Error getting clipboard text"); }
    }
}
