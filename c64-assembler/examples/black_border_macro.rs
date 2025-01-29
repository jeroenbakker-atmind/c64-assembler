use c64_assembler::generator::Generator;
use c64_assembler::generator::{print_hexdump, ProgramGenerator};
use c64_assembler::validator::AssemblerResult;
use c64_assembler_macro::application;

fn main() -> AssemblerResult<()> {
    let application = application!(
        name="Set black border"
        include_vic20_defines
        module!(
            name="main"
            instructions!(
            include_basic_header
            main_entry_point:
                "Load black color into accumulator"
                lda #$00
                sta VIC20_BORDER_COLOR
                rts
            )
        )
    )?;

    let bytes = ProgramGenerator::default().generate(application)?;
    print_hexdump(&bytes);
    Ok(())
}
