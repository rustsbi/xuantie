//! mrvbr, machine reset vector base address register
use core::arch::asm;

/// Get reset vector base address
#[inline]
pub fn get_rvbr() -> usize {
    let ans: usize;
    unsafe { asm!("csrr {}, 0x7C7", out(reg) ans) };
    ans
}
