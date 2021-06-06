//! mhcr, machine hardware configuration register
use bit_field::BitField;

/// mchr register
#[derive(Clone, Copy, Debug)]
pub struct Mhcr {
    bits: usize,
}

impl Mhcr {
    /// I-cache enable
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
    /// Branch target predict enable
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

set!(0x7C1);
clear!(0x7C1);
read_csr_as!(Mhcr, 0x7C1);

set_clear_csr! {
    /// I-cache enable
    , set_ie, clear_ie, 1 << 0
}
set_clear_csr! {
    /// D-cache enable
    , set_de, clear_de, 1 << 1
}
set_clear_csr! {
    /// Cache write allocate configuration enable
    , set_wa, clear_wa, 1 << 2
}
set_clear_csr! {
    /// Write back enable; clear this bit to be write through
    , set_wb, clear_wb, 1 << 3
}
set_clear_csr! {
    /// Return stack enable
    , set_rs, clear_rs, 1 << 4
}
set_clear_csr! {
    /// Branch predict enable
    , set_bpe, clear_bpe, 1 << 5
}
set_clear_csr! {
    /// Branch target predict enable
    , set_btb, clear_btb, 1 << 6
}
set_clear_csr! {
    /// Write bulk transfer enable
    , set_wbr, clear_wbr, 1 << 8
}
