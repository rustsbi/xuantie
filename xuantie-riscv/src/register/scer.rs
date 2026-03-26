//! scer, supervisor L1-cache error control register
pub use super::mcer::RamId;
use bit_field::BitField;

/// scer register
#[derive(Clone, Copy, Debug)]
pub struct Scer {
    bits: usize,
}

impl Scer {
    /// Error index
    #[inline]
    pub fn err_index(&self) -> u16 {
        self.bits.get_bits(0..=15) as u16
    }
    /// Error way
    #[inline]
    pub fn err_way(&self) -> u8 {
        self.bits.get_bits(17..=18) as u8
    }
    /// RAM that the error correction fatal error taken place
    #[inline]
    pub fn ramid(&self) -> RamId {
        match self.bits.get_bits(21..=23) {
            0 => RamId::ICacheTag,
            1 => RamId::ICacheData,
            2 => RamId::DCacheTag,
            3 => RamId::DCacheData,
            4 => RamId::JTlbTag,
            5 => RamId::JTlbData,
            _ => unreachable!(),
        }
    }
    /// Count of errors that are already fixed
    #[inline]
    pub fn fix_cnt(&self) -> u8 {
        self.bits.get_bits(24..=29) as u8
    }
    /// Is L1-cache error correction error a fatal error
    #[inline]
    pub fn err_fatal(&self) -> bool {
        self.bits.get_bit(30)
    }
    /// Error correction information valid
    #[inline]
    pub fn err_vld(&self) -> bool {
        self.bits.get_bit(31)
    }
}

read_csr_as!(Scer, 0x5C3);
