use c64_assembler::{
    builder::{ApplicationBuilder, InstructionBuilder, ModuleBuilder},
    generator::{Generator, ProgramGenerator},
    validator::{AssemblerResult, Validator},
};

/// The addr_y instructions were added twice to the result when using the program generation.
/// When using zero page the zero page was followed up with an absolute y-indexing.
#[test]
fn program_v0_1_0_addr_y_instructions() -> AssemblerResult<()> {
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
    application.validate()?;
    assert_eq!(0x0803, application.lookup_address(&"label_a".to_string())?);
    let program = ProgramGenerator::default().generate(application)?;
    assert_eq!([0x00, 0x08, 0xB9, 0x03, 0x08], program.as_slice());

    Ok(())
}

/// The zeropage_y instructions were followed with an addr_y.
#[test]
fn program_v0_1_0_zeropage_y_instructions() -> AssemblerResult<()> {
    let application = ApplicationBuilder::default()
        .define_address("ZEROPAGE_ADDRESS", 0xFE)
        .module(
            ModuleBuilder::default()
                .instructions(
                    InstructionBuilder::default()
                        .ldx_addr_y("ZEROPAGE_ADDRESS")
                        .label("label_a")
                        .build(),
                )
                .build(),
        )
        .build()?;
    application.validate()?;
    assert_eq!(0x0802, application.lookup_address(&"label_a".to_string())?);
    let program = ProgramGenerator::default().generate(application)?;
    assert_eq!([0x00, 0x08, 0xB6, 0xFE], program.as_slice());

    Ok(())
}
