//! mcor, machine cache operation register

set!(0x7C2);
clear!(0x7C2);

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
    _set(bits)
}

/// Invalidate branch history table
#[inline]
pub unsafe fn bht_inv() {
    _set(1 << 16)
}

/// Invalidate branch target predict table
#[inline]
pub unsafe fn btb_inv() {
    _set(1 << 17)
}
