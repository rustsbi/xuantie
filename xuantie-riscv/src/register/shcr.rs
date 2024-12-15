//! shcr, supervisor hardware configuration register
use bit_field::BitField;

/// shcr register
#[derive(Clone, Copy, Debug)]
pub struct Shcr {
    bits: usize,
}

impl Shcr {
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

read_csr_as!(Shcr, 0x5C1);
