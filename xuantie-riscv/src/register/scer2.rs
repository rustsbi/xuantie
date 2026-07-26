//! scer2, supervisor L2-cache error control register
//!
//! This 64-bit register maps the fields of [`super::mcer2`] into supervisor mode.
use bit_field::BitField;

/// scer2 register
#[derive(Clone, Copy, Debug)]
pub struct Scer2 {
    bits: usize,
}

#[cfg(target_pointer_width = "64")]
impl Scer2 {
    /// Error index
    #[inline]
    pub fn err_index(&self) -> u32 {
        self.bits.get_bits(3..=20) as u32
    }

    /// Error way
    #[inline]
    pub fn err_way(&self) -> u8 {
        self.bits.get_bits(21..=24) as u8
    }

    /// SRAM ID where the ECC error occurred
    #[inline]
    pub fn ramid(&self) -> u8 {
        self.bits.get_bits(52..=53) as u8
    }

    /// Number of corrected ECC errors
    #[inline]
    pub fn fix_cnt(&self) -> u8 {
        self.bits.get_bits(55..=59) as u8
    }

    /// L2-cache fatal two-bit ECC error
    #[inline]
    pub fn ecc_fatal(&self) -> bool {
        self.bits.get_bit(62)
    }

    /// L2-cache ECC or parity error information is valid
    #[inline]
    pub fn err_vld(&self) -> bool {
        self.bits.get_bit(63)
    }
}

#[cfg(target_pointer_width = "64")]
read_csr_as!(Scer2, 0x5C2);

#[cfg(all(test, target_pointer_width = "64"))]
mod tests {
    use super::Scer2;

    #[test]
    fn matches_mcer2_field_mapping() {
        let scer2 = Scer2 {
            bits: ((0x1_2345u64 << 3)
                | (0x7u64 << 21)
                | (0x1u64 << 52)
                | (0x1Fu64 << 55)
                | (1u64 << 62)
                | (1u64 << 63)) as usize,
        };

        assert_eq!(scer2.err_index(), 0x1_2345);
        assert_eq!(scer2.err_way(), 0x7);
        assert_eq!(scer2.ramid(), 0x1);
        assert_eq!(scer2.fix_cnt(), 0x1F);
        assert!(scer2.ecc_fatal());
        assert!(scer2.err_vld());
    }
}
