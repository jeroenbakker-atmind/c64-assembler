# c64-assembler

A modern assembler for the Commodore 64 that integrates seamlessly with Rust and Cargo workflows. This tool allows you to assemble 6502 assembly code into PRG, making it easy to build and test Commodore 64 programs directly from your Rust projects.

> [!NOTE]
> This project is still in development. You're free to use/adapt/distribute it (as long as you respect the license).

## Features

- Assembles 6502 assembly code for the Commodore 64.
- Integrates with Rust projects via Cargo.
- Assembly code can be build via rust builders or via a rust macro
- Outputs PRG compatible with C64 emulators and real hardware.
- Outputs Dasm compatible source files.

## Installation

You can add `c64-assembler` as a rust dependency into your cargo.toml:

```sh
cargo add c64-assembler
```

## Usage

### Basic Assembly

To assemble C64 assembly.

```rust
use c64_assembler::builder::ApplicationBuilder;
use c64_assembler::builder::module::ModuleBuilder;
use c64_assembler::builder::InstructionBuilder;

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
    .build().unwrap();
```

After this the application can be generated to bytes (`Vec<u8>`) using the `ProgramGenerator`

```rust
use c64_assembler::generator::{Generator, ProgramGenerator, print_hexdump};

let bytes = ProgramGenerator::default().generate(application).unwrap();
print_hexdump(&bytes);
```

```txt
0000:  00 08 00 0C  08 0A 00 9E  20 32 30 36  32 00 00 00
0010:  A9 00 8D 20  D0 60
```

Or generate to dasm source using the `DasmGenerator`

```rust
use c64_assembler::generator::{Generator, DasmGenerator};

let source = DasmGenerator::default().generate(application).unwrap();
println!("{}", source);
```

```asm
; --- Application: SET BLACK BORDER ---
; NOTE: This file is generated, do not modify

  processor 6502

VIC2_BORDER_COLOR = $D020

  org $0800

; --- Module begin: MAIN ---
  byte $00, $0C, $08     ; New basic line
  ; 10 SYS 2062
  byte $0A, $00, $9E, $20, $32, $30, $36, $32
  byte $00, $00, $00     ; End basic program

main_entry_point:
  lda #$00               ; Load black color
  sta VIC2_BORDER_COLOR
  rts
```

### Using macros (work in progress)

> [!NOTE]
> This is still in development and doesn't include all features yet.

The `c64-assembly-macro` crate introduces several macros to reduce the boiler plating.

```rust
use c64_assembler_macro::application;

let application = application!(
    name="Set black border"
    include_vic2_defines
    module!(
        name="main"
        instructions!(
        include_basic_header
        main_entry_point:
            "Load black color into accumulator"
            lda #$00
            sta VIC2_BORDER_COLOR
            rts
        )
    )
).unwrap();
```

## Development & Contribution

We welcome contributions! To get started:

1. Fork the repository.

2. Clone your fork:

   ```sh
   git clone https://github.com/jeroenbakker-atmind/c64-assembler
   ```

3. Make your changes and submit a pull request.

TIP: Create an issue first, for discussing of getting guidance.

## License

This project is licensed under the GPL-3.0-or-later.

## Credits & Acknowledgments

Special thanks to the C64 and Rust communities for inspiration and support.

## Resources & Links

- [6502 Assembly Reference](https://www.masswerk.at/6502/)
- [VICE Emulator](https://vice-emu.sourceforge.io/)
