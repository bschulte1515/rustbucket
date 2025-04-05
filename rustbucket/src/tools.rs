pub mod keylogger;
pub mod clipboard;
pub mod replaceboard;
pub mod ghost;
pub mod obfuscate;

pub enum Tool {
    Keylogger,
    Clipboard,
    ReplaceBoard,
    Ghost,
    Obufscate,
    Invalid,
}
