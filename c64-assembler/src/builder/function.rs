use crate::{Function, Instructions};

#[derive(Default, Clone)]
pub struct FunctionBuilder {
    function: Function,
}

impl FunctionBuilder {
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.function.name = name.to_string();
        self
    }

    pub fn doc(&mut self, documentation: &[&str]) -> &mut Self {
        for d in documentation {
            self.function.documentation.push(d.to_string());
        }
        self
    }

    pub fn instructions(&mut self, instructions: Instructions) -> &mut Self {
        self.function.instructions = instructions;
        self
    }

    pub fn build(&self) -> Function {
        self.function.clone()
    }
}
