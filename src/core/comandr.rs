use std::collections::HashMap;

use crate::{core::module::Module, ComandrResult};
use crate::Command;
use simsearch::SimSearch;

use super::module::ComandrModule;


////// console_log!() macro
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
/// end of macro



pub struct Comandr {
    pub modules: HashMap<String, Box<dyn Module + Send + Sync>>,
    pub engine: SimSearch<String>, // module:comand
    pub hi: String,
}

impl Comandr {
    pub fn new() -> Comandr {
        let mut engine: SimSearch<String> = SimSearch::new();
        let mut modules: HashMap<String, Box<dyn Module + Send + Sync>> = HashMap::new();

        let comandr_module: Box<dyn Module + Send + Sync> = Box::new(ComandrModule::new());

        let mut comandr = Comandr { modules, engine, hi: "hiiiii inside comandr".to_owned() };


        for command in comandr_module.commands() {
            comandr.activate_command(&comandr_module, command);
        }

        let result = comandr.search("comandr".to_owned());
        console_log!("result in new: {:?}", result);

        comandr.modules.insert("comandr".to_owned(), comandr_module);

        comandr

        
    }

    pub fn list_commands(&self) -> Vec<String> {
        let mut commands: Vec<String> = Vec::new();
        for (name, module) in self.modules.iter() {
            for command in module.commands() {
                commands.push(command.name.clone());
            }
        }
        commands
    }

    pub fn activate_command(&mut self, module: &Box<dyn Module + Send + Sync>, command: &Command) {
        let name = module.name() + ":" + command.name.as_str();
        self.engine.insert(name.clone(), name.as_str());
    }

    pub fn search(&mut self, string: String) -> Vec<String> {
        console_log!("comandr search method");
        let result = self.engine.search(string.as_str());
        return result;
    }

    pub fn execute(&mut self, command: String, args: Vec<String>) {
        let module_name = command.split(":").nth(0).expect("comandr id had no module part");
        let command_name = command.split(":").nth(1).expect("comandr id had no command part");
        let module = self.modules.get_mut(module_name).expect("module not found");
        let mut command = module.get_command(command_name.to_owned()).expect("command not found");
        (command.closure)();
    }

    pub fn init(&mut self) -> ComandrResult<()> {

        #[cfg(feature = "wasm-target")]
        {
            crate::platform::web::init(self);
        }

        Ok(())
    }
}

pub fn test_fn() {
    println!("hello from test_fn");
}

