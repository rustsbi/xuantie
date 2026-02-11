//! XuanTie Core Local Interrupt Controller peripheral.

use volatile_register::{RO, RW};

/// Register block for XuanTie Core Local Interrupt Controller (CLIC) peripheral.
#[repr(C, align(4))]
pub struct Clic {
    /// CLIC configuration register (`CLICCFG`).
    #[doc(alias = "CLICCFG")]
    pub cfg: RW<Config>,
    /// CLIC information register (`CLICINFO`).
    #[doc(alias = "CLICINFO")]
    pub info: RO<Info>,
    /// Interrupt threshold register (`MINTTHRESH`).
    #[doc(alias = "MINTTHRESH")]
    pub mint_thresh: RW<MintThresh>,
    _reserved0: [u8; 0xFF4],
    /// Interrupt registers for each interrupt source (256 interrupts).
    pub interrupts: [Interrupt; 256],
}

/// Interrupt registers for a single interrupt source.
#[repr(C)]
pub struct Interrupt {
    /// Interrupt pending register (`CLICINTIP`).
    #[doc(alias = "CLICINTIP")]
    pub int_ip: RW<Pending>,
    /// Interrupt enable register (`CLICINTIE`).
    #[doc(alias = "CLICINTIE")]
    pub int_ie: RW<Enable>,
    /// Interrupt attribute register (`CLICINTATTR`).
    #[doc(alias = "CLICINTATTR")]
    pub int_attr: RW<Attribute>,
    /// Interrupt control register (`CLICINTCTL`).
    #[doc(alias = "CLICINTCTL")]
    pub int_ctl: RW<u8>,
}

/// CLIC configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Config(u8);

impl Config {
    const NMBITS: u8 = 0x3 << 5;
    const NLBITS: u8 = 0xF << 1;
    const NVBITS: u8 = 0x1;

    /// Set nmbits (`NMBITS`).
    #[doc(alias = "NMBITS")]
    #[inline]
    pub const fn set_nmbits(self, val: u8) -> Self {
        assert!(val < 0x4, "NMBITS value out of range (expected 0..=0x3)");
        Self((self.0 & !Self::NMBITS) | (Self::NMBITS & (val << 5)))
    }
    /// Get nmbits.
    #[inline]
    pub const fn nmbits(self) -> u8 {
        (self.0 & Self::NMBITS) >> 5
    }
    /// Set nlbits (`NLBITS`).
    #[doc(alias = "NLBITS")]
    #[inline]
    pub const fn set_nlbits(self, val: u8) -> Self {
        assert!(val < 0x10, "NLBITS value out of range (expected 0..=0xF)");
        Self((self.0 & !Self::NLBITS) | (Self::NLBITS & (val << 1)))
    }
    /// Get nlbits.
    #[inline]
    pub const fn nlbits(self) -> u8 {
        (self.0 & Self::NLBITS) >> 1
    }
    /// Enable vector mode (`NVBITS`).
    #[doc(alias = "NVBITS")]
    #[inline]
    pub const fn enable_vector_mode(self) -> Self {
        Self(self.0 | Self::NVBITS)
    }
    /// Disable vector mode.
    #[inline]
    pub const fn disable_vector_mode(self) -> Self {
        Self(self.0 & !Self::NVBITS)
    }
    /// Check if vector mode is enabled.
    #[inline]
    pub const fn is_vector_mode_enabled(self) -> bool {
        (self.0 & Self::NVBITS) != 0
    }
}

/// CLIC information register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Info(u32);

impl Info {
    const CLICCTLBITS: u32 = 0xF << 21;
    const VERSION: u32 = 0xFF << 13;
    const NUM_INTERRUPT: u32 = 0x1FFF;

