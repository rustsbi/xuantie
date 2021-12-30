//! mcor, machine cache operation register
use bit_field::BitField;
use core::arch::asm;

bitflags::bitflags! {
    /// Select cache to operate
    pub struct Cache: usize {
        /// Select instruction cache
        const INSTRUCTION = 1 << 0;
        /// Select data cache
        const DATA = 1 << 1;
        /// Select both instruction and data caches
        const BOTH = (1 << 0) | (1 << 1);
    }
}

bitflags::bitflags! {
    /// Select operation for caches
    pub struct Operation: usize {
        /// Invalidate cache
        const INVALIDATE = 1 << 4;
        /// Clear cache
        const CLEAR = 1 << 5;
    }
}

/// Perform cache operation
#[inline]
pub unsafe fn cache(cache: Cache, op: Operation) {
    let bits = cache.bits() | op.bits();
    let mut value: usize;
    asm!("csrr {}, 0x7C2", out(reg) value);
    value.set_bits(0..=5, bits as usize);
    asm!("csrw 0x7C2, {}", in(reg) value);
}

/// Invalidate branch history table
#[inline]
pub unsafe fn bht_inv() {
    asm!("csrs 0x7C2, {}", in(reg) 1 << 16);
}

/// Invalidate branch target buffer table
#[inline]
pub unsafe fn btb_inv() {
    asm!("csrs 0x7C2, {}", in(reg) 1 << 17);
}
