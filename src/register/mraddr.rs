//! mraddr, machine reset address register
use core::arch::asm;

/// Get machine reset address
#[inline]
pub fn read() -> usize {
    let ans: usize;
    unsafe { asm!("csrr {}, 0x7E0", out(reg) ans) }
    ans
}
