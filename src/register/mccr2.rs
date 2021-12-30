//! mccr2, machine L2-cache control register
use bit_field::BitField;
use core::arch::asm;

/// mccr2 register
#[derive(Clone, Copy, Debug)]
pub struct Mccr2 {
    bits: usize,
}

/// L2-cache data ram visit latency
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DLTNCY {
    Cycle1 = 0,
    Cycle2 = 1,
    Cycle3 = 2,
    Cycle4 = 3,
    Cycle5 = 4,
    Cycle6 = 5,
    Cycle7 = 6,
    Cycle8 = 7,
}

/// L2-cache tag ram visit latency
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TLTNCY {
    Cycle1 = 0,
    Cycle2 = 1,
    Cycle3 = 2,
    Cycle4 = 3,
    Cycle5 = 4,
}

/// L2-cache instruction prefetch
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IPRF {
    Disabled = 0,
    Prefetch1Line = 1,
    Prefetch2Lines = 2,
    Prefetch3Lines = 3,
}

impl Mccr2 {
    /// Refill enable
    #[inline]
    pub fn rfe(&self) -> bool {
        self.bits.get_bit(0)
    }
    /// Error correction enable
    #[inline]
    pub fn eccen(&self) -> bool {
        self.bits.get_bit(1)
    }
    /// L2-cache enable
    #[inline]
    pub fn l2en(&self) -> bool {
        self.bits.get_bit(3)
    }
    /// L2-cache data ram visit latency configuration
    #[inline]
    pub fn dltncy(&self) -> DLTNCY {
        match self.bits.get_bits(16..=18) {
            0 => DLTNCY::Cycle1,
            1 => DLTNCY::Cycle2,
            2 => DLTNCY::Cycle3,
            3 => DLTNCY::Cycle4,
            4 => DLTNCY::Cycle5,
            5 => DLTNCY::Cycle6,
            6 => DLTNCY::Cycle7,
            7 => DLTNCY::Cycle8,
            _ => unreachable!(),
        }
    }
    /// L2-cache data ram setup latency enable
    #[inline]
    pub fn dsetup(&self) -> bool {
        self.bits.get_bit(19)
    }
    /// L2-cache tag ram visit latency configuration
    #[inline]
    pub fn tltncy(&self) -> TLTNCY {
        match self.bits.get_bits(22..=24) {
            0 => TLTNCY::Cycle1,
            1 => TLTNCY::Cycle2,
            2 => TLTNCY::Cycle3,
            3 => TLTNCY::Cycle4,
            4 => TLTNCY::Cycle5,
            _ => unreachable!(),
        }
    }
    /// L2-cache tag ram setup latency enable
    #[inline]
    pub fn tsetup(&self) -> bool {
        self.bits.get_bit(25)
    }
    /// L2-cache instruction prefetch enable
    #[inline]
    pub fn iprf(&self) -> IPRF {
        match self.bits.get_bits(29..=30) {
            0 => IPRF::Disabled,
            1 => IPRF::Prefetch1Line,
            2 => IPRF::Prefetch2Lines,
            3 => IPRF::Prefetch3Lines,
            _ => unreachable!(),
        }
    }
    /// L2-cache TLB prefetch enable
    #[inline]
    pub fn tprf(&self) -> bool {
        self.bits.get_bit(31)
    }
}

read_csr_as!(Mccr2, 0x7C3);

set_clear_csr! {
    /// Refill enable
    , 0x7C3, set_rfe, clear_rfe, 1 << 0
}
set_clear_csr! {
    /// Error correction enable
    , 0x7C3, set_eccen, clear_eccen, 1 << 1
}
set_clear_csr! {
    /// L2-cache data ram setup latency enable
    , 0x7C3, set_dsetup, clear_dsetup, 1 << 19
}
set_clear_csr! {
    /// L2-cache tag ram setup latency enable
    , 0x7C3, set_tsetup, clear_tsetup, 1 << 25
}
set_clear_csr! {
    /// L2-cache TLB prefetch enable
    , 0x7C3, set_tprf, clear_tprf, 1 << 31
}

/// L2-cache data ram visit latency configuration
#[inline]
pub unsafe fn set_dltncy(dltncy: DLTNCY) {
    let mut value: usize;
    asm!("csrr {}, 0x7C3", out(reg) value);
    value.set_bits(16..=18, dltncy as usize);
    asm!("csrw 0x7C3, {}", in(reg) value);
}

/// L2-cache tag ram visit latency configuration
#[inline]
pub unsafe fn set_tltncy(tltncy: TLTNCY) {
    let mut value: usize;
    asm!("csrr {}, 0x7C3", out(reg) value);
    value.set_bits(22..=24, tltncy as usize);
    asm!("csrw 0x7C3, {}", in(reg) value);
}

/// L2-cache instruction prefetch enable
#[inline]
pub unsafe fn set_iprf(iprf: IPRF) {
    let mut value: usize;
    asm!("csrr {}, 0x7C3", out(reg) value);
    value.set_bits(29..=30, iprf as usize);
    asm!("csrw 0x7C3, {}", in(reg) value);
}
