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
/// Clears all L1 D-cache table elements, write all dirty items to next level storage.
///
/// # Permissions
///
/// Must run on M or S mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
#[inline]
pub unsafe fn dcache_call() {
    asm!(".insn i 0x0B, 0, x0, x0, 0x001")
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
///   this instruction will raise illegal instruction when being run U mode.
#[inline]
pub unsafe fn dcache_cva(va: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x025", in(reg) va)
}

/// DCACHE.CPA, D-cache clean dirty item for physical address instruction
#[inline]
pub unsafe fn dcache_cpa(pa: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x029", in(reg) pa)
}

/// DCACHE.CSW, D-cache clean dirty item for way or set instruction
#[inline]
pub unsafe fn dcache_csw(set_or_way: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x021", in(reg) set_or_way)
}

/// DCACHE.IALL, D-cache invalid all items instruction
#[inline]
pub unsafe fn dcache_iall() {
    asm!(".insn i 0x0B, 0, x0, x0, 0x002")
}

/// DCACHE.IVA, D-cache invalid item for virtual address instruction
#[inline]
pub unsafe fn dcache_iva(va: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x026", in(reg) va)
}

#[inline]
/// DCACHE.IPA, D-cache invalid item for physical address instruction
pub unsafe fn dcache_ipa(pa: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x02A", in(reg) pa)
}

/// DCACHE.ISW, D-cache invalid item for way or set instruction
#[inline]
pub unsafe fn dcache_isw(set_or_way: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x022", in(reg) set_or_way)
}

/// DCACHE.CIALL, D-cache clean all dirty and invalid item instruction
#[inline]
pub unsafe fn dcache_ciall() {
    asm!(".insn i 0x0B, 0, x0, x0, 0x003")
}

/// DCACHE.CIVA, D-cache clean dirty and invalid for virtual address instruction
#[inline]
pub unsafe fn dcache_civa(va: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x027", in(reg) va)
}

/// DCACHE.CIPA, D-cache clean dirty and invalid for physical address instruction
#[inline]
pub unsafe fn dcache_cipa(pa: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x02B", in(reg) pa)
}

/// DCACHE.CISW, D-cache clean dirty and invalid for set or way instruction
#[inline]
pub unsafe fn dcache_cisw(set_or_way: usize) {
    asm!(".insn i 0x0B, 0, x0, {}, 0x023", in(reg) set_or_way)
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
