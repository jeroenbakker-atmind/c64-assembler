use crate::{Function, Instructions, Module};

#[derive(Default, Clone)]
pub struct ModuleBuilder {
    module: Module,
}

impl ModuleBuilder {
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.module.name = name.to_string();
        self
    }

    pub fn instructions(&mut self, instructions: Instructions) -> &mut Self {
        self.module.instructions = instructions;
        self
    }
    pub fn function(&mut self, function: Function) -> &mut Self {
        self.module.functions.push(function);
        self
    }

    pub fn finalize(&self) -> Module {
        self.module.clone()
    }
}
