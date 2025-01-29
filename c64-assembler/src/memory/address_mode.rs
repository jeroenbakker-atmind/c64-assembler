use crate::{validator::AssemblerResult, Application};

use super::{label::AddressReference, Address, ZeroPage};

#[derive(Clone, Debug, Default, PartialEq)]
pub enum AddressMode {
    #[default]
    Implied,
    Accumulator,
    Immediate(Immediate),
    Absolute(AddressReference),
    AbsoluteX(AddressReference),
    AbsoluteY(AddressReference),
    Relative(AddressReference),
    Indirect(AddressReference),
    IndexedIndirect(AddressReference),
    IndirectIndexed(AddressReference),
}

fn is_zeropage(application: &Application, address_reference: &AddressReference) -> AssemblerResult<bool> {
    Ok(application.lookup_address(&address_reference.name)?.is_zeropage())
}

impl AddressMode {
    /// Total number of bytes the instruction occupies on a 6502.
    ///
    /// Application parameter is used to identify if an instruction should use its zeropage variant.
    pub fn byte_size(&self, application: &Application) -> AssemblerResult<Address> {
        match &self {
            AddressMode::Implied | AddressMode::Accumulator => Ok(1),
            AddressMode::Relative(_) | AddressMode::Immediate(_) => Ok(2),
            AddressMode::Absolute(address_reference)
            | AddressMode::AbsoluteX(address_reference)
            | AddressMode::AbsoluteY(address_reference) => {
                if is_zeropage(application, address_reference)? {
                    Ok(2)
                } else {
                    Ok(3)
                }
            }
            AddressMode::Indirect(_address_reference) => Ok(3),
            AddressMode::IndexedIndirect(_address_reference) | AddressMode::IndirectIndexed(_address_reference) => {
                Ok(2)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Immediate {
    Byte(u8),
    Low(AddressReference),
    High(AddressReference),
}
