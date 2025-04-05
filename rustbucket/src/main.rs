use std::{env, process};

fn main() {
    let tool = rustbucket::build(env::args()).unwrap_or_else(|err| {
        eprintln!("rustbucket encountered an error: {err}");
        process::exit(1);
    }); 
    if let Err(e) = rustbucket::run(tool) {
        eprintln!("rustbucket encountered an error: {e}");
        process::exit(1);
    }
}
