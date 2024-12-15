//! scer2, supervisor L2-cache error control register
use bit_field::BitField;

/// scer2 register
#[derive(Clone, Copy, Debug)]
pub struct Scer2 {
    bits: usize,
}

impl Scer2 {
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

read_csr_as!(Scer2, 0x5C2);
