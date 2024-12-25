//! smeh, supervisor memory entry high register.
use bit_field::BitField;

/// smeh register.
#[derive(Clone, Copy, Debug)]
pub struct Smeh {
    bits: usize,
}

/// Page size.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PageSize {
    /// 4-KiB page.
    Page4K = 1,
    /// 2-MiB page.
    Page2M = 2,
    /// 1-GiB page.
    Page1G = 4,
}

impl Smeh {
    /// Get address space number.
    #[inline]
    pub fn asid(&self) -> u16 {
        self.bits.get_bits(0..=15) as u16
    }
    /// Get page size.
    #[inline]
    pub fn page_size(&self) -> PageSize {
        match self.bits.get_bits(16..=18) {
            1 => PageSize::Page4K,
            2 => PageSize::Page2M,
            4 => PageSize::Page1G,
            _ => unreachable!(),
        }
    }
    /// Get virtual page number.
    #[inline]
    pub fn vpn(&self) -> usize {
        self.bits.get_bits(19..=45)
    }
}

read_csr_as!(Smeh, 0x9C2);
