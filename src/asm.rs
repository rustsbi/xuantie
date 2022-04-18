//! Assembly instructions
//!
//! This module allows you to use XuanTie instructions without using specialized assembler or compiler.
//!
//! Not all these instructions are supported on your XuanTie platform.
//! You may use `mcpuid` register to get your implementation model, or read the manual
//! before using any of following assembly instructions.
use core::arch::asm;

// T-Head extended instructions are mostly encoded under custom-0 opcode space (opcode 0x0B).

/// DCACHE.CALL, D-cache clean all dirty items instruction
///
/// Clears all L1 D-cache table items, write all dirty items to next level storage.
///
/// # Permissions
///
/// Must run on M or S mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on C906 core.
#[inline]
pub unsafe fn dcache_call() {
    asm!(".insn i 0x0B, 0, x0, x0, 0x001")
}

/// DCACHE.IALL, D-cache invalid all items instruction
///
/// Invalidates all L1 D-cache table items.
///
/// # Permissions
///
/// Can run on M or S mode.
///
/// # Exceptions
///
/// May raise illegal instruction exception.
///
/// - When `mxstatus.theadisaee = 0`, this instruction always raise illegal instruction exception.
/// - When `mxstatus.theadisaee = 1`, this instruction will raise illegal instruction when being run on U mode.
///
/// # Platform support
///
/// This instruction is supported on C906 core.
#[inline]
pub unsafe fn dcache_iall() {
    asm!(".insn i 0x0B, 0, x0, x0, 0x002")
}

/// DCACHE.CIALL, D-cache clean all dirty and invalid item instruction
///
/// Writes all L1 D-cache dirty items to next level storage, and invalidate all L1 D-cache table items.
///
/// # Permissions
///
/// Can run on M or S mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception, or load page fault exception.
///
/// - When `mxstatus.theadisaee = 0`, this instruction always raise illegal instruction exception.
/// - When `mxstatus.theadisaee = 1`, this instruction will raise illegal instruction when being run on U mode.
///
/// # Platform support
///
/// This instruction is supported on C906 core.
#[inline]
pub unsafe fn dcache_ciall() {
    asm!(".insn i 0x0B, 0, x0, x0, 0x003")
}

/// IPUSH, fast interrupt stack push instruction
///
/// Push interrupt switch registers into current stack.
/// It pushes `mcause`, `mepc`, `x1`, `x5` to `x7`, `x10` to `x17` and `x28` to `x31` into stack.
/// Another word, the pushed `xi` integer registers are `ra`, `t0` to `t6`, and `a0` to `a7` (not in order)
/// other than CSR registers `mcause` and `mepc`.
///
/// In pseudocode, it performs like:
///
/// ```no_run
/// *sp.offset(-1) = mcause;
/// *sp.offset(-2) = mepc;
/// *sp.offset(-3) = ra;
/// /* ... Mem[sp - 4] ..= Mem[sp - 72] ← mcause, mepc, {xi} */
/// *sp.offset(-18) = t6;
/// sp = sp.offset(-18);
/// ```
///
/// # Permissions
///
/// Must run on M mode.
///
/// # Exceptions
///
/// Raises store unaligned exception, store access exception, or illegal instruction exception.
#[inline]
pub unsafe fn ipush() {
    asm!(".insn i 0x0B, 0, x0, x0, 0x004")
}

/// IPOP, fast interrupt stack pop instruction
///
/// Pop interrupt switch registers from current stack, and return from interrupt environment.
/// It pops `mcause`, `mepc`, `x1`, `x5` to `x7`, `x10` to `x17` and `x28` to `x31` from stack.
/// Another word, the poped `xi` integer registers are `ra`, `t0` to `t6`, and `a0` to `a7` (not in order)
/// other than CSR registers `mcause` and `mepc`.
///
/// In pseudocode, it performs like:
///
/// ```no_run
/// mcause = *sp.offset(17);
/// mepc = *sp.offset(16);
/// ra = *sp.offset(15);
/// /* ... mcause, mepc, {xi} ← Mem[sp + 68] ..= Mem[sp] */
/// t6 = *sp.offset(0);
/// sp = sp.offset(18);
/// riscv::asm::mret();
/// ```
///
/// # Permissions
///
/// Must run on M mode.
///
/// # Exceptions
///
/// Raises store unaligned exception, store access exception, or illegal instruction exception.
#[inline]
pub unsafe fn ipop() {
    asm!(".insn i 0x0B, 0, x0, x0, 0x005")
}

