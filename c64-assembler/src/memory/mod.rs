pub mod address_mode;
pub mod define;
pub mod label;
pub mod user_count;

/// Memory address
pub type Address = u16;

/// Trait for instructions that can handle zeropage and regular addresses.
///
/// Zeropage are special address mode on the 6502 where less CPU-cycles and
/// instruction size is used when accessing RAM below 0x0100.
///
/// Based on the is_zeropage the caller can decide if it only needs the low byte or
/// both the low and high byte.
pub trait ZeroPage {
    /// Is the address a zeropage address
    fn is_zeropage(&self) -> bool;

    /// Get the lower byte of the address
    fn low(&self) -> u8;

    /// Get the higher byte of the address
    fn high(&self) -> u8;
}

impl ZeroPage for Address {
    /// Is the address a zeropage address
    ///
    /// ```
    /// use c64_assembler::memory::ZeroPage;
    ///
    /// assert_eq!(true, 0x00FE.is_zeropage());
    /// assert_eq!(true, 0x00FF.is_zeropage());
    /// assert_eq!(false, 0x0100.is_zeropage());
    /// assert_eq!(false, 0x0101.is_zeropage());
    /// ```
    fn is_zeropage(&self) -> bool {
        *self < 0x100
    }

    /// Get the lower byte of the address
    ///
    /// ```
    /// use c64_assembler::memory::ZeroPage;
    ///
    /// assert_eq!(0xFE, 0x00FE.low());
    /// assert_eq!(0xFF, 0x00FF.low());
    /// assert_eq!(0x00, 0x0100.low());
    /// assert_eq!(0x01, 0x0101.low());
    /// ```
    fn low(&self) -> u8 {
        (self & 0xFF) as u8
    }

    /// Get the higher byte of the address
    ///
    /// ```
    /// use c64_assembler::memory::ZeroPage;
    ///
    /// assert_eq!(0x00, 0x00FE.high());
    /// assert_eq!(0x00, 0x00FF.high());
    /// assert_eq!(0x01, 0x0100.high());
    /// assert_eq!(0x01, 0x0101.high());
    /// ```
    fn high(&self) -> u8 {
        (self >> 8) as u8
    }
}
