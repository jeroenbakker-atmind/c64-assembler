use crate::{instruction::operation::Operation, Application, Instructions};

use super::{AssemblerResult, Error};

pub fn validate_address_names_unique(application: &Application) -> AssemblerResult<()> {
    let mut visited_names = vec![];
    for module in &application.modules {
        validate_instructions(&mut visited_names, &module.instructions)?;
        for function in &module.functions {
            validate_instructions(&mut visited_names, &function.instructions)?;
        }
    }
    Ok(())
}

fn validate_instructions(visited_names: &mut Vec<String>, instructions: &Instructions) -> AssemblerResult<()> {
    for instruction in &instructions.instructions {
        if let Operation::Label(label) = &instruction.operation {
            if visited_names.contains(label) {
                return Err(Error::AddressNameNotUnique(label.to_string()));
            }
            visited_names.push(label.clone());
        }
    }
    Ok(())
}
