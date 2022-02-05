// Generated by gir (https://github.com/gtk-rs/gir @ e0d8d8d645b1)
// from ../gir-files (@ 54ae87ae2ece)
// from ../xrd-gir-files (@ 7913ad2986bb+)
// DO NOT EDIT

mod client;
pub use self::client::Client;

mod container;
pub use self::container::Container;

mod desktop_cursor;
pub use self::desktop_cursor::DesktopCursor;

mod input_synth;
pub use self::input_synth::InputSynth;

mod window;
pub use self::window::Window;

mod window_manager;
pub use self::window_manager::WindowManager;

mod enums;
pub use self::enums::ClientMode;

pub mod functions;

#[doc(hidden)]
pub mod traits {
    pub use super::client::ClientExt;
    pub use super::desktop_cursor::DesktopCursorExt;
    pub use super::window::WindowExt;
}
