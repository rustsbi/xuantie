//! XuanTie Core Local Interrupt (CLINT) peripheral.

use volatile_register::RW;

/// Machine-mode Software Interrupt Pending (MSIP) register.
#[repr(transparent)]
pub struct MSIP(RW<u32>);

/// Machine-mode Timer Compare (MTIMECMP) register group.
#[repr(C)]
pub struct MTIMECMP {
    /// Low 32-bit word of MTIMECMP register.
    mtimecmpl: RW<u32>,
    /// High 32-bit word of MTIMECMP register.
    mtimecmph: RW<u32>,
}

/// Supervisor-mode Software Interrupt Pending (SSIP) register.
#[repr(transparent)]
pub struct SSIP(RW<u32>);

/// Supervisor-mode Timer Compare (STIMECMP) register group.
#[repr(C)]
pub struct STIMECMP {
    /// Low 32-bit word of STIMECMP register.
    stimecmpl: RW<u32>,
    /// High 32-bit word of STIMECMP register.
    stimecmph: RW<u32>,
}

/// Register block for XuanTie Core Local Interrupt (CLINT) peripheral.
#[repr(C)]
pub struct THeadClint {
    msip: [MSIP; 4096],
    mtimecmp: [MTIMECMP; 4096],
    ssip: [SSIP; 1024],
    stimecmp: [STIMECMP; 1024],
}

impl THeadClint {
    /// Determine whether `hart_idx` has a valid machine mode software interrupt.
    #[inline]
    pub fn read_msip(&self, hart_idx: usize) -> bool {
        self.msip[hart_idx].0.read() != 0
    }

    /// Set the machine mode software interrupt of `hart_idx`.
    #[inline]
    pub fn set_msip(&self, hart_idx: usize) {
        unsafe { self.msip[hart_idx].0.write(1) }
    }

    /// Clear the machine mode software interrupt of `hart_idx`.
    #[inline]
    pub fn clear_msip(&self, hart_idx: usize) {
        unsafe { self.msip[hart_idx].0.write(0) }
    }

    /// Read the mtimecmp register of `hart_idx`.
    #[inline]
    pub fn read_mtimecmp(&self, hart_idx: usize) -> u64 {
        let mtimecmpl = self.mtimecmp[hart_idx].mtimecmpl.read();
        let mtimecmph = self.mtimecmp[hart_idx].mtimecmph.read();
        ((mtimecmph as u64) << 32) | mtimecmpl as u64
    }

    /// Write the mtimecmp register of `hart_idx`.
    #[inline]
    pub fn write_mtimecmp(&self, hart_idx: usize, val: u64) {
        let mtimecmpl: u32 = (val & 0xffffffff) as u32;
        let mtimecmph: u32 = (val >> 32) as u32;
        unsafe { self.mtimecmp[hart_idx].mtimecmpl.write(mtimecmpl) }
        unsafe { self.mtimecmp[hart_idx].mtimecmph.write(mtimecmph) }
    }

    /// Determine whether `hart_idx` has a valid supervisor mode software interrupt.
    #[inline]
    pub fn read_ssip(&self, hart_idx: usize) -> bool {
        self.ssip[hart_idx].0.read() != 0
    }

    /// Set the supervisor mode software interrupt of `hart_idx`.
    #[inline]
    pub fn set_ssip(&self, hart_idx: usize) {
        unsafe { self.ssip[hart_idx].0.write(1) }
    }

    /// Clear the supervisor mode software interrupt of `hart_idx`.
    #[inline]
    pub fn clear_ssip(&self, hart_idx: usize) {
        unsafe { self.ssip[hart_idx].0.write(0) }
    }

    /// Read the `stimecmp` register of `hart_idx`.
    #[inline]
    pub fn read_stimecmp(&self, hart_idx: usize) -> u64 {
        let stimecmpl = self.stimecmp[hart_idx].stimecmpl.read();
        let stimecmph = self.stimecmp[hart_idx].stimecmph.read();
        ((stimecmph as u64) << 32) | stimecmpl as u64
    }

    /// Write the `stimecmp` register of `hart_idx`.
    #[inline]
    pub fn write_stimecmp(&self, hart_idx: usize, val: u64) {
        let stimecmpl: u32 = (val & 0xffffffff) as u32;
        let stimecmph: u32 = (val >> 32) as u32;
        unsafe { self.stimecmp[hart_idx].stimecmpl.write(stimecmpl) }
        unsafe { self.stimecmp[hart_idx].stimecmph.write(stimecmph) }
    }
}
