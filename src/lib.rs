
// #![feature(thread_local)]
#![ allow( warnings ) ]

pub mod core {
    pub mod hotkey;
    pub mod command;
    pub mod comandr;
    pub mod error;
    pub mod module;
}

pub use core::command::Command;
pub use core::comandr::Comandr;
pub use core::error::ComandrError;
pub use core::error::ComandrResult;
pub use core::module::Module;

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
