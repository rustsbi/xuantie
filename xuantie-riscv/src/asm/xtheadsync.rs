use core::arch::asm;

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
/// This instruction is supported on Xuantie C910, C906, C907, C908, E907 and E906 cores.
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
/// This instruction is supported on Xuantie C910, C907 and C908 cores.
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
/// This instruction is supported on Xuantie C910, C906, C907, C908, E907 and E906 cores.
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
/// This instruction is supported on Xuantie C910, C907 and C908 cores.
#[inline]
pub unsafe fn sync_is() {
    // th.sync.is
    asm!(".insn i 0x0B, 0, x0, x0, 0x01B")
}

// TODO platform support

/// Invalidate TLB on all harts via broadcasting for all address spaces and virtual addresses.
///
/// # Permissions
///
/// This instruction can be executed in all privilege levels higher than U mode. Attempts to execute
/// this instruction in U mode raise an illegal instruction exception.
///
/// # Exceptions
///
/// This instruction does not trigger any exceptions.
#[inline]
pub unsafe fn sfence_vmas_all() {
    // th.sfence.vmas x0, x0
    asm!(
        ".insn r 0x0B, 0, x0, x0, x0, 0x02",
        options(nostack, preserves_flags)
    )
}

/// Invalidate TLB on all harts via broadcasting for given virtual address.
///
/// # Permissions
///
/// This instruction can be executed in all privilege levels higher than U mode. Attempts to execute
/// this instruction in U mode raise an illegal instruction exception.
///
/// # Exceptions
///
/// This instruction does not trigger any exceptions.
#[inline]
pub unsafe fn sfence_vmas_vaddr(vaddr: usize) {
    // th.sfence.vmas vaddr, x0
    asm!(".insn r 0x0B, 0, x0, {}, x0, 0x02", in(reg) vaddr, options(nostack, preserves_flags))
}

/// Invalidate TLB on all harts via broadcasting for given address space.
///
/// # Permissions
///
/// This instruction can be executed in all privilege levels higher than U mode. Attempts to execute
/// this instruction in U mode raise an illegal instruction exception.
///
/// # Exceptions
///
/// This instruction does not trigger any exceptions.
#[inline]
pub unsafe fn sfence_vmas_asid(asid: usize) {
    // th.sfence.vmas x0, asid
    asm!(".insn r 0x0B, 0, x0, x0, {}, 0x02", in(reg) asid, options(nostack, preserves_flags))
}

/// Invalidate TLB on all harts via broadcasting for given virtual address and address space.
///
/// # Permissions
///
/// This instruction can be executed in all privilege levels higher than U mode. Attempts to execute
/// this instruction in U mode raise an illegal instruction exception.
///
/// # Exceptions
///
/// This instruction does not trigger any exceptions.
#[inline]
pub unsafe fn sfence_vmas(vaddr: usize, asid: usize) {
    // th.sfence.vmas vaddr, asid
    asm!(".insn r 0x0B, 0, x0, {}, {}, 0x02", in(reg) vaddr, in(reg) asid, options(nostack, preserves_flags))
}
