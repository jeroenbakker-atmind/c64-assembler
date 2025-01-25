use crate::Application;

mod dasm;
mod program;

pub trait Generator {
    type Output;
    fn generate(self, application: Application) -> Self::Output;
}

pub use dasm::*;
pub use program::*;
