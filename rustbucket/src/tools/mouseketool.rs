use std::error::Error;
use mouse_rs::Mouse;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mouse = Mouse::new();
    mouse.move_to(500, 500)?;
    Ok(())
}
