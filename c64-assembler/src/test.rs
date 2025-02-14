use crate::{
    builder::{ApplicationBuilder, InstructionBuilder, ModuleBuilder},
    generator::{DasmGenerator, Generator, ProgramGenerator},
    validator::AssemblerResult,
    Application,
};

fn test_application() -> AssemblerResult<Application> {
    ApplicationBuilder::default()
        .name("test build dasm")
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
        .build()
}

#[test]
fn build_dasm() -> AssemblerResult<()> {
    let application = test_application()?;
    let dasm_source = DasmGenerator::default().generate(application)?;
    println!("{dasm_source}");
    Ok(())
}

#[test]
fn build_program() -> AssemblerResult<()> {
    let application = test_application()?;
    let mut address = application.entry_point;
    let program_binary = ProgramGenerator::default().generate(application)?;

    // print program to console.
    program_binary.chunks(16).for_each(|chunk| {
        let mut line = Vec::new();

        line.push(format!("{:04X}: ", address));
        address += 16;

        chunk.chunks(4).for_each(|chunk| {
            chunk.iter().for_each(|byte| {
                line.push(format!("{:02X}", byte));
            });
            line.push("".to_string());
        });
        println!("{}", line.join(" ").trim_end());
    });
    Ok(())
}
