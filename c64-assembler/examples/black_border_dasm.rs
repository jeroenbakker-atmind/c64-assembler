use c64_assembler::builder::ApplicationBuilder;
use c64_assembler::builder::InstructionBuilder;
use c64_assembler::builder::ModuleBuilder;
use c64_assembler::generator::DasmGenerator;
use c64_assembler::generator::Generator;
use c64_assembler::validator::AssemblerResult;

fn main() -> AssemblerResult<()> {
    let application = ApplicationBuilder::default()
        .name("Set black border")
        .include_vic2_defines()
        .module(
            ModuleBuilder::default()
                .name("main")
                .instructions(
                    InstructionBuilder::default()
                        .add_basic_header()
                        .label("main_entry_point")
                        .lda_imm(0x00)
                        .comment("Load black color")
                        .sta_addr("VIC2_BORDER_COLOR")
                        .rts()
                        .build(),
                )
                .build(),
        )
        .build()?;

    let source = DasmGenerator::default().generate(application)?;
    println!("{source}");
    Ok(())
}