    /// Get clicctlbits (`CLICCTLBITS`).
    #[doc(alias = "CLICCTLBITS")]
    #[inline]
    pub const fn clicctlbits(self) -> u8 {
        ((self.0 & Self::CLICCTLBITS) >> 21) as u8
    }
    /// Get version (`VERSION`).
    #[doc(alias = "VERSION")]
    #[inline]
    pub const fn version(self) -> u8 {
        ((self.0 & Self::VERSION) >> 13) as u8
    }
    /// Get number of interrupts (`NUM_INTERRUPT`).
    #[doc(alias = "NUM_INTERRUPT")]
    #[inline]
    pub const fn num_interrupt(self) -> u16 {
        (self.0 & Self::NUM_INTERRUPT) as u16
    }
}

/// Interrupt threshold register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MintThresh(u32);

impl MintThresh {
    const THRESHOLD: u32 = 0xFFFFFF;

    /// Set machine mode interrupt threshold (`THRESHOLD`).
    #[doc(alias = "THRESHOLD")]
    #[inline]
    pub const fn set_mint_thresh(self, threshold: u32) -> Self {
        assert!(
            threshold < 0x1000000,
            "THRESHOLD value out of range (expected 0..=0xFFFFFF)"
        );
        Self((self.0 & !Self::THRESHOLD) | (Self::THRESHOLD & threshold))
    }
    /// Get machine mode interrupt threshold.
    #[inline]
    pub const fn mint_thresh(self) -> u32 {
        self.0 & Self::THRESHOLD
    }
}

/// Interrupt pending register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Pending(u8);

impl Pending {
    const PENDING: u8 = 0x1;

    /// Check if the interrupt is pending (`PENDING`).
    #[doc(alias = "PENDING")]
    #[inline]
    pub const fn is_pending(self) -> bool {
        (self.0 & Self::PENDING) != 0
    }
    /// Clear pending interrupt.
    #[inline]
    pub const fn clear_pending(self) -> Self {
        Self(self.0 & !Self::PENDING)
    }
}

/// Interrupt enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Enable(u8);

impl Enable {
    const ENABLE: u8 = 0x1;

    /// Enable interrupt (`ENABLE`).
    #[doc(alias = "ENABLE")]
    #[inline]
    pub const fn enable(self) -> Self {
        Self(self.0 | Self::ENABLE)
    }
    /// Disable interrupt.
    #[inline]
    pub const fn disable(self) -> Self {
        Self(self.0 & !Self::ENABLE)
    }
    /// Check if the interrupt is enabled.
    #[inline]
    pub const fn is_enabled(self) -> bool {
        (self.0 & Self::ENABLE) != 0
    }
}

/// Interrupt attribute register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Attribute(u8);

impl Attribute {
    const MODE: u8 = 0x3 << 6;
    const TRIG: u8 = 0x3 << 1;
    const SHV: u8 = 0x1;

    /// Set privilege mode (`MODE`).
    #[doc(alias = "MODE")]
    #[inline]
    pub const fn set_mode(self, mode: u8) -> Self {
        assert!(mode < 0x4, "MODE value out of range (expected 0..=0x3)");
        Self((self.0 & !Self::MODE) | (Self::MODE & (mode << 6)))
    }
    /// Get privilege mode.
    #[inline]
    pub const fn mode(self) -> u8 {
        (self.0 & Self::MODE) >> 6
    }
    /// Set trigger mode (`TRIG`).
    #[doc(alias = "TRIG")]
    #[inline]
    pub const fn set_trig(self, trig: u8) -> Self {
        assert!(trig < 0x4, "TRIG value out of range (expected 0..=0x3)");
        Self((self.0 & !Self::TRIG) | (Self::TRIG & (trig << 1)))
    }
    /// Get trigger mode.
    #[inline]
    pub const fn trig(self) -> u8 {
        (self.0 & Self::TRIG) >> 1
    }
    /// Set hardware vector interrupt bit (`SHV`)..
    #[doc(alias = "SHV")]
    #[inline]
    pub const fn set_hardware_vector(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::SHV)
        } else {
            Self(self.0 & !Self::SHV)
        }
    }
    /// Get hardware vector interrupt bit.
    #[inline]
    pub const fn hardware_vector(self) -> bool {
        (self.0 & Self::SHV) != 0
    }
}
