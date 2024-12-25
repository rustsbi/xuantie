//! smel, supervisor memory entry low register.
use crate::paging::Entry;
use core::arch::asm;

/// Reads the smel register.
///
/// The `xuantie-riscv`` crate assumes that the `smel` register is available only on
/// 64-bit Xuantie architectures. Attempting to read the smel register on non-64-bit
/// architectures using this function will result in an `unimplemented!` panic.
#[inline]
pub fn read() -> Entry {
    match () {
        // Only enable smel when 64-bit, as it would contain a page table entry
        #[cfg(target_pointer_width = "64")]
        () => {
            let bits: usize;
            unsafe { asm!("csrr {}, 0x9C1", out(reg) bits) }
            Entry::from_bits(bits as u64)
        }
        #[cfg(not(target_pointer_width = "64"))]
        () => {
            unimplemented!()
        }
    }
}
