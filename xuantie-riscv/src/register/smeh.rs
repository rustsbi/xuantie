//! smeh, supervisor memory entry high register
use bit_field::BitField;
use core::arch::asm;

/// smeh register
#[derive(Clone, Copy, Debug)]
pub struct Smeh {
    bits: usize,
}

/// Page size
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PageSize {
    Page4K = 0,
    Page2M = 1,
    Page1G = 2,
}

impl Smeh {
    /// Get address space number
    #[inline]
    pub fn asid(&self) -> u16 {
        self.bits.get_bits(0..=15) as u16
    }
    /// Get page size
    #[inline]
    pub fn page_size(&self) -> PageSize {
        match self.bits.get_bits(16..=17) {
            0 => PageSize::Page4K,
            1 => PageSize::Page2M,
            2 => PageSize::Page1G,
            _ => unreachable!(),
        }
    }
    /// Get virtual page number
    #[inline]
    pub fn vpn(&self) -> usize {
        self.bits.get_bits(19..=45)
    }
}

read_csr_as!(Smeh, 0x9C2);
