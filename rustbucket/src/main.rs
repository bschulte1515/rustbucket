use std::{env, process, error::Error};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("rustbucket encountered an error: {err}");
        process:exit(1);
    })

    if let Err(e) = rustbucket::run() {
        eprintln!("rustbucket encountered an error: {e}");
        process:exit(1);
    }
}
