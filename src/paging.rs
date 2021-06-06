//! Paging support

use bit_field::BitField;

/// Xuantie extended page table entry
#[derive(Clone, Copy, Debug)]
pub struct Entry {
    bits: usize,
}

impl Entry {
    /// Set physical page number
    #[inline]
    pub fn set_ppn(&mut self, ppn: usize) {
        assert!(ppn <= (1 << (28 + 1)));
        self.bits |= ppn << 10;
    }
    /// Get physical page number
    #[inline]
    pub fn ppn(&self) -> usize {
        self.bits.get_bits(10..38)
    }
    // todo: set flags
}

bitflags::bitflags! {
    /// Xuantie page table entry flags
    pub struct Flags: usize {
        /// Valid
        const V = 1 << 0;
        /// Read
        const R = 1 << 1;
        /// Write
        const W = 1 << 2;
        /// Execute
        const X = 1 << 3;
        /// User mode
        const U = 1 << 4;
        /// Global
        const G = 1 << 5;
        /// Accessed
        const A = 1 << 6;
        /// Dirty
        const D = 1 << 7;
        /// Secure world trustable
        const T = 1 << 59;
        /// Buffer
        const B = 1 << 61;
        /// Cacheable
        const C = 1 << 62;
        /// Strong order
        const SO = 1 << 63;
    }
}
