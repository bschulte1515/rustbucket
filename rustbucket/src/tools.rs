pub mod keylogger;
pub mod clipboard;
pub mod replaceboard;
pub mod ghost;
pub mod mouseketool;
pub mod obfuscate;

pub enum Tool {
    Keylogger,
    Clipboard,
    ReplaceBoard,
    Ghost,
    Mouseketool,
    Obfuscate,
    Invalid,
}
