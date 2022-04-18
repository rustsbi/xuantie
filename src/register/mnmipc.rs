//! mnmipc, machine NMI exception program counter
//!
//! # Platform support
//!
//! This register is supported on Xuantie E907, E906 and E902 cores.
use core::arch::asm;

/// Get NMI exception program counter
#[inline]
pub fn read() -> usize {
    let ans: usize;
    unsafe { asm!("csrr {}, 0x7E3", out(reg) ans) }
    ans
}
