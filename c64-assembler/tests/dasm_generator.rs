use c64_assembler::{
    builder::{ApplicationBuilder, InstructionBuilder, ModuleBuilder},
    generator::{DasmGenerator, Generator},
    validator::AssemblerResult,
};

/// X address indexing was written as absolute load.
#[test]
fn dasm_v0_1_0_missing_x_indexing() -> AssemblerResult<()> {
    let application = ApplicationBuilder::default()
        .module(
            ModuleBuilder::default()
                .instructions(
                    InstructionBuilder::default()
                        .lda_addr_x("label_a")
                        .label("label_a")
                        .build(),
                )
                .build(),
        )
        .build()?;
    let dasm_source = DasmGenerator::default().generate(application)?;
    assert!(dasm_source.contains("lda label_a,x"));
    Ok(())
}

/// Y address indexing was written as absolute load.
#[test]
fn dasm_v0_1_0_missing_y_indexing() -> AssemblerResult<()> {
    let application = ApplicationBuilder::default()
        .module(
            ModuleBuilder::default()
                .instructions(
                    InstructionBuilder::default()
                        .lda_addr_y("label_a")
                        .label("label_a")
                        .build(),
                )
                .build(),
        )
        .build()?;
    let dasm_source = DasmGenerator::default().generate(application)?;
    assert!(dasm_source.contains("lda label_a,y"));
    Ok(())
}
