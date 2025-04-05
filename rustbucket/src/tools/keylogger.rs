use rdev::{listen, Event};

fn callback(event: Event) {
    println!("{:?}", event);
}

fn main() {
    if let Err(e) = listen(callback) {
        eprintln!("Error: {:?}", e);
    }
}

