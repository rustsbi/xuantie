//! mnmipc, machine NMI exception program counter
use core::arch::asm;

/// Get NMI exception program counter
#[inline]
pub fn read() -> usize {
    let ans: usize;
    unsafe { asm!("csrr {}, 0x7E3", out(reg) ans) }
    ans
}
