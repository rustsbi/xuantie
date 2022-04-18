//! mhcr, machine hardware configuration register
//!
//! # Platform support
//!
//! This register is supported on Xuantie C910, C906 and E902 cores.
use bit_field::BitField;
use core::arch::asm;

/// mhcr register
#[derive(Clone, Copy, Debug)]
pub struct Mhcr {
    bits: usize,
}

impl Mhcr {
    /// I-cache enable
    ///
    /// # Platform support
    ///
    /// This bit is supported on Xuantie C910, C906 and E902 cores.
    #[inline]
    pub fn ie(&self) -> bool {
        self.bits.get_bit(0)
    }
    /// D-cache enable
    #[inline]
    pub fn de(&self) -> bool {
        self.bits.get_bit(1)
    }
    /// Cache write allocate configuration enable
    #[inline]
    pub fn wa(&self) -> bool {
        self.bits.get_bit(2)
    }
    /// Write back enable; true for write back, false for write through
    #[inline]
    pub fn wb(&self) -> bool {
        self.bits.get_bit(3)
    }
    /// Return stack enable
    #[inline]
    pub fn rs(&self) -> bool {
        self.bits.get_bit(4)
    }
    /// Branch predict enable
    #[inline]
    pub fn bpe(&self) -> bool {
        self.bits.get_bit(5)
    }
    /// Branch target buffer enable
    #[inline]
    pub fn btb(&self) -> bool {
        self.bits.get_bit(6)
    }
    /// Write bulk transfer enable
    #[inline]
    pub fn wbr(&self) -> bool {
        self.bits.get_bit(8)
    }
}

read_csr_as!(Mhcr, 0x7C1);

set_clear_csr! {
    /// I-cache enable
    , 0x7C1, set_ie, clear_ie, 1 << 0
}
set_clear_csr! {
    /// D-cache enable
    , 0x7C1, set_de, clear_de, 1 << 1
}
set_clear_csr! {
    /// Cache write allocate configuration enable
    , 0x7C1, set_wa, clear_wa, 1 << 2
}
set_clear_csr! {
    /// Write back enable; clear this bit to be write through
    , 0x7C1, set_wb, clear_wb, 1 << 3
}
set_clear_csr! {
    /// Return stack enable
    , 0x7C1, set_rs, clear_rs, 1 << 4
}
set_clear_csr! {
    /// Branch predict enable
    , 0x7C1, set_bpe, clear_bpe, 1 << 5
}
set_clear_csr! {
    /// Branch target buffer enable
    , 0x7C1, set_btb, clear_btb, 1 << 6
}
set_clear_csr! {
    /// Write bulk transfer enable
    , 0x7C1, set_wbr, clear_wbr, 1 << 8
}
