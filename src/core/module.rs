use std::slice::Iter;

use crate::Command;
use super::{comandr::Comandr, command, error::ComandrResult};


pub trait Module {

    fn name(&self) -> String;

    fn commands(&self) -> Iter<Command>;

    fn get_command(&mut self, name: String) -> Option<&mut Command>;

    //fn activate(&mut self, comandr: Comandr) -> ComandrResult<()>;

    //fn deactivate(&mut self, comandr: Comandr) -> ComandrResult<()>;
}

pub struct ComandrModule {
    commands: Vec<Command>
}

impl ComandrModule {
    pub fn new() -> ComandrModule {
        let commands = vec![
            Command { name: "down".to_owned(), closure: Box::new(hello) },
            Command { name: "up".to_owned(), closure: Box::new(hello) },
        ];
        ComandrModule { commands }
    }
}

impl Module for ComandrModule {
    fn name(&self) -> String {
        "comandr".to_owned()
    }

    fn commands(&self) -> Iter<'_, Command> {
        self.commands.iter()
    }

    fn get_command(&mut self, name: String) -> Option<&mut Command> {
        for command in self.commands.iter_mut() {
            if command.name == name {
                return Some(command);
            }
        }
        return None;
    }
}

pub fn hello() -> ComandrResult<()> {
    println!("got comnadr command");

    #[cfg(feature = "wasm-target")]
    unsafe {


        use wasm_bindgen::prelude::*;
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = console)]
            fn log(s: &str);
        }
        macro_rules! console_log {
            // Note that this is using the `log` function imported above during
            // `bare_bones`
            ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
        }
        console_log!("got comandr command")
    }
    Ok(())
}
