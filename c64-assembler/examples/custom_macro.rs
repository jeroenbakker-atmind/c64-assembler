//! Example showing how to add custom macros to the instruction builder.
use c64_assembler::{
    builder::{ApplicationBuilder, InstructionBuilder, ModuleBuilder},
    validator::AssemblerResult,
};

fn main() -> AssemblerResult<()> {
    let _application = ApplicationBuilder::default()
        .name("Example using custom macros")
        .define_address("CURRENT_PTR", 0x00FE)
        .module(
            ModuleBuilder::default()
                .name("main")
                .instructions(InstructionBuilder::default().lda_current_ptr_offs(2).build())
                .build(),
        )
        .build()?;
    Ok(())
}

/// MyMacros contains the macros that we want to extend InstructionBuilder with.
///
/// We can then implement the trait on the InstructionBuilder and call the
/// macro directly when adding other instructions as well.
pub trait MyMacros {
    fn lda_current_ptr_offs(&mut self, offset: u8) -> &mut Self;
}

impl MyMacros for InstructionBuilder {
    fn lda_current_ptr_offs(&mut self, offset: u8) -> &mut Self {
        self.ldy_imm(offset).lda_ind_y("CURRENT_PTR")
    }
}