/// ICACHE.IALL, I-cache invalid all items instruction
#[inline]
pub unsafe fn icache_iall() {
    asm!(".insn i 0x0B, 0, x0, x0, 0x010")
}

/// ICACHE.IALLS, I-cache broadcast all cores to invalid all items instruction
#[inline]
pub unsafe fn icache_ialls() {
    asm!(".insn i 0x0B, 0, x0, x0, 0x011")
}

/// L2CACHE.CALL, L2-cache clean all dirty items instruction
#[inline]
pub unsafe fn l2cache_call() {
    asm!(".insn i 0x0B, 0, x0, x0, 0x015")
}

/// L2CACHE.IALL, L2-cache invalid all items instruction
#[inline]
pub unsafe fn l2cache_iall() {
    asm!(".insn i 0x0B, 0, x0, x0, 0x016")
}

/// L2CACHE.CIALL, L2-cache clean all dirty and invalid item instruction
#[inline]
pub unsafe fn l2cache_ciall() {
    asm!(".insn i 0x0B, 0, x0, x0, 0x017")
}

/// DCACHE.CSW, D-cache clean dirty item on way and set instruction
///
/// Writes D-cache dirty table item corresponding to given way and set to next level storage.
///
/// # Permissions
///
/// Can run on M or S mode.
///
/// # Exceptions
///
/// May raise illegal instruction exception.
///
/// - When `mxstatus.theadisaee = 0`, this instruction always raise illegal instruction exception.
/// - When `mxstatus.theadisaee = 1`, this instruction will raise illegal instruction when being run on U mode.
///
/// # Platform support
///
/// This instruction is supported on C906 core.
///
/// The C906 core has a 4-way set-associative D-cache. Input variable `rs1[31:30]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configurated 32 Kibibytes, `w` equals 13;
/// when configurated 64 Kibibytes, `w` equals 14.
#[inline]
pub unsafe fn dcache_csw(way_and_set: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x021", in(reg) way_and_set)
}

/// DCACHE.ISW, D-cache invalid item for way and set instruction
///
/// Invalidate D-cache dirty table item corresponding to given way and set.
///
/// # Permissions
///
/// Can run on M or S mode.
///
/// # Exceptions
///
/// May raise illegal instruction exception.
///
/// - When `mxstatus.theadisaee = 0`, this instruction always raise illegal instruction exception.
/// - When `mxstatus.theadisaee = 1`, this instruction will raise illegal instruction when being run on U mode.
///
/// # Platform support
///
/// This instruction is supported on C906 core.
///
/// The C906 core has a 4-way set-associative D-cache. Input variable `rs1[31:30]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configurated 32 Kibibytes, `w` equals 13;
/// when configurated 64 Kibibytes, `w` equals 14.
#[inline]
pub unsafe fn dcache_isw(way_and_set: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x022", in(reg) way_and_set)
}

/// DCACHE.CISW, D-cache clean dirty and invalid for way and set instruction
///
/// Writes L1 D-cache dirty item corresponding to given way and set to next level storage,
/// and invalidate this table item.
///
/// # Permissions
///
/// Can run on M or S mode.
///
/// # Exceptions
///
/// May raise illegal instruction exception.
///
/// - When `mxstatus.theadisaee = 0`, this instruction always raise illegal instruction exception.
/// - When `mxstatus.theadisaee = 1`, this instruction will raise illegal instruction when being run on U mode.
///
/// # Platform support
///
/// This instruction is supported on C906 core.
///
/// The C906 core has a 4-way set-associative D-cache. Input variable `rs1[31:30]` represents number of way,
/// while `rs1[w:6]` represents number of set. When D-cache is configurated 32 Kibibytes, `w` equals 13;
/// when configurated 64 Kibibytes, `w` equals 14.
#[inline]
pub unsafe fn dcache_cisw(way_and_set: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x023", in(reg) way_and_set)
}

/// DCACHE.CVAL1, L1 D-cache clean dirty item for virtual address instruction
#[inline]
pub unsafe fn dcache_cval1(va: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x024", in(reg) va)
}

