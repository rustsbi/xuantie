//! XuanTie Clint

use volatile_register::RW;

/// MSIP Register
#[repr(transparent)]
pub struct MSIP(RW<u32>);

/// MTIMECMP Register
#[repr(C)]
pub struct MTIMECMP {
    /// MTIMECMP Low Register
    mtimecmpl: RW<u32>,
    /// MTIMECMP High Register
    mtimecmph: RW<u32>,
}

/// SSIP Register
#[repr(transparent)]
pub struct SSIP(RW<u32>);

/// STIMECMP Register
#[repr(C)]
pub struct STIMECMP {
    /// STIMECMP Low Register
    stimecmpl: RW<u32>,
    /// STIMECMP High Register
    stimecmph: RW<u32>,
}

/// Register block for T-Head Clint
#[repr(C)]
pub struct THeadClint {
    msip: [MSIP; 4096],
    mtimecmp: [MTIMECMP; 4096],
    ssip: [SSIP; 1024],
    stimecmp: [STIMECMP; 1024],
}

impl THeadClint {
    /// Determine whether hart_idx has a valid machine mode software interrupt.
    #[inline]
    pub fn read_msip(&self, hart_idx: usize) -> bool {
        self.msip[hart_idx].0.read() != 0
    }

    /// Set the machine mode software interrupt of hart_idx.
    #[inline]
    pub fn set_msip(&self, hart_idx: usize) {
        unsafe { self.msip[hart_idx].0.write(1) }
    }

    /// Clear the machine mode software interrupt of hart_idx.
    #[inline]
    pub fn clear_msip(&self, hart_idx: usize) {
        unsafe { self.msip[hart_idx].0.write(0) }
    }

    /// Read the mtimecmp register of hart_idx.
    #[inline]
    pub fn read_mtimecmp(&self, hart_idx: usize) -> u64 {
        let mtimecmpl = self.mtimecmp[hart_idx].mtimecmpl.read();
        let mtimecmph = self.mtimecmp[hart_idx].mtimecmph.read();
        ((mtimecmph as u64) << 32) | mtimecmpl as u64
    }

    /// Write the mtimecmp register of hart_idx.
    #[inline]
    pub fn write_mtimecmp(&self, hart_idx: usize, val: u64) {
        let mtimecmpl: u32 = (val & 0xffffffff) as u32;
        let mtimecmph: u32 = (val >> 32) as u32;
        unsafe { self.mtimecmp[hart_idx].mtimecmpl.write(mtimecmpl) }
        unsafe { self.mtimecmp[hart_idx].mtimecmph.write(mtimecmph) }
    }

    /// Determine whether hart_idx has a valid supervisor mode software interrupt.
    #[inline]
    pub fn read_ssip(&self, hart_idx: usize) -> bool {
        self.ssip[hart_idx].0.read() != 0
    }

    /// Set the supervisor mode software interrupt of hart_idx.
    #[inline]
    pub fn set_ssip(&self, hart_idx: usize) {
        unsafe { self.ssip[hart_idx].0.write(1) }
    }

    /// Clear the supervisor mode software interrupt of hart_idx.
    #[inline]
    pub fn clear_ssip(&self, hart_idx: usize) {
        unsafe { self.ssip[hart_idx].0.write(0) }
    }

    /// Read the stimecmp register of hart_idx.
    #[inline]
    pub fn read_stimecmp(&self, hart_idx: usize) -> u64 {
        let stimecmpl = self.stimecmp[hart_idx].stimecmpl.read();
        let stimecmph = self.stimecmp[hart_idx].stimecmph.read();
        ((stimecmph as u64) << 32) | stimecmpl as u64
    }

    /// Write the stimecmp register of hart_idx.
    #[inline]
    pub fn write_stimecmp(&self, hart_idx: usize, val: u64) {
        let stimecmpl: u32 = (val & 0xffffffff) as u32;
        let stimecmph: u32 = (val >> 32) as u32;
        unsafe { self.stimecmp[hart_idx].stimecmpl.write(stimecmpl) }
        unsafe { self.stimecmp[hart_idx].stimecmph.write(stimecmph) }
    }
}

