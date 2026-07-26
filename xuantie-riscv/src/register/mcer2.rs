//! mcer2, machine L2-cache error control register
//!
//! This is a 64-bit register supported on Xuantie C920 and C910.
use bit_field::BitField;

/// mcer2 register
#[derive(Clone, Copy, Debug)]
pub struct Mcer2 {
    bits: usize,
}

#[cfg(target_pointer_width = "64")]
impl Mcer2 {
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
read_csr_as!(Mcer2, 0x7C4);

clear_csr! {
    #[cfg(target_pointer_width = "64")]
    /// Clear the error information valid bit
    , 0x7C4, clear_err_vld, 1usize << 63
}

#[cfg(all(test, target_pointer_width = "64"))]
mod tests {
    use super::Mcer2;

    #[test]
    fn decodes_c910_c920_layout() {
        let mcer2 = Mcer2 {
            bits: ((0x5555u64 << 3)
                | (0xBu64 << 21)
                | (0x2u64 << 52)
                | (0x12u64 << 55)
                | (1u64 << 62)
                | (1u64 << 63)) as usize,
        };

        assert_eq!(mcer2.err_index(), 0x5555);
        assert_eq!(mcer2.err_way(), 0xB);
        assert_eq!(mcer2.ramid(), 0x2);
        assert_eq!(mcer2.fix_cnt(), 0x12);
        assert!(mcer2.ecc_fatal());
        assert!(mcer2.err_vld());
    }
}
