use operation::Operation;

use crate::memory::address_mode::AddressMode;
use crate::memory::Address;
use crate::validator::AssemblerResult;
use crate::Application;
pub mod operation;

/// Assembly instruction
///
/// An instruction is the combination of the operation and the address mode that the
/// operation should be using.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Instruction {
    /// Operation of the instruction.
    pub operation: Operation,
    /// Address mode of the instruction.
    pub address_mode: AddressMode,
    /// Comments for when generating source code.
    pub comments: Vec<String>,
}

impl Instruction {
    /// Total number of bytes the instruction occupies on a 6502.
    ///
    /// Application parameter is used to identify if an instruction should use its zeropage variant.
    pub fn byte_size(&self, application: &Application) -> AssemblerResult<Address> {
        if let Operation::Raw(bytes) = &self.operation {
            Ok(bytes.len() as u16)
        } else if let Operation::Label(_) = &self.operation {
            Ok(0)
        } else {
            self.address_mode.byte_size(application)
        }
    }
}