/// DCACHE.CVA, D-cache clean dirty item for virtual address instruction
///
/// Writes D-cache table item corresponding to virtual address `va` to next level storage.
/// This operation effects on L1 cache on all cores.
///
/// # Permissions
///
/// Can run on M, S or U mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception, or load page fault exception.
///
/// - When `mxstatus.theadisaee = 0`, this instruction always raise illegal instruction exception.
/// - When `mxstatus.theadisaee = 1`, and `mxstatus.ucme = 1`, this instruction can be run on U mode.
/// - When `mxstatus.theadisaee = 1`, and `mxstatus.ucme = 0`,
///   this instruction will raise illegal instruction when being run on U mode.
///
/// # Platform support
///
/// This instruction is supported on C906 core.
#[inline]
pub unsafe fn dcache_cva(va: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x025", in(reg) va)
}

/// DCACHE.IVA, D-cache invalid item for virtual address instruction
///
/// Invalidates D-cache table item corresponding to virtual address `va`.
///
/// # Permissions
///
/// Can run on M or S mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception, or load page fault exception.
///
/// - When `mxstatus.theadisaee = 0`, this instruction always raise illegal instruction exception.
/// - When `mxstatus.theadisaee = 1`, this instruction will raise illegal instruction when being run on U mode.
///
/// # Platform support
///
/// This instruction is supported on C906 core.
#[inline]
pub unsafe fn dcache_iva(va: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x026", in(reg) va)
}

/// DCACHE.CIVA, D-cache clean dirty and invalid for virtual address instruction
///
/// Write D-cache table item corresponding to virtual address `va` to next level storage,
/// and invalidate this table item.
///
/// # Permissions
///
/// Can run on M or S mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception, or load page fault exception.
///
/// - When `mxstatus.theadisaee = 0`, this instruction always raise illegal instruction exception.
/// - When `mxstatus.theadisaee = 1`, and `mxstatus.ucme = 1`, this instruction can be run on U mode.
/// - When `mxstatus.theadisaee = 1`, and `mxstatus.ucme = 0`,
///   this instruction will raise illegal instruction when being run on U mode.
///
/// # Platform support
///
/// This instruction is supported on C906 core.
#[inline]
pub unsafe fn dcache_civa(va: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x027", in(reg) va)
}

/// DCACHE.CPAL1, L1 D-cache clean dirty item for physical address instruction
#[inline]
pub unsafe fn dcache_cpal1(pa: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x028", in(reg) pa)
}

/// DCACHE.CPA, D-cache clean dirty item for physical address instruction
///
/// Writes D-cache table item corresponding to physical address `pa` to next level storage.
/// This operation effects on L1 cache on all cores.
///
/// # Permissions
///
/// Can run on M or S mode.
///
/// # Exceptions
///
/// May raise illegal instruction exception.
///
/// - When `mxstatus.theadisaee = 0`, this instruction always raise illegal instruction exception.
/// - When `mxstatus.theadisaee = 1`, this instruction will raise illegal instruction when being run on U mode.
///
/// # Platform support
///
/// This instruction is supported on C906 core.
#[inline]
pub unsafe fn dcache_cpa(pa: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x029", in(reg) pa)
}

#[inline]
/// DCACHE.IPA, D-cache invalid item for physical address instruction
///
/// Invalidates D-cache table item corresponding to physical address `pa`.
///
/// # Permissions
///
/// Can run on M, S or U mode.
///
/// # Exceptions
///
/// May raise illegal instruction exception.
///
/// - When `mxstatus.theadisaee = 0`, this instruction always raise illegal instruction exception.
/// - When `mxstatus.theadisaee = 1`, this instruction will raise illegal instruction when being run on U mode.
///
/// # Platform support
///
/// This instruction is supported on C906 core.
pub unsafe fn dcache_ipa(pa: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x02A", in(reg) pa)
}

/// DCACHE.CIPA, D-cache clean dirty and invalid for physical address instruction
///
/// Writes D-cache table item corresponding to physical address `pa` to next level storage,
/// and invalidate this table item.
///
/// # Permissions
///
/// Can run on M or S mode.
///
/// # Exceptions
///
/// May raise illegal instruction exception.
///
/// - When `mxstatus.theadisaee = 0`, this instruction always raise illegal instruction exception.
/// - When `mxstatus.theadisaee = 1`, this instruction will raise illegal instruction when being run on U mode.
///
/// # Platform support
///
/// This instruction is supported on C906 core.
#[inline]
pub unsafe fn dcache_cipa(pa: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x02B", in(reg) pa)
}

/// ICACHE.IVA, I-cache invalid item for virtual address instruction
#[inline]
pub unsafe fn icache_iva(va: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x030", in(reg) va)
}

/// ICACHE.IPA, I-cache invalid item for physical address instruction
#[inline]
pub unsafe fn icache_ipa(pa: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x038", in(reg) pa)
}
