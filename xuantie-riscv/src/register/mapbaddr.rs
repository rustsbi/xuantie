//! mapbaddr, machine APB base address register
use core::arch::asm;

/// Get APB peripheral base address
#[inline]
pub fn read() -> usize {
    let ans: usize;
    unsafe { asm!("csrr {}, 0xFC1", out(reg) ans) };
    ans
}
