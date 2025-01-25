//! # C64 Assembler
//!
//! The goal of this crate is to being able to compile C64 assembly directly from Rust.
//!
//! The reasoning behind it is that in a typical C64 development workflow the programs
//! are generated in a separate step and then being saved to a disk image. For generating
//! disk images there are already crates out there like cmb.
//!
//! However some projects require more control over the compilation stage and disk
//! construction stage. Having a C64 assembler written in rust can build a bridge and
//! allows custom disk formats and faster iterations during development.
//!
//! ## Modules and functions
//!
//! An [crate::Application] is organized in [crate::Module] and
//! [crate::Function]. Modules can be shared between applications. A module public API is
//! organized in functions. Multiple variations of functions can exists. By swapping out
//! functions in a module a module can be size-optimized or CPU cycles optimized based on
//! the actual needs of the program.
//!
//! ## Usage
//!
//! ### Building pattern
//!
//! An application can be build using builder patterns.
//!
//! ```
//! use c64_assembler::builder::ApplicationBuilder;
//! use c64_assembler::builder::ModuleBuilder;
//! use c64_assembler::builder::InstructionBuilder;
//!
//! let application = ApplicationBuilder::default()
//!     .name("Set black border")
//!     .include_vic20_defines()
//!     .module(
//!         ModuleBuilder::default()
//!             .name("main")
//!             .instructions(
//!                 InstructionBuilder::default()
//!                     .add_basic_header()
//!                     .label("main_entry_point")
//!                     .lda_imm(0x00)
//!                     .comment("Load black color")
//!                     .sta_addr("VIC20_BORDER_COLOR")
//!                     .rts()
//!                     .finalize(),
//!             )
//!             .finalize(),
//!     )
//!     .finalize();
//! ```
//!
//! ### Using macros (experimental)
//!
//! To reduce the boilerplating macros can be used. This is still under development.
//!
//! ```
//! use c64_assembler_macro::application;
//!
//! let application = application!(
//!     name="Set black border"
//!     include_vic20_defines
//!     module!(
//!         name="main"
//!         instructions!(
//!         include_basic_header
//!         main_entry_point:
//!             "Load black color into accumulator"
//!             lda #$00
//!             sta VIC20_BORDER_COLOR
//!             rts
//!         )
//!     )
//! );
//! ```
//!
//! ### Generating dasm source
//!
//! Using the [crate::generator::dasm::DasmGenerator] a dasm compatible assembly source
//! can be generated.
//!
//! ```
//! use c64_assembler::generator::Generator;
//! use c64_assembler::generator::DasmGenerator;
//! # use c64_assembler::builder::ApplicationBuilder;
//! # let application = ApplicationBuilder::default().finalize();
//!
//! let source = DasmGenerator::default().generate(application);
//! println!("{}", source);
//! ```
//!
//! Would output
//!
//! ```asm
//! ; --- Application: SET BLACK BORDER ---
//! ; NOTE: This file is generated, do not modify
//!
//!   processor 6502
//!
//! VIC20_BORDER_COLOR = $D020
//!
//!   org $0800
//!
//! ; --- Module begin: MAIN ---
//!   byte $00, $0C, $08     ; New basic line
//!   ; 10 SYS 2062
//!   byte $0A, $00, $9E, $20, $32, $30, $36, $32
//!   byte $00, $00, $00     ; End basic program
//!
//! main_entry_point:
//!   lda #$00
//!   sta VIC20_BORDER_COLOR
//!   rts
//! ; --- Module end: MAIN ---
//! ```
//!
//! ### Generating .PRG byte stream
//!
//! Using the [crate::generator::program::ProgramGenerator] to generate the byte stream.
//! The byte stream includes the loading address.
//!
//! ```
//! use c64_assembler::generator::{Generator, ProgramGenerator, print_hexdump};
//! # use c64_assembler::builder::ApplicationBuilder;
//! # let application = ApplicationBuilder::default().finalize();
//!
//! let bytes = ProgramGenerator::default().generate(application);
//! print_hexdump(&bytes);
//! ```
//!
//! ```txt
//! 0000:  00 08 00 0C  08 0A 00 9E  20 32 30 36  32 00 00 00
//! 0010:  A9 00 8D 20  D0 60
//! ```
//!

use std::collections::HashMap;

use instruction::Instruction;
use memory::{define::Define, user_count::UserCount, Address};

pub mod builder;
pub mod generator;
pub mod instruction;
pub mod memory;

#[cfg(test)]
mod test;

/// Application is the root container for the assembler
#[derive(Clone)]
pub struct Application {
    /// Name of the application; only used in comments.
    pub name: String,
    /// Entry point of the application, default = 0x0800
    pub entry_point: Address,
    /// Modules of the application
    pub modules: Vec<Module>,
    /// Defines of the application
    pub defines: Vec<Define>,
    /// Lookup for addresses.
    pub address_lookup: HashMap<String, Address>,
}

/// Module
///
/// A module is reusable part between applications. If you have some frequent used
/// code, you can group it in a module so you can reused it between applications.
///
/// Can be compared with an include statement.
#[derive(Default, Clone)]
pub struct Module {
    /// Name of the module; only used in comments.
    pub name: String,

    /// Module specific utility instructions.
    ///
    /// For sharing code between functions.
    pub instructions: Instructions,

    /// Functions of this module.
    pub functions: Vec<Function>,
}

/// Function is a replaceble public part of a module.
///
/// You can have multiple variations of a function. These functions can be swapped out
/// when building the module.
///
/// # Variations
///
/// Some examples why functions can have variations.
///
/// - Size optimized
/// - Performance optimized
/// - Last known working version
/// - Currently in development
#[derive(Default, Clone)]
pub struct Function {
    /// Name of the function
    pub name: String,

    /// Documentation of the function.
    pub documentation: Vec<String>,

    /// Instructions belonging to this function.
    pub instructions: Instructions,

    user_count: usize,
}

impl UserCount for Function {
    fn user_increase(&mut self) {
        self.user_count += 1;
    }

    fn user_count(&self) -> usize {
        self.user_count
    }
}

/// Stream of instructions.
#[derive(Debug, Default, Clone)]
pub struct Instructions {
    pub instructions: Vec<Instruction>,
}
