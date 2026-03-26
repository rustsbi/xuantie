//! mrmr, machine reset signal register.
//!
//! # Note
//! 
//! The `mrmr` register has been removed in C920V3 (R1S4 and later),
//! and its corresponding functionality no longer exists.
//! Software can still access this register, but reads return zero and
//! writes have no effect, without triggering an exception.
use core::arch::asm;

/// Write to mrmr register to release reset lock for given harts
///
/// Write `0x1` for hart 0, `0x2` for 1, `0x4` for 2, etc.
#[inline]
pub unsafe fn write(bits: usize) {
    asm!("csrs 0x7C6, {}", in(reg) bits);
}
