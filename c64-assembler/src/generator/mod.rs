//! Generators to export to a .PRG or source code.
use crate::Application;

mod dasm;
mod program;

/// Generate an output for a given application.
pub trait Generator {
    type Output;

    /// Generate an output for the given application.
    fn generate(self, application: Application) -> Self::Output;
}

pub use dasm::*;
pub use program::*;
