use c64_assembler::builder::ApplicationBuilder;
use c64_assembler::builder::InstructionBuilder;
use c64_assembler::builder::ModuleBuilder;
use c64_assembler::generator::{print_hexdump, ProgramGenerator};
use c64_assembler::generator::Generator;

fn main() {
    let application = ApplicationBuilder::default()
        .name("Set black border")
        .include_vic20_defines()
        .module(
            ModuleBuilder::default()
                .name("main")
                .instructions(
                    InstructionBuilder::default()
                        .add_basic_header()
                        .label("main_entry_point")
                        .lda_imm(0x00)
                        .comment("Load black color")
                        .sta_addr("VIC20_BORDER_COLOR")
                        .rts()
                        .finalize(),
                )
                .finalize(),
        )
        .finalize();

    let bytes = ProgramGenerator::default().generate(application);
    print_hexdump(&bytes);
}
