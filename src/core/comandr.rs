use std::collections::HashMap;

use crate::{core::module::Module, ComandrResult};
use crate::Command;
use simsearch::{SearchOptions, SimSearch};

use super::module::ComandrModule;


pub struct Comandr {
    pub modules: HashMap<String, Box<dyn Module + Send + Sync>>,
    pub engine: SimSearch<String>, // module:comand
}

impl Comandr {
    pub fn new() -> Comandr {
        let mut engine: SimSearch<String> = SimSearch::new_with(
            SearchOptions::new()
                .levenshtein(true)
                .threshold(0.01)
        );
        let mut modules: HashMap<String, Box<dyn Module + Send + Sync>> = HashMap::new();

        let comandr_module: Box<dyn Module + Send + Sync> = Box::new(ComandrModule::new());

        let mut comandr = Comandr { modules, engine };

        comandr.load_module(comandr_module);

        comandr

        
    }

    pub fn load_module(&mut self, module: Box<dyn Module + Send + Sync>) {
        for command in module.commands() {
            self.activate_command(&module, command);
        }

        self.modules.insert(module.name(), module);
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

    pub fn get_command(&mut self, name: String) -> Option<&mut Command> {
        for (_, mut module) in self.modules.iter_mut() {
            let name_inner = name.split(":").nth(1).expect("command name had no :");
            if let Some(cmd) = module.get_command(name_inner.to_owned()) {
                return Some(cmd);
            }
        }
        None
    }

    pub fn activate_command(&mut self, module: &Box<dyn Module + Send + Sync>, command: &Command) {
        let name = module.name() + ":" + command.name.as_str();
        self.engine.insert(name.clone(), name.as_str());
    }

    pub fn search(&mut self, string: String) -> Vec<String> {
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

