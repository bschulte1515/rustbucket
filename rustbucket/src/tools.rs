// mod keylogger;
pub mod clipboard;
pub mod replaceboard;
pub mod ghost;

pub enum Tool {
    Keylogger,
    Clipboard,
    ReplaceBoard,
    Ghost,
    Invalid,
}
