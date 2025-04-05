use std::{thread, time::Duration, error::Error};
use rdev::{simulate, EventType};

pub fn run() -> Result<(), Box<dyn Error>> {
    let event = EventType::MouseMove {x: 500.0, y: 300.0 };
    simulate(&event)?;
    
    thread::sleep(Duration::from_millis(100)); // Give time for the move to register
    Ok(())
}
