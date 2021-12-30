//! mrmr, machine reset signal register
use core::arch::asm;

/// Write to mrmr register to release reset lock for given harts
///
/// Write `0x1` for hart 0, `0x2` for 1, `0x4` for 2, etc.
#[inline]
pub unsafe fn write(bits: usize) {
    asm!("csrs 0x7C6, {}", in(reg) bits);
}
