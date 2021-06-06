//! Paging support

use bit_field::BitField;

/// Xuantie extended page table entry
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Entry {
    bits: usize,
}

impl Entry {
    /// Convert bit representation into page entry, keeping all the bits
    #[inline]
    pub const fn from_bits(bits: usize) -> Entry {
        Entry { bits }
    }
    /// Returns the raw value of the entry
    #[inline]
    pub const fn bits(&self) -> usize {
        self.bits
    }
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
    /// Insert entry flags, setting corresponding bits to one
    #[inline]
    pub fn insert_flags(&mut self, other: Flags) {
        self.bits |= other.bits()
    }
    /// Remove entry flags, setting corresponding bits to zero
    #[inline]
    pub fn remove_flags(&mut self, other: Flags) {
        self.bits &= !other.bits()
    }
    /// Inserts or removes entry flags depending on the passed value
    #[inline]
    pub fn set_flags(&mut self, other: Flags, value: bool) {
        if value {
            self.insert_flags(other);
        } else {
            self.remove_flags(other);
        }
    }
    /// Toggles the entry flags
    pub fn toggle_flags(&mut self, other: Flags) {
        self.bits ^= other.bits()
    }
    /// Get entry flags
    #[inline]
    pub const fn get_flags(&self) -> Flags {
        Flags::from_bits_truncate(self.bits)
    }
}

bitflags::bitflags! {
    /// Xuantie page table entry flags
    pub struct Flags: usize {
        /// Valid
        const VALID = 1 << 0;
        /// Read
        const READABLE = 1 << 1;
        /// Write
        const WRITABLE = 1 << 2;
        /// Execute
        const EXECUTABLE = 1 << 3;
        /// User mode
        const USER = 1 << 4;
        /// Global
        const GLOBAL = 1 << 5;
        /// Accessed
        const ACCESSED = 1 << 6;
        /// Dirty
        const DIRTY = 1 << 7;
        /// Secure world trustable
        const TRUSTABLE = 1 << 59;
        /// Buffer
        const BUFFER = 1 << 61;
        /// Cacheable
        const CACHEABLE = 1 << 62;
        /// Strong order
        const STRONG_ORDER = 1 << 63;
    }
}
