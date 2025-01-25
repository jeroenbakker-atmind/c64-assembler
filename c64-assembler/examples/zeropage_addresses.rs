//! Show how to use defines
use c64_assembler::builder::ApplicationBuilder;
use c64_assembler::builder::InstructionBuilder;
use c64_assembler::builder::ModuleBuilder;
use c64_assembler::generator::{print_hexdump, ProgramGenerator};
use c64_assembler::generator::Generator;

fn main() {
    let zeropage_fe = "ADDRESS_ZEROPAGE_FE";
    let address_c000 = "ADDRESS_C000";

    let application = ApplicationBuilder::default()
        .name("Defines example")
        // Define an address
        .define_address(address_c000, 0xC000)
        // Define an zeropage address
        .define_address(zeropage_fe, 0xFE)
        .module(
            ModuleBuilder::default()
                .name("main")
                .instructions(
                    InstructionBuilder::default()
                        // Will use 0xAD as opcode as it points to a non zeropage address
                        .lda_addr(address_c000)
                        // Will use 0xA5 as opcode as it points to a zeropage address
                        .lda_addr(zeropage_fe)
                        .finalize(),
                )
                .finalize(),
        )
        .finalize();

    let bytes = ProgramGenerator::default().generate(application);
    print_hexdump(&bytes);
}
