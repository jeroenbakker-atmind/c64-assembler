use crate::{memory::address_mode::AddressMode, Application, Instructions};

use super::{Error, ValidatorResult};

pub fn validate_address_names_exists(application: &Application) -> ValidatorResult<()> {
    for module in &application.modules {
        validate_instructions(application, &module.instructions)?;
        for function in &module.functions {
            validate_instructions(application, &function.instructions)?;
        }
    }
    Ok(())
}

fn validate_instructions(application: &Application, instructions: &Instructions) -> ValidatorResult<()> {
    for instruction in &instructions.instructions {
        match &instruction.address_mode {
            AddressMode::Implied | AddressMode::Accumulator | AddressMode::Immediate(_) => {}
            AddressMode::Absolute(address_reference)
            | AddressMode::AbsoluteX(address_reference)
            | AddressMode::AbsoluteY(address_reference)
            | AddressMode::Relative(address_reference)
            | AddressMode::Indirect(address_reference)
            | AddressMode::IndexedIndirect(address_reference)
            | AddressMode::IndirectIndexed(address_reference) => {
                if !application.address_lookup.contains_key(&address_reference.name) {
                    return Err(Error::AddressNameUnknown(address_reference.name.to_string()));
                }
            }
        }
    }
    Ok(())
}
