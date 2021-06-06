//! mexstatus, machine exception status register 

use bit_field::BitField;

/// mxstatus register
#[derive(Clone, Copy, Debug)]
pub struct Mexstatus {
    bits: usize,
}

/// Software reset mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RSTMD {
    /// No operation required
    Nop = 0,
    /// Reset core only
    ResetCore = 1,
    /// Reset whole system
    ResetSystem = 2,
    // 3: reserved
}

/// Low power mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LPMD {
    /// Deep sleep for next WFI
    DeepSleep = 0,
    /// Light sleep for next WFI
    LightSleep = 1,
}

impl Mexstatus {
    /// Software reset mode
    #[inline]
    pub fn rstmd(&self) -> RSTMD {
        match self.bits.get_bits(0..=1) {
            0b00 => RSTMD::Nop,
            0b01 => RSTMD::ResetCore,
            0b10 => RSTMD::ResetSystem,
            _ => unreachable!()
        }
    }
    /// Low power mode
    #[inline]
    pub fn lpmd(&self) -> LPMD {
        match self.bits.get_bits(2..=3) {
            0b00 => LPMD::DeepSleep,
            0b01 => LPMD::LightSleep,
            _ => unreachable!()
        }
    }
    /// Wait for event mode enble
    #[inline]
    pub fn wfeen(&self) -> bool {
        self.bits.get_bit(4)
    }
    /// Exception state
    #[inline]
    pub fn expt(&self) -> bool {
        self.bits.get_bit(5)
    }
    /// Lockup state
    #[inline]
    pub fn lockup(&self) -> bool {
        self.bits.get_bit(6)
    }
    /// NMI state
    #[inline]
    pub fn nmi(&self) -> bool {
        self.bits.get_bit(7)
    }
    /// Bus error state
    #[inline]
    pub fn buserr(&self) -> bool {
        self.bits.get_bit(8)
    }
    /// Interrupt auto push stack enable
    #[inline]
    pub fn spushen(&self) -> bool {
        self.bits.get_bit(16)
    }
    /// Interrupt auto swap stack enable
    #[inline]
    pub fn spswapen(&self) -> bool {
        self.bits.get_bit(17)
    }
}

set!(0x7E1);
clear!(0x7E1);
read_csr_as!(Mexstatus, 0x7E1);

set_clear_csr! {
    /// Wait for event mode enble
    , set_wfeen, clear_wfeen, 1 << 4
}
set_clear_csr! {
    /// Interrupt auto push stack enable
    , set_spushen, clear_spushen, 1 << 16
}
set_clear_csr! {
    /// Interrupt auto swap stack enable
    , set_spswapen, clear_spswapen, 1 << 17
}

/// Set software reset mode
#[inline]
pub unsafe fn set_rstmd(value: RSTMD) {
    _set(value as usize)
}

/// Set low power mode
#[inline]
pub unsafe fn set_lpmd(value: LPMD) {
    _set((value as usize) << 2)
}
