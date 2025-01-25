use c64_assembler::generator::{ProgramGenerator, Generator};
use c64_assembler_macro::application;
use mos6502::{
    cpu::CPU,
    instruction::Nmos6502,
    memory::{Bus, Memory},
};

#[test]
fn set_black_border() {
    let application = application!(
        name="Set black border"
        include_vic20_defines
        module!(
            name="main"
            instructions!(
            main_entry_point:
                "Load black color into accumulator"
                lda #$00
                sta VIC20_BORDER_COLOR
                rts
            )
        )
    );

    let bytes = ProgramGenerator::default().generate(application);

    // Emulate the program on 6502 CPU
    let mut cpu = CPU::new(Memory::new(), Nmos6502);

    cpu.memory.set_bytes(0x0800, &bytes[2..]);
    cpu.memory.set_byte(0xd020, 0xFF);
    cpu.registers.program_counter = 0x0800;

    cpu.single_step();
    assert_eq!(0xFF, cpu.memory.get_byte(0xd020));
    cpu.single_step();
    assert_eq!(0x00, cpu.memory.get_byte(0xd020));
    cpu.single_step();
}
