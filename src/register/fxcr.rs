//! fxcr, user extended float pointer control register

use bit_field::BitField;

/// Floating-point control and status register
#[derive(Clone, Copy, Debug)]
pub struct Fxcr {
    bits: usize,
}

bitflags::bitflags! {
    /// Fxcr flags
    pub struct Flags: usize {
        /// Inexact
        const NX = 1 << 0;
        /// Underflow
        const UF = 1 << 1;
        /// Overflow
        const OF = 1 << 2;
        /// Divide by Zero
        const DZ = 1 << 3;
        /// Invalid Operation
        const NV = 1 << 4;
        /// Float error accumulator bit
        const FE = 1 << 5;
    }
}

/// Rounding Mode
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoundingMode {
    RoundToNearestEven = 0b000,
    RoundTowardsZero = 0b001,
    RoundDown = 0b010,
    RoundUp = 0b011,
    RoundToNearestMaxMagnitude = 0b100,
    Invalid = 0b111,
}

impl Fxcr {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Accrued Exception Flags
    #[inline]
    pub fn flags(&self) -> Flags {
        Flags::from_bits_truncate(self.bits.get_bits(0..=5))
    }

    /// Output QNaN mode
    #[inline]
    pub fn dqnan(&self) -> bool {
        self.bits.get_bit(23)
    }

    /// Rounding Mode
    #[inline]
    pub fn rm(&self) -> RoundingMode {
        match self.bits.get_bits(24..=26) {
            0b000 => RoundingMode::RoundToNearestEven,
            0b001 => RoundingMode::RoundTowardsZero,
            0b010 => RoundingMode::RoundDown,
            0b011 => RoundingMode::RoundUp,
            0b100 => RoundingMode::RoundToNearestMaxMagnitude,
            _ => RoundingMode::Invalid,
        }
    }
}

set!(0x800);
clear!(0x800);
read_csr_as!(Fxcr, 0x800);

set_clear_csr! {
    /// Output QNaN mode
    , set_dqnan, clear_dqnan, 1 << 23
}

/// Insert float point flags, setting corresponding bits to one.
#[inline]
pub unsafe fn insert_flags(other: Flags) {
    _set(other.bits())
}

/// Remove float point flags, setting corresponding bits to zero
#[inline]
pub unsafe fn remove_flags(other: Flags) {
    _clear(other.bits())
}

/// Inserts or removes float point flags depending on the passed value
#[inline]
pub unsafe fn set_flags(other: Flags, value: bool) {
    if value {
        insert_flags(other);
    } else {
        remove_flags(other);
    }
}
