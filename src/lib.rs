
// #![feature(thread_local)]
#![ allow( warnings ) ]

pub mod core {
    pub mod hotkey;
    pub mod command;
}

pub use core::command;
pub use core::hotkey;

pub mod platform {
    #[cfg(features = "qt")]
    pub mod qt;
    pub mod web;
}

pub mod ui {
    #[cfg(features = "qt")]
    pub mod slint {
        slint::include_modules!();
    }
}
