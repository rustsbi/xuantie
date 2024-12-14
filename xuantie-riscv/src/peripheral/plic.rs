//! XuanTie Platform Local Interrupt Controller (PLIC).

use core::{
    num::NonZeroU32,
    ptr::{read_volatile, write_volatile},
};
use plic::{HartContext, InterruptSource};

const PLIC_CTRL_OFFSET: usize = 0x01F_FFFC;
const PLIC_CTRL_S_PER_ENABLED: u32 = 0x1;
const PLIC_CTRL_S_PER_DISABLED: u32 = 0x0;
const PLIC_CTRL_S_PER_BIT: u32 = 0x1;
const THEAD_PLIC_PRIORITY_BITS: u32 = 5;

/// Register block for XuanTie Platform Local Interrupt Controller (PLIC).
#[repr(C, align(4096))]
pub struct Plic {
    inner: plic::Plic,
}

impl Plic {
    /// Enable supervisor access to all the PLIC registers.
    #[inline]
    pub fn enable_supervisor_access(&self) {
        let plic_ctrl =
            unsafe { (&self.inner as *const _ as *const u8).add(PLIC_CTRL_OFFSET) as *mut u32 };
        unsafe { write_volatile(plic_ctrl, PLIC_CTRL_S_PER_ENABLED) };
    }

    /// Disable supervisor access to all the PLIC registers.
    #[inline]
    pub fn disable_supervisor_access(&self) {
        let plic_ctrl =
            unsafe { (&self.inner as *const _ as *const u8).add(PLIC_CTRL_OFFSET) as *mut u32 };
        unsafe { write_volatile(plic_ctrl, PLIC_CTRL_S_PER_DISABLED) };
    }

    /// Check if supervisor access to all the PLIC registers is enabled.
    #[inline]
    pub fn is_supervisor_access_enabled(&self) -> bool {
        let plic_ctrl =
            unsafe { (&self.inner as *const _ as *const u8).add(PLIC_CTRL_OFFSET) as *const u32 };
        let val = unsafe { read_volatile(plic_ctrl) };
        val & PLIC_CTRL_S_PER_BIT != 0
    }
}

impl Plic {
    /// Sets priority for interrupt `source` to `value`.
    #[inline]
    pub fn set_priority<S>(&self, source: S, value: u32)
    where
        S: InterruptSource,
    {
        self.inner.set_priority(source, value);
    }

    /// Gets priority for interrupt `source`.
    #[inline]
    pub fn get_priority<S>(&self, source: S) -> u32
    where
        S: InterruptSource,
    {
        self.inner.get_priority(source)
    }

    /// Probe maximum level of priority for interrupt `source`.
    #[inline]
    pub fn probe_priority_bits<S>(&self, _source: S) -> u32
    where
        S: InterruptSource,
    {
        THEAD_PLIC_PRIORITY_BITS
    }

    /// Check if interrupt `source` is pending.
    #[inline]
    pub fn is_pending<S>(&self, source: S) -> bool
    where
        S: InterruptSource,
    {
        self.inner.is_pending(source)
    }

    /// Enable interrupt `source` in `context`.
    #[inline]
    pub fn enable<S, C>(&self, source: S, context: C)
    where
        S: InterruptSource,
        C: HartContext,
    {
        self.inner.enable(source, context);
    }

    /// Disable interrupt `source` in `context`.
    #[inline]
    pub fn disable<S, C>(&self, source: S, context: C)
    where
        S: InterruptSource,
        C: HartContext,
    {
        self.inner.disable(source, context);
    }

    /// Check if interrupt `source` is enabled in `context`.
    #[inline]
    pub fn is_enabled<S, C>(&self, source: S, context: C) -> bool
    where
        S: InterruptSource,
        C: HartContext,
    {
        self.inner.is_enabled(source, context)
    }

    /// Get interrupt threshold in `context`.
    #[inline]
    pub fn get_threshold<C>(&self, context: C) -> u32
    where
        C: HartContext,
    {
        self.inner.get_threshold(context)
    }

    /// Set interrupt threshold for `context` to `value`.
    #[inline]
    pub fn set_threshold<C>(&self, context: C, value: u32)
    where
        C: HartContext,
    {
        self.inner.set_threshold(context, value);
    }

    /// Probe maximum supported threshold value the `context` supports.
    #[inline]
    pub fn probe_threshold_bits<C>(&self, _context: C) -> u32
    where
        C: HartContext,
    {
        THEAD_PLIC_PRIORITY_BITS
    }

    /// Claim an interrupt in `context`, returning its source.
    #[inline]
    pub fn claim<C>(&self, context: C) -> Option<NonZeroU32>
    where
        C: HartContext,
    {
        self.inner.claim(context)
    }

    /// Mark that interrupt identified by `source` is completed in `context`.
    #[inline]
    pub fn complete<C, S>(&self, context: C, source: S)
    where
        C: HartContext,
        S: InterruptSource,
    {
        self.inner.complete(context, source);
    }
}
