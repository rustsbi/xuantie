use core::arch::asm;

// TODO: th.sfence.vmas

/// Synchronize instruction.
///
/// Ensures that all instructions before retire earlier than this instruction,
/// and all instructions after retire later than this instruction.
///
/// # Permissions
///
/// Can run on M, U mode, or S mode if applicable.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906, C907, E907 and E906 cores.
#[inline]
pub unsafe fn sync() {
    // th.sync
    asm!(".insn i 0x0B, 0, x0, x0, 0x018")
}

/// Synchronize and broadcast instruction.
///
/// Ensures that all instructions before retire earlier than this instruction,
/// and all instructions after retire later than this instruction.
/// This request will be broadcast to all other harts.
///
/// # Permissions
///
/// Can run on M, S or U mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910 and C907 cores.
#[inline]
pub unsafe fn sync_s() {
    // th.sync.s
    asm!(".insn i 0x0B, 0, x0, x0, 0x019")
}

/// Synchronize and clean instruction.
///
/// Ensures that all instructions before retire earlier than this instruction,
/// and all instructions after retire later than this instruction.
/// The pipeline is emptied when this instruction retires.
///
/// # Permissions
///
/// Can run on M, U mode, or S mode if applicable.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906, C907, E907 and E906 cores.
#[inline]
pub unsafe fn sync_i() {
    // th.sync.i
    asm!(".insn i 0x0B, 0, x0, x0, 0x01A")
}

/// Synchronize, clean and broadcast instruction.
///
/// Ensures that all instructions before retire earlier than this instruction,
/// and all instructions after retire later than this instruction.
/// The pipeline is emptied when this instruction retires.
/// This request will be broadcast to all other harts.
///
/// # Permissions
///
/// Can run on M, S or U mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910 and C907 cores.
#[inline]
pub unsafe fn sync_is() {
    // th.sync.is
    asm!(".insn i 0x0B, 0, x0, x0, 0x01B")
}
