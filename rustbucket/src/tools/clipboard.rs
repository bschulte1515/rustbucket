use arboard::Clipboard;

pub fn run() -> Result<String, &'static str> {
    let mut clipboard = Clipboard::new().expect("Failed to open clipboard");

    match clipboard.get_text() {
        Ok(text) => { println!("Clipboard text: {text}");
                      return Ok(text); }
        Err(_) => { println!("Error getting clipboard text"); 
                    return Err("Error getting clipboard text"); }
    }
}
