use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::format;
use std::string::FromUtf8Error;
use std::io;
use colored::Colorize;
use tracing::{trace, debug, info, warn, error};

#[macro_export]
macro_rules! comandr_err {
    ($($arg:tt)*) => { ComandrError::new().msg(format!( $($arg)*)) };
}

pub type ComandrResult<T> = Result<T, ComandrError>;

pub trait IntoComandrResult<T, S> {
    fn comandr_result(self) -> ComandrResult<T>;
    fn comandr_result_msg(self, msg: S) -> ComandrResult<T> where S: std::fmt::Display ;
}

#[derive(Debug, Clone)]
pub struct ComandrError {
    pub messages: Vec<String>,
    pub code_location: Option<ComandrCodeLocation>,
}

#[derive(Debug, Clone)]
pub struct ComandrCodeLocation {
    file: String,
    line: u32,
    column: u32,
}

impl Display for ComandrCodeLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "file: {} line: {} column: {}", self.file, self.line, self.column)
    }
}

impl From<&std::panic::Location <'_>> for ComandrCodeLocation {
    fn from(panic_location: &std::panic::Location) -> ComandrCodeLocation {
        let file = panic_location.file().to_string();
        let line = panic_location.line();
        let column = panic_location.column();
        ComandrCodeLocation {file, line, column}
    }
}

impl ComandrError {
    #[track_caller]
    pub fn new() -> ComandrError {
        let caller_location = std::panic::Location::caller();

        return ComandrError {
            messages: Vec::new(),
            code_location: Some(caller_location.into()),
        }
    }

    pub fn msg<E>(mut self, msg: E) -> ComandrError 
        where E: std::fmt::Display
    {
        self.messages.push(format!("{}", msg));
        return self;
    }
    
    pub fn log(self) -> ComandrError {
        error!("ComandrError envountered!");
        if let Some(ref location) = self.code_location {
            error!("[ {} ] {}", "LOCATION".yellow(), location);
        };
        for msg in &self.messages {
            error!("[ {} ] {}", "MSG".yellow(), msg);
        }
        self
    }

    pub fn critical(self){
        let mut msg_iter = self.messages.iter();
        error!("ComandrError");
        for msg in msg_iter {
            error!("{} {}", "MSG".red(), msg)
        }
        if let Some(location) = self.code_location {
            error!("{} {}", "LOCATION".red(), location)
        }
        panic!();
    }

    pub fn location(mut self, location: ComandrCodeLocation) -> ComandrError {

        self.code_location = Some(location);

        self
    }

}


impl<T: Display> From<T> for ComandrError {
    #[track_caller]
    fn from(value: T) -> Self {
        let caller_location = std::panic::Location::caller();
        ComandrError::new()
            .msg(format!("From {}: {}", std::any::type_name_of_val(&value), value))
            .location(caller_location.into())
    }
}

impl<T, E, S> IntoComandrResult<T, S> for Result<T, E> where E: std::fmt::Display {
    #[track_caller]
    fn comandr_result(self) -> ComandrResult<T> {
        let caller_location = std::panic::Location::caller();

        match self {
            Ok(val) => ComandrResult::Ok(val),
            Err(err) => ComandrResult::Err(ComandrError::new()
                .msg(format!("From {}: {}", std::any::type_name_of_val(&err), err))),
        }
    }
    #[track_caller]
    fn comandr_result_msg(self, msg: S) -> ComandrResult<T> where S: std::fmt::Display {
        let caller_location = std::panic::Location::caller();
        match self {
            Ok(val) => ComandrResult::Ok(val),
            Err(err) => ComandrResult::Err(ComandrError::new()
                .msg(format!("{}", msg))
                .msg(format!("From {}: {}", std::any::type_name_of_val(&err), err))),
        }
    }
}




