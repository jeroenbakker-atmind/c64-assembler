use c64_assembler::{
    generator::{print_hexdump, Generator, ProgramGenerator},
    validator::AssemblerResult,
};
use c64_assembler_macro::application;

/// Relative jumps are calculated from the start of the next instruction.
#[test]
fn relative_jump() -> AssemblerResult<()> {
    let application = application!(
                name="Relative addressing"
                module!(
                    name="main"
                    instructions!(
          bne jump_label
          rts
    jump_label:
                    )
                )
            )?;

    let bytes = ProgramGenerator::default().generate(application)?;
    print_hexdump(&bytes);
    assert_eq!(&[0x00, 0x08, 0xD0, 0x01, 0x60], bytes.as_slice());
    Ok(())
}
