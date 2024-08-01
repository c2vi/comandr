
#![feature(thread_local)]

pub mod core {
    pub mod hotkey;
    pub mod command;
}

pub use core::command;
pub use core::hotkey;

pub mod platform {
    pub mod qt;
}

pub mod ui {
    pub mod slint {
        slint::include_modules!();
    }
}
