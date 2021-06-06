//! mxstatus, machine extended status register

use bit_field::BitField;

/// mxstatus register
#[derive(Clone, Copy, Debug)]
pub struct Mxstatus {
    bits: usize,
}

/// Current privileged mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PM {
    Machine = 3,
    Supervisor = 1,
    User = 0,
}

impl Mxstatus {
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

set!(0x7C0);
clear!(0x7C0);
read_csr_as!(Mxstatus, 0x7C0);

set_clear_csr! {
    /// User mode performance monitor enable
    , set_pmdu, clear_pmdu, 1 << 10
}
set_clear_csr! {
    /// Supervisor mode performance monitor enable
    , set_pmds, clear_pmds, 1 << 11
}
set_clear_csr! {
    /// Machine mode performance monitor enable
    , set_pmdm, clear_pmdm, 1 << 13
}
set_clear_csr! {
    /// Unaligned access enable
    , set_mm, clear_mm, 1 << 15
}
set_clear_csr! {
    /// User mode allow extended cache instruction
    , set_ucme, clear_ucme, 1 << 16
}
set_clear_csr! {
    /// CLINT supervisor extension enable
    , set_clintee, clear_clintee, 1 << 17
}
set_clear_csr! {
    /// Hardware refill when TLB item absent enable
    , set_mhrd, clear_mhrd, 1 << 18
}
set_clear_csr! {
    /// Extend MMU page table entry address attributes enable
    , set_maee, clear_maee, 1 << 21
}
set_clear_csr! {
    /// T-Head extended instruction set architecture enable
    , set_theadisaee, clear_theadisaee, 1 << 22
}
