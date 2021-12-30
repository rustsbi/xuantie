//! smel, supervisor memory entry low register
use crate::paging::Entry;
use core::arch::asm;

/// Reads the smel register
#[inline]
pub fn read() -> Entry {
    let bits: usize;
    unsafe { asm!("csrr {}, 0x9C1", out(reg) bits) }
    Entry::from_bits(bits)
}
