//! Validate the consistency of an application.
//!
//! ```
//! use c64_assembler_macro::application;
//! use c64_assembler::validator::Validator;
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
//! assert!(application.validate().is_ok());
//! ```
use address_names_exists::validate_address_names_exists;
use address_names_unique::validate_address_names_unique;

use crate::Application;

mod address_names_exists;
mod address_names_unique;
mod relative_addressing;

pub trait Validator {
    fn validate(&self) -> ValidatorResult<()>;
}

pub type ValidatorResult<T> = Result<T, Error>;

pub enum Error {
    AddressNameUnknown(String),
    AddressNameNotUnique(String),
}

impl Validator for Application {
    fn validate(&self) -> ValidatorResult<()> {
        validate_address_names_exists(self)?;
        validate_address_names_unique(self)?;
        Ok(())
    }
}
