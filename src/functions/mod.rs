pub use window::*;
pub use text_editor::*;
pub use folders::*;
pub use terminal::*;
pub use commands::*;
pub use commands_for_windows::*;
pub use commands_for_linux::*;

pub mod commands_for_linux;
pub mod commands_for_windows;
pub mod commands;
pub mod terminal;
pub mod folders;
pub mod text_editor;
pub mod window;
