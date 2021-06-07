//! mcer2, machine L2-cache error control register

use bit_field::BitField;

/// mcer2 register
#[derive(Clone, Copy, Debug)]
pub struct Mcer2 {
    bits: usize,
}

impl Mcer2 {
    /// Error index
    #[inline]
    pub fn err_index(&self) -> u16 {
        self.bits.get_bits(0..=15) as u16
    }
    /// Error way
    #[inline]
    pub fn err_way(&self) -> u8 {
        self.bits.get_bits(17..=20) as u8
    }
    /// L2-cache 2 bit or parity error also happened elsewhere other than current position
    #[inline]
    pub fn oth_err(&self) -> bool {
        self.bits.get_bit(30)
    }
    /// L2-cache 2 bit error correction error or parity check error
    #[inline]
    pub fn ecc_err(&self) -> bool {
        self.bits.get_bit(31)
    }
}

set!(0x7C4);
clear!(0x7C4);
read_csr_as!(Mcer2, 0x7C4);

clear_csr! {
    /// Clear error correction information valid bit
    , clear_ecc_err, 1 << 31
}
