use c64_assembler::generator::program::ProgramGenerator;
use c64_assembler::generator::Generator;
use c64_assembler_macro::application;
use cbm::disk::directory::FileType;
use cbm::disk::file::{FileOps, Scheme};
use cbm::disk::{Disk, Id, D64};
use cbm::Petscii;

fn main() {
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
    );

    // Compile to program.
    let program = ProgramGenerator::default().generate(application);

    // Create a disk in memory.
    let geometry = D64::geometry(false);
    let mut disk = D64::open_memory(geometry).unwrap();
    disk.write_format(&Petscii::from_str("DISK 1"), &Id::from_bytes(b"FYR"))
        .unwrap();

    // Write program to disk.
    let d64_file = disk
        .create_file(&Petscii::from_str("set black border"), FileType::PRG, Scheme::Linear)
        .unwrap();
    d64_file.writer().unwrap().write(&program).unwrap();

    // List file entries on disk.
    disk.iter().flatten().for_each(|entry| {
        println!("{:<4} {:<16}", entry.file_size, entry.filename);
    });
}
