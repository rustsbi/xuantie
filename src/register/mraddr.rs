//! mraddr, machine reset address register
//!
//! # Platform support
//!
//! This register is supported on Xuantie E906 and E902 cores.
use core::arch::asm;

/// Get machine reset address
#[inline]
pub fn read() -> usize {
    let ans: usize;
    unsafe { asm!("csrr {}, 0x7E0", out(reg) ans) }
    ans
}
