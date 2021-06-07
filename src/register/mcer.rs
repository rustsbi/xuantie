//! mcer, machine L1-cache error control register

use bit_field::BitField;

/// mcer register
#[derive(Clone, Copy, Debug)]
pub struct Mcer {
    bits: usize,
}

/// Error controllable RAM index
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RAMID {
    /// Instruction cache tag ram
    ICacheTag = 0,
    /// Instruction cache data ram
    ICacheData = 1,
    /// Data cache tag ram
    DCacheTag = 2,
    /// Data cache data ram
    DCacheData = 3,
    /// jTLB tag ram
    JTlbTag = 4,
    /// jTLB data ram
    JTlbData = 5,
}

impl Mcer {
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
    pub fn ramid(&self) -> RAMID {
        match self.bits.get_bits(21..=23) {
            0 => RAMID::ICacheTag,
            1 => RAMID::ICacheData,
            2 => RAMID::DCacheTag,
            3 => RAMID::DCacheData,
            4 => RAMID::JTlbTag,
            5 => RAMID::JTlbData,
            _ => unreachable!()
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

set!(0x7C8);
clear!(0x7C8);
read_csr_as!(Mcer, 0x7C8);

clear_csr! {
    /// Clear error correction fatal error bit
    , clear_err_fatal, 1 << 30
}
clear_csr! {
    /// Clear error correction information valid bit
    , clear_err_vld, 1 << 31
}
