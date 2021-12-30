//! sxstatus, supervisor extended state register

pub use super::mxstatus::PM;
use bit_field::BitField;

/// sxstatus register
#[derive(Clone, Copy, Debug)]
pub struct Sxstatus {
    bits: usize,
}

impl Sxstatus {
    /// User mode performance monitor enable
    #[inline]
    pub fn pmdu(&self) -> bool {
        self.bits.get_bit(10)
    }
    /// Supervisor mode performance monitor enable
    #[inline]
    pub fn pmds(&self) -> bool {
        self.bits.get_bit(11)
    }
    /// Machine mode performance monitor enable
    #[inline]
    pub fn pmdm(&self) -> bool {
        self.bits.get_bit(12)
    }
    /// Is PMP minimum stride 4K bytes
    #[inline]
    pub fn pmp4k(&self) -> bool {
        self.bits.get_bit(14)
    }
    /// Unaligned access enable
    #[inline]
    pub fn mm(&self) -> bool {
        self.bits.get_bit(15)
    }
    /// User mode allow extended cache instruction
    #[inline]
    pub fn ucme(&self) -> bool {
        self.bits.get_bit(16)
    }
    /// CLINT supervisor extension enable
    #[inline]
    pub fn clintee(&self) -> bool {
        self.bits.get_bit(17)
    }
    /// Hardware refill when TLB item absent enable
    #[inline]
    pub fn mhrd(&self) -> bool {
        self.bits.get_bit(18)
    }
    /// Extend MMU page table entry address attributes enable
    #[inline]
    pub fn maee(&self) -> bool {
        self.bits.get_bit(21)
    }
    /// T-Head extended instruction set architecture enable
    #[inline]
    pub fn theadisaee(&self) -> bool {
        self.bits.get_bit(22)
    }
    /// Current privileged mode
    #[inline]
    pub fn pm(&self) -> PM {
        match self.bits.get_bits(30..=31) {
            0b00 => PM::User,
            0b01 => PM::Supervisor,
            0b11 => PM::Machine,
            _ => unreachable!(),
        }
    }
}

set!(0x5C0);
clear!(0x5C0);
read_csr_as!(Sxstatus, 0x5C0);

set_clear_csr! {
    /// Unaligned access enable
    , set_mm, clear_mm, 1 << 15
}
